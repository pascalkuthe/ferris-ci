pub use crate::llvm::archive::populate_archive;

mod archive;
mod build;

const BUILD_DIR: &str = "llvm_build";
const SRC_DIR: &str = "llvm_src";
