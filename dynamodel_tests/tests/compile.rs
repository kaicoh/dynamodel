#[test]
fn compile_err() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fails/**/*.rs");
}

#[test]
fn compile_success() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/passes/**/*.rs");
}
