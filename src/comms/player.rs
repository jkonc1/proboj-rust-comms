use super::runner;
use crate::block::Block;
use crate::internal_types::Args;
use crate::types::{PlayerInfo, Status};
use std::str::FromStr;

pub fn read_player(target: &PlayerInfo) -> (Status, Option<Block>) {
    runner::send_command("READ PLAYER", Args::from_str(target.name()), "");
    let (status, data) = runner::read_runner();
    let status = Status::from_str(&status).unwrap_or(Status::Error);
    let data = data.concat();
    match status {
        Status::Ok => match Block::from_str(data.as_str()) {
            Ok(block) => (status, Some(block)),
            Err(_) => (Status::Error, None),
        },
        _ => (status, None),
    }
}

pub fn to_player(target: &PlayerInfo, block: &Block) -> Status {
    runner::send_command(
        "TO PLAYER",
        Args::from_str(target.name()),
        &block.to_string(),
    )
}

pub fn send_and_read_player(target: &PlayerInfo, block: &Block) -> (Status, Option<Block>) {
    if to_player(target, block) != Status::Ok {
        (Status::Error, None)
    } else {
        read_player(target)
    }
}

pub fn log_player(target: &PlayerInfo, message: &str) -> Status {
    todo!(); // Zosikanovat gardenera nech prida tuto feature
    runner::send_command("LOG PLAYER", Args::from_str(target.name()), message)
}

pub fn pause_player(target: &PlayerInfo) -> Status {
    runner::send_command("PAUSE PLAYER", Args::from_str(target.name()), "")
}

pub fn continue_player(target: &PlayerInfo) -> Status {
    runner::send_command("RESUME PLAYER", Args::from_str(target.name()), "")
}

pub fn kill_player(target: &PlayerInfo) -> Status {
    runner::send_command("KILL PLAYER", Args::from_str(target.name()), "")
}
