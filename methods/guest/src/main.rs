#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

use c_kzg::{KzgCommitment, KzgSettings};
mod kzg_utils;
mod kzg_trust_setup;
use kzg_trust_setup::KZG_TRUST_SETUP;
use kzg_utils::parse_kzg_trusted_setup;
use sha2::{Digest, Sha256};

risc0_zkvm::guest::entry!(main);

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
    let (g1, g2) = parse_kzg_trusted_setup(&KZG_TRUST_SETUP).unwrap();
    // TODO: do something with the input

    // uncomment any of following line, you will see the error message
    // undefined symbol: malloc
    // let kzg_settings = KzgSettings::load_trusted_setup(&g1.0, &g2.0).unwrap();
    // let kzg_commit = KzgCommitment::blob_to_kzg_commitment(&input.into(), &kzg_settings).unwrap();
    // let versioned_hash = kzg_to_versioned_hash(&kzg_commit);

    // write public output to the journal
    // env::commit(&kzg_commit.as_slice());
}
