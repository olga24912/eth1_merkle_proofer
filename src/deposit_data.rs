use serde_derive::{Deserialize, Serialize};
use bls::PublicKeyBytes;
use bls::SignatureBytes;

use ethereum_types::H256;
pub type Hash256 = H256;

#[derive(Deserialize, Serialize)]
pub struct DepositData {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: Hash256,
    #[serde(with = "eth2_serde_utils::quoted_u64")]
    pub amount: u64,
    pub signature: SignatureBytes,
}
