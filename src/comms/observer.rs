use super::runner;
use crate::block::Block;
use crate::internal_types::Args;

pub fn to_observer(block: Block) {
    runner::send_command("TO OBSERVER", Args::new_empty(), &block.to_string())
}
