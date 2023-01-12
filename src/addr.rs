use bitcoin::hashes::hex::FromHex;

pub fn create_address() {
    let seed_hex = "038109007313a5807b2eccc082c8c3fbb988a973cacf1a7df9ce725c31b14776";

    let network = bitcoin::Network::Testnet;

    let seed = Vec::from_hex(seed_hex).unwrap();
}
