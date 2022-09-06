fn main() {
    // while ferris-ci itself is currently build with musl, the various c libs are not (yet)
    let target = std::env::var("TARGET").unwrap().replace("musl", "gnu");
    println!("cargo:rustc-env=CFG_COMPILER_HOST_TRIPLE={}", target);
}
