pub fn main() {
    println!("cargo:rustc-link-search=native={}", "/home/chenyh/prog/binutils-install/lib");
    println!("cargo:rustc-link-lib=static=iberty");
    println!("cargo:rustc-link-lib=dylib=z");
}
