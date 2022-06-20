use serde_derive::{Deserialize, Serialize};
use ethereum_types::H256;

pub type Hash256 = H256;

#[derive(Deserialize, Serialize)]
pub struct Checkpoint {
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub epoch: u64,
    pub root: Hash256,
}
