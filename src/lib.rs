mod beacon_block_header;
mod eth1_data;
mod graffiti;
mod signed_beacon_block_header;
mod proposer_slashing;
mod checkpoint;
mod attestation_data;
mod indexed_attestation;
mod attester_slashing;
mod attestation;
mod deposit_data;
mod deposit;
mod voluntary_exit;
mod signed_voluntary_exit;
mod sync_aggregate;
mod execution_playload;
mod beacon_block_body;

use beacon_block_header::BeaconBlockHeader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
