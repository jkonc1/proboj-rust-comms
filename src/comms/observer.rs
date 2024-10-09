use super::runner;
use crate::block::Block;
use crate::internal_types::Args;
use crate::types::Status;

/**
Send `Block` to observer.
*/
pub fn to_observer(block: &Block) -> Status {
    runner::send_command("TO OBSERVER", Args::new(), &block.to_string())
}
