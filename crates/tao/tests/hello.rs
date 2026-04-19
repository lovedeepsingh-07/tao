#[tokio::test]
#[rstest::rstest]
#[case("hello", "hello")]
async fn hello(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(input, expected);
}
