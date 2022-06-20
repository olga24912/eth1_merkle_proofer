use bls::Signature;
use crate::eth1_data::Eth1Data;
use crate::graffiti::Graffiti;
use crate::proposer_slashing::ProposerSlashing;
use ssz_types::VariableList;
use ssz_types::typenum::{U16, U2, U128};
use crate::attester_slashing::AttesterSlashing;
use crate::attestation::Attestation;
use crate::deposit::Deposit;
use crate::signed_voluntary_exit::SignedVoluntaryExit;
use crate::sync_aggregate::SyncAggregate;
use crate::execution_playload::FullPayload;

pub struct BeaconBlockBody {
    pub randao_reveal: bls::Signature,
    pub eth1_data: Eth1Data,
    pub graffiti: Graffiti,
    pub proposer_slashings: VariableList<ProposerSlashing, U16>,
    pub attester_slashings: VariableList<AttesterSlashing, U2>,
    pub attestations: VariableList<Attestation, U128>,
    pub deposit: VariableList<Deposit, U16>,
    pub voluntary_exits: VariableList<SignedVoluntaryExit, U16>,
    pub sync_aggregate: SyncAggregate,
    pub execution_playload: FullPayload,
}
