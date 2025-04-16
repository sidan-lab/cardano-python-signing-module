#[allow(unused_must_use)]
fn main() {
    cxx_build::bridge("src/lib.rs")
        .warnings(false)
        .extra_warnings(false)
        .std("c++11")
        .compile("signer-demo");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
