cargo build
cp -f target/debug/libsigner.a src/libsigner.a
cp -f $(realpath target/cxxbridge/signer/src/lib.rs.h) src/lib.rs.h
cp -f $(realpath target/cxxbridge/rust/cxx.h) src/cxx.h
rm -rf target
swig -c++ -python src/signer.i
cd src
python setup.py build_ext --inplace
cd ..