use super::runner;
use crate::block::Block;
use crate::internal_types::Args;
use crate::types::Status;

/**
Send `BlockEquivalent` to observer.
*/
pub fn to_observer<T>(data: T) -> Status
where
    T: Into<Block>,
{
    let block: Block = data.into();
    let (status, _) = runner::send_command("TO OBSERVER", Args::new(), &block.to_string());
    status
}
