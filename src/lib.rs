use whisky_wallet::{
    derivation_indices::DerivationIndices, Account, MnemonicWallet, RootKeyWallet, Wallet,
    WalletType,
};

#[cxx::bridge]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type Signer;
        fn new_mnemonic_signer(mnemonic_phrase: &str, derivation_path: &str) -> Box<Signer>;
        fn new_bech32_signer(root_private_key: &str, derivation_path: &str) -> Box<Signer>;
        fn new_cli_signer(ed25519_key: &str) -> Box<Signer>;
        fn sign_transaction(&mut self, tx_hex: &str) -> String;
        fn get_public_key(&self) -> String;
    }
}

fn new_mnemonic_signer(mnemonic_phrase: &str, derivation_path: &str) -> Box<Signer> {
    let wallet = Wallet::new(WalletType::MnemonicWallet(MnemonicWallet {
        mnemonic_phrase: mnemonic_phrase.to_string(),
        derivation_indices: DerivationIndices::from_str(derivation_path),
    }));
    let account = wallet.get_account().unwrap();
    Box::new(Signer { account })
}

fn new_bech32_signer(root_private_key: &str, derivation_path: &str) -> Box<Signer> {
    let wallet = Wallet::new(WalletType::RootKeyWallet(RootKeyWallet {
        root_key: root_private_key.to_string(),
        derivation_indices: DerivationIndices::from_str(derivation_path),
    }));
    let account = wallet.get_account().unwrap();
    Box::new(Signer { account })
}

fn new_cli_signer(ed25519_key: &str) -> Box<Signer> {
    let wallet = Wallet::new_cli(ed25519_key);
    let account = wallet.get_account().unwrap();
    Box::new(Signer { account })
}

struct Signer {
    account: Account,
}

impl Signer {
    pub fn sign_transaction(&self, tx_hex: &str) -> String {
        self.account.sign_transaction(tx_hex).unwrap_or_else(|_| {
            panic!("Failed to sign transaction with the provided account");
        })
    }

    pub fn get_public_key(&self) -> String {
        self.account.public_key.to_hex()
    }
}
