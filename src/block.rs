use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::str::Lines;
use scan_fmt::scan_fmt;

pub trait BlockEquivalent{
    type Err;
    fn to_block(&self) -> Block;
    fn from_block(_ : &Block) -> Result<Block, Self::Err>;
}

impl BlockEquivalent for Block{
    type Err = ();
    fn to_block(&self) -> Block {
        self.clone()
    }
    fn from_block(block: &Block) -> Result<Block, Self::Err> {
        Ok(block.clone())
    }
}

#[derive(Clone)]
pub struct Block {
    pub name: String,
    pub content: BlockType,
}

type Line = String;

#[derive(Clone)]
pub enum BlockType {
    TextBlock(Vec<Line>),
    ArrayBlock(Vec<Block>),
}

#[derive(Debug)]
pub enum BlockParsingError {
    InvalidHeader(String),
    EOF,
}

impl Display for BlockParsingError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            BlockParsingError::InvalidHeader(s) => write!(f, "Invalid header: {s}"),
            BlockParsingError::EOF => write!(f, "Unexpected EOF"),
        }
    }
}

impl Error for BlockParsingError {}

impl Block {
    pub fn new_text_block(name: String, content: Vec<Line>) -> Block {
        Block {
            name,
            content: BlockType::TextBlock(content),
        }
    }

    pub fn new_array_block(name: String, content: Vec<Block>) -> Block {
        Block {
            name,
            content: BlockType::ArrayBlock(content),
        }
    }
    fn parse_from_iter(mut line_iter: &mut Lines) -> Result<Block, BlockParsingError> {
        let header_line = line_iter.next().ok_or(BlockParsingError::EOF)?;

        // the format of header line is `NAME:str TYPE:str LENGHT:u32` split and parse it

        let scan = scan_fmt!(header_line, "{} {} {d}", String, String, u32);

        if scan.is_err() {
            return Err(BlockParsingError::InvalidHeader(header_line.to_string()));
        }
        let (name, block_type, length) = scan.unwrap();

        match block_type.as_str() {
            "TEXT" => Self::parse_text_block(&mut line_iter, name, length),
            "ARRAY" => Self::parse_array_block(&mut line_iter, name, length),
            _ => return Err(BlockParsingError::InvalidHeader(header_line.to_string())),
        }
    }

    fn parse_text_block(
        line_iter: &mut Lines,
        name: String,
        length: u32,
    ) -> Result<Block, BlockParsingError> {
        let mut content = vec![];

        for _ in 0..length {
            let line = line_iter.next();

            if let Some(line) = line {
                content.push(line.to_string());
            } else {
                return Err(BlockParsingError::EOF);
            }
        }
        Ok(Block::new_text_block(name, content))
    }

    fn parse_array_block(
        line_iter: &mut Lines,
        name: String,
        length: u32,
    ) -> Result<Block, BlockParsingError> {
        let mut content = vec![];

        for _ in 0..length {
            let block = Block::parse_from_iter(line_iter)?;
            content.push(block);
        }
        Ok(Block::new_array_block(name, content))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self.content {
            BlockType::TextBlock(lines) => {
                writeln!(f, "{} TEXT {}", self.name, lines.len())?;
                for line in lines {
                    writeln!(f, "{}", line)?;
                }
                Ok(())
            }
            BlockType::ArrayBlock(blocks) => {
                writeln!(f, "{} ARRAY {}", self.name, blocks.len())?;
                for block in blocks {
                    write!(f, "{}", block)?;
                }
                Ok(())
            }
        }
    }
}

impl FromStr for Block {
    type Err = BlockParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();
        Block::parse_from_iter(&mut line_iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_block() {
        let input = "HEADER TEXT 2\nHello\nWorld\n";

        let block = input.parse::<Block>().unwrap();

        if let BlockType::TextBlock(ref lines) = block.content {
            assert_eq!(lines.len(), 2);
            assert_eq!(lines[0], "Hello");
            assert_eq!(lines[1], "World");
        } else {
            panic!("Invalid block type");
        }
    }

    #[test]
    fn test_parsing_nested_block() {
        let input = "HEADER ARRAY 2\nHEADER TEXT 2\nHello\nWorld\nHEADER TEXT 1\nRust\n";

        let block = input.parse::<Block>().unwrap();

        if let BlockType::ArrayBlock(ref blocks) = block.content {
            assert_eq!(blocks.len(), 2);

            if let BlockType::TextBlock(ref lines) = blocks[0].content {
                assert_eq!(lines.len(), 2);
                assert_eq!(lines[0], "Hello");
                assert_eq!(lines[1], "World");
            } else {
                panic!("Invalid block type");
            }

            if let BlockType::TextBlock(ref lines) = blocks[1].content {
                assert_eq!(lines.len(), 1);
                assert_eq!(lines[0], "Rust");
            } else {
                panic!("Invalid block type");
            }
        } else {
            panic!("Invalid block type");
        }
    }

    #[test]
    fn test_parsing_invalid_header() {
        let input = "HEADER INVALID 2\nHello\nWorld\n";

        let block = input.parse::<Block>();

        assert!(block.is_err());
    }

    #[test]
    fn test_parsing_eof() {
        let input = "HEADER TEXT 2\nHello\n";

        let block = input.parse::<Block>();

        assert!(block.is_err());
    }

    #[test]
    fn test_display_block() {
        let input = "HEADER TEXT 2\nHello\nWorld\n";

        let block = input.parse::<Block>().unwrap();

        let output = format!("{}", block);

        assert_eq!(input, output);
    }

    #[test]
    fn test_display_nested_block() {
        let input = "HEADER ARRAY 2\nHEADER TEXT 2\nHello\nWorld\nHEADER TEXT 1\nRust\n";

        let block = input.parse::<Block>().unwrap();

        let output = format!("{}", block);

        assert_eq!(input, output);
    }

    #[test]
    fn test_display_empty_text_block() {
        let input = "HEADER TEXT 0\n";

        let block = input.parse::<Block>().unwrap();

        let output = format!("{}", block);

        assert_eq!(input, output);
    }

    #[test]
    fn test_display_empty_array_block() {
        let input = "HEADER ARRAY 0\n";

        let block = input.parse::<Block>().unwrap();

        let output = format!("{}", block);

        assert_eq!(input, output);
    }
}
