use crate::checkpoint::Checkpoint;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

use ethereum_types::H256;
pub type Hash256 = H256;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct AttestationData {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub slot: u64,

    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub index: u64,

    //LMD GHOST vote
    pub beacon_block_root: Hash256,

    //FFG Vote
    pub source: Checkpoint,
    pub target: Checkpoint,
}
