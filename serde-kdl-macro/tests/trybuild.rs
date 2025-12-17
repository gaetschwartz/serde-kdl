//! Compile-fail tests using trybuild
//!
//! These tests verify that invalid KDL syntax produces helpful compile errors.

#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/specs/compile_fail/**/*.rs");
}
