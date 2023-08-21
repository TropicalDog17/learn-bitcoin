use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PrivateKey, PublicKey};
fn main() {
    // Generate random key pair.
    let s = Secp256k1::new();
    let keypair = s.generate_keypair(&mut rand::thread_rng());
    let private_key = PrivateKey::new_uncompressed(keypair.0, Network::Testnet);
    let public_key = PublicKey::new(keypair.1);
    // let private_key = PrivateKey::new()
    // Generate pay-to-pubkey-hash address.
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    println!("Private key: {}", private_key);
    println!("Public key: {}", public_key);
    println!("Bitcoin address: {address:?}");
}
