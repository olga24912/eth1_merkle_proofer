use ethereum_types::H256;

pub type Hash256 = H256;

pub struct Checkpoint {
    pub epoch: u64,
    pub root: Hash256,
}
