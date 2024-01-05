#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/success/*.rs");
    t.compile_fail("tests/failed/*.rs");
}
