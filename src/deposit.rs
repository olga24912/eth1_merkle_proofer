use ssz_types::FixedVector;
use ssz_types::typenum::U33;
use crate::deposit_data::DepositData;

use ethereum_types::H256;
pub type Hash256 = H256;

pub struct Deposit {
    pub proof: FixedVector<Hash256, U33>,
    pub data: DepositData,
}
