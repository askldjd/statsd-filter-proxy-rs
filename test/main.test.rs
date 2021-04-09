use filter;

#[test]
fn sanity_test() {
    assert_eq!(true, filter::should_filter());
}
