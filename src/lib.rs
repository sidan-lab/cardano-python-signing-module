use bip39::{Language, Mnemonic};
use cardano_serialization_lib::{Bip32PrivateKey, FixedTransaction, PrivateKey, PublicKey};

#[cxx::bridge]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type Signer;
        fn new_mnemonic_signer(
            mnemonic_phrase: &str,
            account_index: u32,
            key_index: u32,
        ) -> Box<Signer>;
        fn new_bech32_signer(
            root_private_key: &str,
            account_index: u32,
            key_index: u32,
        ) -> Box<Signer>;
        fn sign_transaction(&mut self, tx_hex: &str) -> String;
        fn get_public_key(&self) -> String;
    }
}

fn new_mnemonic_signer(mnemonic_phrase: &str, account_index: u32, key_index: u32) -> Box<Signer> {
    let mnemonic =
        Mnemonic::from_phrase(mnemonic_phrase, Language::English).expect("Invalid mnemonic phrase");
    let entropy = mnemonic.entropy();
    let root_key = Bip32PrivateKey::from_bip39_entropy(entropy, &[]);

    let hardened_key_start = 2147483648;
    let account_key = root_key
        .derive(hardened_key_start + 1852)
        .derive(hardened_key_start + 1815)
        .derive(hardened_key_start + account_index);

    let private_key = account_key.derive(0).derive(key_index).to_raw_key();
    let public_key = private_key.to_public();
    Box::new(Signer {
        private_key,
        public_key,
    })
}

fn new_bech32_signer(root_private_key: &str, account_index: u32, key_index: u32) -> Box<Signer> {
    let root_key = Bip32PrivateKey::from_bech32(root_private_key).unwrap();

    let hardened_key_start = 2147483648;
    let account_key = root_key
        .derive(hardened_key_start + 1852)
        .derive(hardened_key_start + 1815)
        .derive(hardened_key_start + account_index);

    let private_key = account_key.derive(0).derive(key_index).to_raw_key();
    let public_key = private_key.to_public();
    Box::new(Signer {
        private_key,
        public_key,
    })
}

struct Signer {
    private_key: PrivateKey,
    public_key: PublicKey,
}

impl Signer {
    pub fn sign_transaction(&self, tx_hex: &str) -> String {
        let mut tx = FixedTransaction::from_hex(tx_hex).expect("Invalid transaction bytes");
        tx.sign_and_add_vkey_signature(&self.private_key).unwrap();
        tx.to_hex()
    }

    pub fn get_public_key(&self) -> String {
        self.public_key.clone().to_hex()
    }
}
