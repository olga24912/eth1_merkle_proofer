use bls::PublicKeyBytes;
use bls::SignatureBytes;

use ethereum_types::H256;
pub type Hash256 = H256;

pub struct DepositData {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub signature: SignatureBytes,
}
