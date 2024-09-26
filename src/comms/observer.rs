use super::runner;
use crate::block::Block;
use crate::internal_types::Args;

/**
Send `Block` to observer.
*/
pub fn to_observer(block: &Block) {
    runner::send_command("TO OBSERVER", Args::new(), &block.to_string())
}
