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
use crate::execution_playload::ExecutionPayload;
use std::env;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use tree_hash_derive::TreeHash;
use ethereum_types::H256;
use merkle_proof::MerkleTree;

#[derive(Deserialize, Serialize, TreeHash)]
pub struct BeaconBlockBody {
    pub randao_reveal: bls::Signature,
    pub eth1_data: Eth1Data,
    pub graffiti: Graffiti,
    pub proposer_slashings: VariableList<ProposerSlashing, U16>,
    pub attester_slashings: VariableList<AttesterSlashing, U2>,
    pub attestations: VariableList<Attestation, U128>,
    pub deposits: VariableList<Deposit, U16>,
    pub voluntary_exits: VariableList<SignedVoluntaryExit, U16>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
}

impl BeaconBlockBody {
    pub fn get_leaves(&self) -> [H256; 10] {
       use tree_hash::TreeHash;

       let mut llist : [H256; 10] = [
           self.randao_reveal.tree_hash_root(), 
           self.eth1_data.tree_hash_root(),
           self.graffiti.tree_hash_root(),
           self.proposer_slashings.tree_hash_root(),
           self.attester_slashings.tree_hash_root(),
           self.attestations.tree_hash_root(),
           self.deposits.tree_hash_root(),
           self.voluntary_exits.tree_hash_root(),
           self.sync_aggregate.tree_hash_root(),
           self.execution_payload.tree_hash_root()];

       llist
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn body_from_json() {
        let mut path_exmp_json = std::env::current_exe().unwrap();
        path_exmp_json.pop();
        path_exmp_json.push("../../..");
        path_exmp_json.push("source");
        path_exmp_json.push("body_exmp1.json");
        
        //println!("{:?}", path_exmp_json);

        let json_str = std::fs::read_to_string(path_exmp_json).expect("Unable to read file");

        let bbody : crate::beacon_block_body::BeaconBlockBody = serde_json::from_str(&json_str).unwrap();

        assert_eq!(format!("{:?}", bbody.eth1_data.deposit_root), "0x47909297c0124f98ddbfe84edf497ff503795627c275cfe43e23f309df579937");

        //println!("{:?}", json_str);


        //let body_json = json::parse(&json_str);
        //println!("{:?}", body_json);
    }

    #[test]
    fn get_tree_hash_root() {
        let mut path_exmp_json = std::env::current_exe().unwrap();
        path_exmp_json.pop();
        path_exmp_json.push("../../..");
        path_exmp_json.push("source");
        path_exmp_json.push("body_exmp2.json");

        let json_str = std::fs::read_to_string(path_exmp_json).expect("Unable to read file");

        let bbody : crate::beacon_block_body::BeaconBlockBody = serde_json::from_str(&json_str).unwrap();

        use tree_hash::TreeHash;
        println!("{:?}", bbody.tree_hash_root());

        assert_eq!(format!("{:?}", bbody.tree_hash_root()), "0x1906eb5417e2ff500a7c48c0704138b340f7d04c9ce7df8d213ad35232a4ff60");
    }

    #[test]
    fn build_merkle_tree() {
        let mut path_exmp_json = std::env::current_exe().unwrap();
        path_exmp_json.pop();
        path_exmp_json.push("../../..");
        path_exmp_json.push("source");
        path_exmp_json.push("body_exmp2.json");

        let json_str = std::fs::read_to_string(path_exmp_json).expect("Unable to read file");

        let bbody : crate::beacon_block_body::BeaconBlockBody = serde_json::from_str(&json_str).unwrap();

        use tree_hash::TreeHash;
        println!("{:?}", bbody.tree_hash_root());

        assert_eq!(format!("{:?}", bbody.tree_hash_root()), "0x1906eb5417e2ff500a7c48c0704138b340f7d04c9ce7df8d213ad35232a4ff60");

        let mut leaves = bbody.get_leaves();

        let mtree = merkle_proof::MerkleTree::create(&leaves, 4);

        assert_eq!(mtree.hash(), bbody.tree_hash_root());

        let mut proofg = mtree.generate_proof(1, 4);

        println!("{:?}", proofg);

        assert_eq!(bbody.eth1_data.tree_hash_root(), proofg.0);
        
        use ethereum_types::H256;
        let aproof : [H256; 4] = [proofg.1[0], proofg.1[1], proofg.1[2], proofg.1[3]];
        assert!(merkle_proof::verify_merkle_proof(proofg.0, &aproof, 4, 1, bbody.tree_hash_root()));

    }
}

