use bitcoin::{
    self,
    secp256k1::{rand, PublicKey, Secp256k1, SecretKey}, PrivateKey, Network, Address,
};

pub fn key_to_address() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::new(&mut rand::thread_rng());

    let private_key = PrivateKey::new(secret_key, Network::Testnet);
    
    
    let public_key = Address::p2pkh(&private_key.public_key(&secp), Network::Bitcoin);

    println!("Public Key: {}", public_key.to_string());

    // public_key.
}
