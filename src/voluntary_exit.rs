use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct VoluntaryExit {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub epoch: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub validator_index: u64,
}
