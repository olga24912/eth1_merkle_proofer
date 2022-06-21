use ssz_types::typenum::{U256, U32, U1073741824, U1048576};
use ethereum_types::{H256, H160};
use ssz_types::{FixedVector, VariableList};
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;

pub type Hash256 = H256;

#[derive(Deserialize, Serialize)]
pub struct ExecutionBlockHash(Hash256);

impl tree_hash::TreeHash for ExecutionBlockHash {
    fn tree_hash_type() -> tree_hash::TreeHashType {
        Hash256::tree_hash_type()
    }

    fn tree_hash_packed_encoding(&self) -> Vec<u8> {
        self.0.tree_hash_packed_encoding()
    }

    fn tree_hash_packing_factor() -> usize {
        Hash256::tree_hash_packing_factor()
    }

    fn tree_hash_root(&self) -> tree_hash::Hash256 {
        self.0.tree_hash_root()
    }
}

pub type Address = H160;

pub type Transaction = VariableList<u8, U1073741824>;
pub type Transactions = VariableList<Transaction, U1048576>;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct ExecutionPayload {
    pub parent_hash: ExecutionBlockHash,
    pub fee_recipient: Address,
    pub state_root: Hash256,
    pub receipts_root: Hash256,
    #[serde(with = "ssz_types::serde_utils::hex_fixed_vec")]
    pub logs_bloom: FixedVector<u8, U256>,
    pub prev_randao: Hash256,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub block_number: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub gas_limit: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub gas_used: u64,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub timestamp: u64,
    #[serde(with = "ssz_types::serde_utils::hex_var_list")]
    pub extra_data: VariableList<u8, U32>,
    #[serde(with = "eth2_serde_utils::quoted_u256")]
    pub base_fee_per_gas: ethereum_types::U256,
    pub block_hash: ExecutionBlockHash,
    #[serde(with = "ssz_types::serde_utils::list_of_hex_var_list")]
    pub transactions: Transactions,
}

#[derive(Deserialize, Serialize, TreeHash)]
pub struct FullPayload {
    pub execution_payload: ExecutionPayload,
}
