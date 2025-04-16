cargo build
cp -f target/debug/libsigner.a src/libsigner.a
cp -f $(realpath target/cxxbridge/signer/src/lib.rs.h) src/lib.rs.h
swig -c++ -python src/signer.i
cd src
python setup.py build_ext --inplace
cd ..