#include "lib.rs.h"

#include <string>

// class MnemonicSigner
// {
// public:
//     MnemonicSigner(std::string mnemonic, std::uint32_t account_index, std::uint32_t key_index);
//     ~MnemonicSigner() = default;

//     std::string sign_tx(std::string tx_hex);

// private:
//     std::string mnemonic_secret;
//     std::uint32_t account_index;
//     std::uint32_t key_index;
// };

// class Bech32Signer
// {
// public:
//     Bech32Signer(std::string bech32, std::uint32_t account_index, std::uint32_t key_index);
//     ~Bech32Signer() = default;

//     std::string sign_tx(std::string tx_hex);

// private:
//     std::string bech32_secret;
//     std::uint32_t account_index;
//     std::uint32_t key_index;
// };

std::string sign_mnemonic(std::string mnemonic, int account_index, int key_index, std::string tx_hex);

std::string sign_bech32(std::string bech32, int account_index, int key_index, std::string tx_hex);