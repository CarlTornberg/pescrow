use std::{env::current_dir, path::Path};

use litesvm::LiteSVM;

pub fn create_svm(program_name: &str) -> LiteSVM {

    let program_path = Path::
        join(&current_dir().unwrap(), "target/sbpf-solana-solana/release")
        .join(program_name);
    let mut svm = LiteSVM::new();
    if let Err(e) = svm.add_program_from_file(crate::ID, program_path) {
        panic!("Could not add {} program: {}", program_name, e);
    }
    svm
}
