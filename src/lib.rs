/*!
# proboj-rust-comms

A crate for handling communication with [ksp-proboj-runner](https://github.com/trojsten/ksp-proboj).
*/

pub use crate::comms::{init, observer, player};
pub use crate::types::Status;

mod internal_types;
pub mod types;
pub mod block;

pub mod comms {
    pub mod init;
    pub mod observer;
    pub mod player;
    mod runner;
}

