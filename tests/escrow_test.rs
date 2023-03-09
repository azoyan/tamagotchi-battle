use factory_escrow_io::FactoryAction;
use gtest::{Program, System};

#[test]
fn init_escrow_factory() {
    let system = System::new();
    let escrow_code_id =
        system.submit_code("../escrow/target/wasm32-unknown-unknown/release/escrow.opt.wasm");
    let escrow_factory = Program::current(&system);
    let res = escrow_factory.send(100, escrow_code_id);
    assert!(!res.main_failed());
    assert!(res.log().is_empty());
}
