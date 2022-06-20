use ssz_types::typenum::{U256, U32, U1073741824, U1048576};
use ethereum_types::{H256, H160};
use ssz_types::{FixedVector, VariableList};


pub type Hash256 = H256;
pub struct ExecutionBlockHash(Hash256);
pub type Address = H160;

pub type Transaction = VariableList<u8, U1073741824>;
pub type Transactions = VariableList<Transaction, U1048576>;

pub struct ExecutionPayload {
    pub parent_hash: ExecutionBlockHash,
    pub fee_recipient: Address,
    pub state_root: Hash256,
    pub receipts_root: Hash256,
    pub logs_bloom: FixedVector<u8, U256>,
    pub prev_randao: Hash256,
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: VariableList<u8, U32>,
    pub base_fee_per_gas: ethereum_types::U256,
    pub block_hash: ExecutionBlockHash,
    pub transactions: Transactions,
}

pub struct FullPayload {
    pub execution_payload: ExecutionPayload,
}
