use rstest::rstest;

use parsec_api_types::BackendAddr;

#[rstest]
#[case("https://foo.bar", false)]
#[case("https://foo.bar/redirection", false)]
#[case("https://foo.bar/redirect", true)]
#[case("https://foo.bar/not_valid", false)]
fn test_faulty_addr_redirection(#[case] raw_url: &str, #[case] is_valid: bool) {
    let res = dbg!(BackendAddr::from_http_redirection(raw_url));

    assert_eq!(res.is_ok(), is_valid)
}
