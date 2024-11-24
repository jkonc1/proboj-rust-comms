/*!
# proboj-rust-comms

A crate for handling communication with [ksp-proboj-runner](https://github.com/trojsten/ksp-proboj).
*/

pub use crate::comms::{end, init, observer, player};
pub use crate::types::{PlayerInfo, Status};
pub use block::{Block, BlockType};

pub mod block;
mod internal_types;
pub mod types;

pub mod comms {
    pub mod end;
    pub mod init;
    pub mod observer;
    pub mod player;
    mod runner;
}
