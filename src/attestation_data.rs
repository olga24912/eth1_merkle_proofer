use crate::checkpoint::Checkpoint;

use ethereum_types::H256;
pub type Hash256 = H256;

pub struct AttestationData {
    pub slot: u64,
    pub index: u64,

    //LMD GHOST vote
    pub beacon_block_root: Hash256,

    //FFG Vote
    pub source: Checkpoint,
    pub target: Checkpoint,
}
