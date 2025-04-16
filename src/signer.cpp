#include "lib.rs.h"

#include <string>

// class MnemonicSigner
// {
// public:
//     MnemonicSigner(std::string mnemonic, std::uint32_t account_index, std::uint32_t key_index)
//     {
//         this->mnemonic_secret = mnemonic;
//         this->account_index = account_index;
//         this->key_index = key_index;
//     };
//     ~MnemonicSigner() = default;

//     std::string sign_tx(std::string tx_hex)
//     {
//         auto signer = new_mnemonic_signer(mnemonic_secret, account_index, key_index);
//         std::string signed_tx = signer->sign_transaction(tx_hex).c_str();
//         return signed_tx;
//     };

// private:
//     std::string mnemonic_secret;
//     std::uint32_t account_index;
//     std::uint32_t key_index;
// };

// class Bech32Signer
// {
// public:
//     Bech32Signer(std::string bech32, std::uint32_t account_index, std::uint32_t key_index)
//     {
//         this->bech32_secret = bech32;
//         this->account_index = account_index;
//         this->key_index = key_index;
//     };
//     ~Bech32Signer() = default;

//     std::string sign_tx(std::string tx_hex)
//     {
//         auto signer = new_bech32_signer(bech32_secret, account_index, key_index);
//         std::string signed_tx = signer->sign_transaction(tx_hex).c_str();
//         return signed_tx;
//     };

// private:
//     std::string bech32_secret;
//     std::uint32_t account_index;
//     std::uint32_t key_index;
// };

std::string sign_mnemonic(std::string mnemonic, int account_index, int key_index, std::string tx_hex)
{
    auto signer = new_mnemonic_signer(mnemonic, account_index, key_index);
    std::string signed_tx = signer->sign_transaction(tx_hex).c_str();
    return signed_tx;
}

std::string sign_bech32(std::string bech32, int account_index, int key_index, std::string tx_hex)
{
    auto signer = new_bech32_signer(bech32, account_index, key_index);
    std::string signed_tx = signer->sign_transaction(tx_hex).c_str();
    return signed_tx;
}