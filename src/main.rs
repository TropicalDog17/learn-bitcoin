use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PrivateKey, PublicKey};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
fn calculate_p2pkh_address(public_key: &[u8], network_prefix: u8) -> String {
    // Step 1: Calculate SHA256 hash of the public key
    // let mut sha256_hasher = Sha256::new();
    // sha256_hasher.input(public_key);
    // let sha256_result = sha256_hasher.result();

    // // Step 2: Calculate RIPEMD160 hash of the SHA256 result
    // let mut ripemd160_hasher = Ripemd160::new();
    // ripemd160_hasher.input(&sha256_result);
    // let ripemd160_result = ripemd160_hasher.result();

    // // Step 3: Create the extended hash with the network prefix byte
    // let mut extended_hash = vec![network_prefix];
    // extended_hash.extend(ripemd160_result);

    // // Step 4: Calculate the double SHA256 checksum
    // let mut checksum_hasher = Sha256::new();
    // checksum_hasher.input(&extended_hash);
    // let checksum_result = checksum_hasher.result();
    // let checksum_result = Sha256::digest(&checksum_result);

    // // Step 5: Append the first 4 bytes of the checksum to the extended hash
    // extended_hash.extend(&checksum_result[0..4]);

    // // Step 6: Encode the extended hash plus checksum using Base58 encoding
    // let bitcoin_address = bs58::encode(extended_hash).into_string();
    let mut hasher = Sha256::new();
    hasher.update(public_key);
    let result = hasher.finalize();
    let mut hasher = Ripemd160::new();
    hasher.update(result);
    let result = hasher.finalize();
    let mut extended_hash = vec![network_prefix];
    extended_hash.extend(result);

    let mut checksum_hasher = Sha256::new();
    checksum_hasher.update(&extended_hash);
    let checksum_result = checksum_hasher.finalize();
    let mut checksum_hasher = Sha256::new();
    checksum_hasher.update(&checksum_result);
    let checksum_result = checksum_hasher.finalize();
    extended_hash.extend(&checksum_result[0..4]);
    let s = bs58::encode(extended_hash).into_string();
    s
    // bitcoin_address
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
    println!("{s}");
    println!("Generated: {}", s);
    println!("Private key: {}", private_key);
    println!("Public key: {}", public_key);
    println!("Bitcoin address: {address:?}");
}
