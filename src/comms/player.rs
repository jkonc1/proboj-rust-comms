use super::runner;
use crate::block::Block;
use crate::internal_types::{Args, PlayerInfo};
use crate::types::Status;
use std::str::FromStr;

/**
If `Status` from runner is not `OK`, the function returns an empty `Block`.

If `Block` fails to parse from user input, the function sets status to `Status::Error` and returns an empty `Block`.
*/
pub fn read_player(target: PlayerInfo) -> (Status, Block) {
    runner::send_command("READ PLAYER", Args::from_str(target), "");
    let (status, data) = runner::read_runner();
    let status = Status::from_str(&status).unwrap_or(Status::Error);
    let data = data.concat();
    match status {
        Status::Ok => match Block::from_str(data.as_str()) {
            Ok(block) => (status, block),
            Err(_) => (Status::Error, Block::new()),
        },
        _ => (status, Block::new()),
    }
}

pub fn to_player(target: PlayerInfo, block: &Block) {
    runner::send_command("TO PLAYER", Args::from_str(target), &block.to_string())
}

pub fn send_and_read_player(target: PlayerInfo, block: &Block) -> (Status, Block) {
    to_player(target, block);
    read_player(target)
}

pub fn log_player(target: PlayerInfo, message: &str) {
    todo!(); // Zosikanovat gardenera nech prida tuto feature
    runner::send_command("LOG PLAYER", Args::from_str(target), message)
}

pub fn pause_player(target: PlayerInfo) {
    runner::send_command("PAUSE PLAYER", Args::from_str(target), "")
}

pub fn continue_player(target: PlayerInfo) {
    runner::send_command("RESUME PLAYER", Args::from_str(target), "")
}

pub fn kill_player(target: PlayerInfo) {
    runner::send_command("KILL PLAYER", Args::from_str(target), "")
}
