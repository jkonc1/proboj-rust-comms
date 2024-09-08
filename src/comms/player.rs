use super::runner;
use crate::block::Block;
use crate::internal_types::{Args, PlayerInfo};
use crate::types::Status;

pub fn read_player(target: PlayerInfo) -> (Status, Block) {
    runner::send_command("READ PLAYER", Args::new_name(target), "");
    let (status, data) = runner::read_runner();
    let status = Status::from_string(&status);
    match status {
        Status::Ok => (status, Block::new(data)),
        _ => (status, Block::new(vec![])),
    }
}

pub fn to_player(target: PlayerInfo, block: Block) {
    runner::send_command("TO PLAYER", Args::new_name(target), &block.to_string())
}

pub fn send_and_read_player(target: PlayerInfo, block: Block) -> (Status, Block) {
    to_player(target, block);
    read_player(target)
}

pub fn log_player(target: PlayerInfo, message: &str) {
    todo!(); // Zosikanovat gardenera nech prida tuto feature
    runner::send_command("LOG PLAYER", Args::new_name(target), message)
}

pub fn pause_player(target: PlayerInfo) {
    runner::send_command("PAUSE PLAYER", Args::new_name(target), "")
}

pub fn continue_player(target: PlayerInfo) {
    runner::send_command("RESUME PLAYER", Args::new_name(target), "")
}

pub fn kill_player(target: PlayerInfo) {
    runner::send_command("KILL PLAYER", Args::new_name(target), "")
}
