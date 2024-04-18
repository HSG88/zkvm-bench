#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

use sha3::{Digest, Keccak256};



fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u8 = env::read();

    // TODO: do something with the input
    let mut hasher = Keccak256::new();
    hasher.update([input]);
    let result = hasher.finalize();

    // write public output to the journal
    env::commit(&result[0]);
}
