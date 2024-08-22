/*
Comms api:
Player
- read_player(PlayerInfo) -> (Status, Block)
- to_player(PlayerInfo, Block)
- send_and_read(PlayerInfo, Block) -> (Status, Block)
- log_player(PlayerInfo, &str)
- pause_player(PlayerInfo)
- continue_player(PlayerInfo)
- kill_player(PlayerInfo)

Observer
- to_observer(Block)
- send_scores(map<PlayerInfo, i32>)

Init
- init() -> (Vec<PlayerInfo>, Args)

Internal
- send_command(PlayerInfo, command : &str, args : Args, payload : Block)
- read_runner() -> (Status, Block)
*/

pub mod types; // TODO: Odtialto by mal byt public asi len Status

pub mod comms {
    pub mod init;
    pub mod observer;
    pub mod player;
    mod runner;
}

pub mod block {
    pub struct Block {
        data: Vec<String>,
    }

    impl Block {
        // TODO: Placeholder implementation
        pub fn new(data: Vec<String>) -> Self {
            Self { data }
        }
        pub fn new_empty() -> Self {
            Self { data: vec![] }
        }
    }

    impl std::fmt::Display for Block {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut res = String::new();
            for (i, s) in self.data.iter().enumerate() {
                if i != 0 {
                    res += "\n";
                }
                res += s;
            }
            write!(f, "{res}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        let args = types::Args::new(vec!["a", "b", "c"]);
        assert_eq!(args.to_string(), "a b c");
    }
}
