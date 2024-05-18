#[test]
fn compile_err() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/**/*.rs");
}
