use rust::our_assert_eq_wrapper;

#[test]
fn test_nested() {
    // NOTE: This line intentionally triggers a "can't compare `{integer}` with `{float}`"
    // error to check if RustBuilder recovers from it.
    our_assert_eq_wrapper!(4, 2.5);
}
