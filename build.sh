# cargo build
# cp -f target/debug/libsigner.a src/libsigner.a
# cp -f $(realpath target/cxxbridge/signer/src/lib.rs.h) src/lib.rs.h
# swig -c++ -python src/signer.i
# cd src
# python setup.py build_ext --inplace
# cd ..

#!/bin/bash
# Cross-platform build script

set -e

# Detect OS
case "$(uname -s)" in
    Linux*)     OS=linux;;
    Darwin*)    OS=macos;;
    MINGW*)     OS=windows;;
    MSYS*)      OS=windows;;
    CYGWIN*)    OS=windows;;
    *)          OS=unknown;;
esac

echo "Building for $OS..."

# Build the Rust library
cargo build

# Copy the built library to the correct location
if [ "$OS" = "windows" ]; then
    cp -f target/debug/signer.lib src/libsigner.a
    # On Windows, the path might be different
    cp -f "$(find target/cxxbridge -name "lib.rs.h")" src/lib.rs.h
else
    cp -f target/debug/libsigner.a src/libsigner.a
    cp -f $(realpath target/cxxbridge/signer/src/lib.rs.h) src/lib.rs.h
fi

# Generate SWIG bindings
swig -c++ -python src/signer.i

# Build the Python module
cd src
python setup.py build_ext --inplace
cd ..

echo "Build completed successfully for $OS"