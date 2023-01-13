use bitcoin::{self, secp256k1::{SecretKey, rand, Secp256k1, PublicKey}};
// use secp256k1::{SecretKey, };

pub fn key_to_address() {
    let secp = Secp256k1::new();
    let private_key = SecretKey::new(&mut rand::thread_rng());
   let public_key = PublicKey::from_secret_key(&secp, &private_key);

   println!("Public Key: {public_key}");
}