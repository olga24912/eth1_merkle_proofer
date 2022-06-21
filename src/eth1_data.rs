use ethereum_types::H256;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

pub type Hash256 = H256;

#[derive(Serialize, Deserialize, TreeHash)]
pub struct Eth1Data {
    pub deposit_root: Hash256,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub deposit_count: u64,
    pub block_hash: Hash256,
}
