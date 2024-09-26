/*!
# proboj-rust-comms

A crate for handling communication with [ksp-proboj-runner]().
*/

pub use crate::comms::{init, observer, player};
pub use crate::types::Status;

mod internal_types;
pub mod types;

pub mod comms {
    pub mod init;
    pub mod observer;
    pub mod player;
    mod runner;
}

pub mod block {
    use std::str::FromStr;

    pub struct Block {
        data: Vec<String>,
    }

    impl Block {
        // TODO: Placeholder implementation
        pub fn new() -> Self {
            Self { data: vec![] }
        }
    }

    pub struct ParseError;

    impl FromStr for Block {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let data: Vec<_> = s.split("/n").map(|s| s.to_string()).collect();
            Ok(Block { data })
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
