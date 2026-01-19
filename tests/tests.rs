mod common;
use common::*;
use pinocchio::Address;

const NAME: &str = "pinocchio_template.so";
const ID: Address = pinocchio_template::ID;

#[test]
fn setup_svm() {
    let _ = create_svm(NAME);
}

