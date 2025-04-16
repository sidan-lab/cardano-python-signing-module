%module CardanoSigner

%include "std_string.i"

%{
#define SWIG_FILE_WITH_INIT
#include "lib.rs.h"
#include "signer.h"
#include <string>
%}

std::string sign_mnemonic(std::string mnemonic, int account_index, int key_index, std::string tx_hex);

std::string sign_bech32(std::string bech32, int account_index, int key_index, std::string tx_hex);