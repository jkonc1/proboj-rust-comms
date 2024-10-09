use super::runner;
use crate::block::BlockEquivalent;
use crate::internal_types::Args;
use crate::types::Status;

/**
Send `BlockEquivalent` to observer.
*/
pub fn to_observer(data: &impl BlockEquivalent) -> Status {
    let block = data.to_block();
    runner::send_command("TO OBSERVER", Args::new(), &block.to_string())
}
