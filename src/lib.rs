use whisky::{Account, Wallet};

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
    let mut wallet = Wallet::new_mnemonic(mnemonic_phrase);
    let account = wallet
        .payment_account(account_index, key_index)
        .get_account()
        .unwrap();
    Box::new(Signer { account })
}

fn new_bech32_signer(root_private_key: &str, account_index: u32, key_index: u32) -> Box<Signer> {
    let mut wallet = Wallet::new_root_key(root_private_key);
    let account = wallet
        .payment_account(account_index, key_index)
        .get_account()
        .unwrap();
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
