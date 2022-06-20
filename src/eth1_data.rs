use ethereum_types::H256;

pub type Hash256 = H256;

pub struct Eth1Data {
    pub deposit_root: Hash256,
    pub deposit_count: u64,
    pub block_hash: Hash256,
}
