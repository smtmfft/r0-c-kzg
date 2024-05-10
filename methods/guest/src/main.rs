#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

use c_kzg_taiko::{KzgCommitment, KzgSettings};
mod kzg_utils;
mod kzg_trust_setup;
mod std_utils;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

const KZG_TRUST_SETUP_DATA: &[u8] = include_bytes!("../../../kzg_settings_raw.bin");

pub const VERSIONED_HASH_VERSION_KZG: u8 = 0x01;

pub fn kzg_to_versioned_hash(commitment: &KzgCommitment) -> [u8; 32]{
    let mut res = Sha256::digest(commitment.as_slice());
    res[0] = VERSIONED_HASH_VERSION_KZG;
    res.into()
}

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let mut input = [0; 4096*32];
    env::read_slice(&mut input);
    // sample input for testing
    env::log(format!("input: {:?}", hex::encode(Sha256::digest(&input.as_slice()))).as_str());

    // uncomment any of following line, you will see the error message
    let mut data = KZG_TRUST_SETUP_DATA.to_owned().clone();
    let kzg_settings = KzgSettings::from_u8_slice(&mut data);

    let kzg_commit = KzgCommitment::blob_to_kzg_commitment(&input.into(), &kzg_settings).unwrap();
    let versioned_hash = kzg_to_versioned_hash(&kzg_commit);
    env::log(format!("in VM versioned_hash: {:?}", hex::encode(versioned_hash)).as_str());

    // write public output to the journal
    env::commit(&versioned_hash);
}
