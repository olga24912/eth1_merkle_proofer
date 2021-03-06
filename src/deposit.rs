use ssz_types::FixedVector;
use ssz_types::typenum::U33;
use crate::deposit_data::DepositData;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

use ethereum_types::H256;
pub type Hash256 = H256;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct Deposit {
    pub proof: FixedVector<Hash256, U33>,
    pub data: DepositData,
}
