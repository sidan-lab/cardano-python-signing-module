use whisky::{Account, MnemonicWallet, RootKeyWallet, Wallet};

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
    let mut derivation_path_vec: Vec<&str> = derivation_path.split('/').collect();
    // remove first element if it is "m"
    if derivation_path_vec[0] == "m" {
        derivation_path_vec.remove(0);
    }
    let derivation_path_vec_u32: Vec<u32> = derivation_path_vec
        .iter()
        .filter_map(|&s| {
            if s.ends_with("'") {
                let mut chars = s.chars();
                chars.next_back(); // Remove the last character (')
                                   // Parse the string to u32 and add 0x80000000 for hardening
                Some(chars.as_str().parse::<u32>().unwrap() + 0x80000000)
            } else {
                s.parse::<u32>().ok()
            }
        })
        .collect();
    let wallet = Wallet::new(whisky::WalletType::MnemonicWallet(MnemonicWallet {
        mnemonic_phrase: mnemonic_phrase.to_string(),
        derivation_indices: whisky::derivation_indices::DerivationIndices(derivation_path_vec_u32),
    }));
    let account = wallet.get_account().unwrap();
    Box::new(Signer { account })
}

fn new_bech32_signer(root_private_key: &str, derivation_path: &str) -> Box<Signer> {
    let derivation_path_vec: Vec<&str> = derivation_path.split('/').collect();
    let derivation_path_vec_u32: Vec<u32> = derivation_path_vec
        .iter()
        .filter_map(|&s| {
            if s.ends_with("'") {
                let mut chars = s.chars();
                chars.next_back(); // Remove the last character (')
                                   // Parse the string to u32 and add 0x80000000 for hardening
                Some(chars.as_str().parse::<u32>().unwrap() + 0x80000000)
            } else {
                s.parse::<u32>().ok()
            }
        })
        .collect();
    let wallet = Wallet::new(whisky::WalletType::RootKeyWallet(RootKeyWallet {
        root_key: root_private_key.to_string(),
        derivation_indices: whisky::derivation_indices::DerivationIndices(derivation_path_vec_u32),
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
