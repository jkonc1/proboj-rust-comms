use serde::{de::DeserializeOwned, Serialize};
use super::runner;
use crate::internal_types::Args;
use crate::types::{PlayerInfo, Status};

pub fn read_player<'a, T>(target: &PlayerInfo) -> (Status, Option<T>) 
    where T : DeserializeOwned
{
    let (status, data) = runner::send_command("READ PLAYER", Args::from_str(target.name()), "");
    let data = data.join("\n");
    match status {
        Status::Ok => match serde_json::from_str::<T>(&data) {
            Ok(block) => (status, Some(block)),
            Err(_) => (Status::Error, None),
        },
        _ => (status, None),
    }
}

pub fn to_player<T>(target: &PlayerInfo, data: T) -> Status
where
    T: Serialize,
{
    let json = serde_json::to_string(&data).expect("Failed to serialize data to JSON");
    let (status, _) = runner::send_command(
        "TO PLAYER",
        Args::from_str(target.name()),
        &json,
    );
    status
}

pub fn send_and_read_player<T, R>(target: &PlayerInfo, data: T) -> (Status, Option<R>)
where
    T: Serialize, 
    R: DeserializeOwned,
{
    if to_player(target, data) != Status::Ok {
        (Status::Error, None)
    } else {
        read_player(target)
    }
}

pub fn log_player(target: &PlayerInfo, message: &str) -> Status {
    todo!(); // Zosikanovat gardenera nech prida tuto feature
    let (status, _) = runner::send_command("LOG PLAYER", Args::from_str(target.name()), message);
    status
}

pub fn pause_player(target: &PlayerInfo) -> Status {
    let (status, _) = runner::send_command("PAUSE PLAYER", Args::from_str(target.name()), "");
    status
}

pub fn continue_player(target: &PlayerInfo) -> Status {
    let (status, _) = runner::send_command("RESUME PLAYER", Args::from_str(target.name()), "");
    status
}

pub fn kill_player(target: &PlayerInfo) -> Status {
    let (status, _) = runner::send_command("KILL PLAYER", Args::from_str(target.name()), "");
    status
}
