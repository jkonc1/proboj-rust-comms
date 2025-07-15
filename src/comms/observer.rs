use serde::Serialize;
use super::runner;
use crate::internal_types::Args;
use crate::types::Status;

/**
Send `BlockEquivalent` to observer.
*/
pub fn to_observer<T>(data: T) -> Status
where
    T: Serialize,
{
    let json = serde_json::to_string(&data).expect("Failed to serialize data to JSON");
    let (status, _) = runner::send_command("TO OBSERVER", Args::new(), &(json + "\n"));
    status
}
