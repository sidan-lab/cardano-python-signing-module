%module CardanoSigner

%include "std_string.i"

%{
#define SWIG_FILE_WITH_INIT
#include "lib.rs.h"
#include "signer.h"
#include <string>
%}

std::string sign_mnemonic(std::string mnemonic, std::string derivation_path, std::string tx_hex);

std::string sign_bech32(std::string bech32, std::string derivation_path, std::string tx_hex);

std::string sign_cli(std::string ed25519_key, std::string tx_hex);