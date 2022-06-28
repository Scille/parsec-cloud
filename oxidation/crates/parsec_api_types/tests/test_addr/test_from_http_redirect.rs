use rstest::rstest;

use parsec_api_types::BackendAddr;

#[rstest]
#[case("https://foo.bar")]
#[case("https://foo.bar/redirection")]
#[case("https://foo.bar/not_valid")]
fn test_faulty_addr_redirection(#[case] raw_url: &str) {
    let res = dbg!(BackendAddr::from_http_redirection(raw_url));

    assert!(res.is_err());
}
