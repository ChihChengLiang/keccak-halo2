use hex_literal::hex;
use sha3::{Digest, Keccak256};

fn main() {
    println!("Hello, world!");

    let mut hasher = Keccak256::new();

    // write input message
    hasher.update(b"abc");

    // read hash digest
    let result = hasher.finalize();

    assert_eq!(
        result[..],
        hex!("4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45")[..]
    );
}
