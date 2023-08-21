use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
fn calculate_p2pkh_address(public_key: &[u8], network_prefix: u8) -> String {
    // reference: Mastering Bitcoin, p.102
    // Step 1: Calculate SHA256 hash of the public key

    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(public_key);
    let sha256_hash = sha256_hasher.finalize();
    // Step 2: Calculate RIPEMD160 hash of the SHA256 result

    let mut ripemd160_hasher = Ripemd160::new();
    ripemd160_hasher.update(sha256_hash);
    let pub_key_hash = ripemd160_hasher.finalize();

    // Step 3: Base58Check Encode

    // Extend the public key hash with prefix value of network_prefix(e.g: Bitcoin: 0x00);
    let mut extended_hash = vec![network_prefix];
    extended_hash.extend(pub_key_hash);

    // Calculate the double SHA256 checksum of the extended hash
    let mut checksum_hasher = Sha256::new();
    checksum_hasher.update(&extended_hash);
    let checksum_result = checksum_hasher.finalize();
    let mut checksum_hasher = Sha256::new();
    checksum_hasher.update(&checksum_result);
    let checksum_result = checksum_hasher.finalize();

    // Append the first 4 bytes of the checksum to the extended hash
    extended_hash.extend(&checksum_result[0..4]);

    //Encode the extended hash plus checksum using Base58 encoding
    let s = bs58::encode(extended_hash).into_string();

    s
}

fn main() {
    // Generate random key pair.
    let s = Secp256k1::new();
    let keypair = s.generate_keypair(&mut rand::thread_rng());
    let private_key = PrivateKey::new_uncompressed(keypair.0, Network::Testnet);
    let public_key = PublicKey::new(keypair.1);
    // let private_key = PrivateKey::new()
    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    // Try to generate bitcoin address using algorithm derive from "Mastering Bitcoin"
    // Specifically, we are going to generate an address A from a public key K with the following:
    // A = RIPEMD160(SHA256(K))

    let s = calculate_p2pkh_address(&public_key.to_bytes(), 0x00);
    println!("Generated: {}", s);
    println!("Private key: {}", private_key);
    println!("Public key: {}", public_key);
    println!("Bitcoin address: {address:?}");
}
