// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pretty_assertions::assert_eq;
use rstest::rstest;
use rstest_reuse::*;
use serde_test::{assert_tokens, Token};
use std::str::FromStr;

use libparsec_crypto::SigningKey;
use libparsec_types::*;

const ORG: &str = "MyOrg";
const RVK: &str = "P25GRG3XPSZKBEKXYQFBOLERWQNEDY3AO43MVNZCLPXPKN63JRYQssss";
const TOKEN: &str = "a0000000000000000000000000000001";
const DOMAIN: &str = "parsec.cloud.com";
const ENCRYPTED_PATH: &str = "HRSW4Y3SPFYHIZLEL5YGC6LMN5QWIPQs";
const WORKSPACE_ID: &str = "2d4ded1274064608833b7f57f01156e2";
const INVITATION_TYPE: &str = "claim_user";

/*
 * Helpers to parametrize the tests on different addr types
 */

trait Testbed {
    fn url(&self) -> String;

    fn assert_addr_ok(&self, url: &str);
    fn assert_addr_ok_with_expected(&self, url: &str, expected_url: &str);
    fn assert_addr_err(&self, url: &str, err_msg: &str);

    fn get_organization_id(&self, _url: &str) -> OrganizationID {
        unimplemented!()
    }
    fn assert_redirection_addr_ok(&self, redirection_url: &str, expected_url: &str);
    fn to_redirection_url(&self, _url: &str) -> String;
}

macro_rules! impl_testbed_common {
    ($addr_type:ty) => {
        fn assert_addr_ok(&self, url: &str) {
            self.assert_addr_ok_with_expected(url, url)
        }
        fn assert_addr_ok_with_expected(&self, url: &str, expected_url: &str) {
            let addr: $addr_type = url.parse().unwrap();
            let expected_addr: $addr_type = expected_url.parse().unwrap();

            // Compare the strings
            assert_eq!(addr.to_url().as_str(), expected_url);

            // Also compare the objects
            assert_eq!(addr, expected_addr);

            // Test serde
            // `Token::Str` requires `&'static str` but we have a regular `&str` here...
            // the solution is to explicitly leak the memory *shocked*
            let static_expected_url = Box::leak(expected_url.to_string().into_boxed_str());
            assert_tokens(&addr, &[Token::Str(static_expected_url)]);
        }
        fn assert_addr_err(&self, url: &str, err_msg: &str) {
            let ret = url.parse::<$addr_type>();
            assert_eq!(ret, Err(err_msg));
        }
        fn assert_redirection_addr_ok(&self, redirection_url: &str, expected_url: &str) {
            // From redirection scheme
            let expected_addr: $addr_type = expected_url.parse().unwrap();
            let addr = <$addr_type>::from_http_redirection(redirection_url).unwrap();
            assert_eq!(addr, expected_addr);

            // From any
            let addr2 = <$addr_type>::from_any(redirection_url).unwrap();
            assert_eq!(addr2, expected_addr);
        }
        fn to_redirection_url(&self, url: &str) -> String {
            let addr: $addr_type = url.parse().unwrap();
            addr.to_http_redirection_url().to_string()
        }
    };
}

macro_rules! impl_testbed_with_org {
    ($addr_type:ty) => {
        fn get_organization_id(&self, url: &str) -> OrganizationID {
            let addr: $addr_type = url.parse().unwrap();
            addr.organization_id().to_owned()
        }
    };
}

struct BackendAddrTestbed {}
impl Testbed for BackendAddrTestbed {
    impl_testbed_common!(BackendAddr);
    fn url(&self) -> String {
        format!("parsec://{}", DOMAIN)
    }
}

struct BackendOrganizationAddrTestbed {}
impl Testbed for BackendOrganizationAddrTestbed {
    impl_testbed_common!(BackendOrganizationAddr);
    impl_testbed_with_org!(BackendOrganizationAddr);
    fn url(&self) -> String {
        format!("parsec://{}/{}?rvk={}", DOMAIN, ORG, RVK)
    }
}

struct BackendOrganizationBootstrapAddrTestbed {}
impl Testbed for BackendOrganizationBootstrapAddrTestbed {
    impl_testbed_common!(BackendOrganizationBootstrapAddr);
    impl_testbed_with_org!(BackendOrganizationBootstrapAddr);
    fn url(&self) -> String {
        format!(
            "parsec://{}/{}?action=bootstrap_organization&token={}",
            DOMAIN, ORG, TOKEN
        )
    }
}

struct BackendOrganizationFileLinkAddrTestbed {}
impl Testbed for BackendOrganizationFileLinkAddrTestbed {
    impl_testbed_common!(BackendOrganizationFileLinkAddr);
    impl_testbed_with_org!(BackendOrganizationFileLinkAddr);
    fn url(&self) -> String {
        format!(
            "parsec://{}/{}?action=file_link&workspace_id={}&path={}",
            DOMAIN, ORG, WORKSPACE_ID, ENCRYPTED_PATH
        )
    }
}

struct BackendInvitationAddrTestbed {}
impl Testbed for BackendInvitationAddrTestbed {
    impl_testbed_common!(BackendInvitationAddr);
    impl_testbed_with_org!(BackendInvitationAddr);
    fn url(&self) -> String {
        format!(
            "parsec://{}/{}?action={}&token={}",
            DOMAIN, ORG, INVITATION_TYPE, TOKEN
        )
    }
}

#[template]
#[rstest(
    testbed,
    case::backend_addr(&BackendAddrTestbed{}),
    case::organization_addr(&BackendOrganizationAddrTestbed{}),
    case::organization_bootstrap_addr(&BackendOrganizationBootstrapAddrTestbed{}),
    case::organization_file_link_addr(&BackendOrganizationFileLinkAddrTestbed{}),
    case::invitation_addr(&BackendInvitationAddrTestbed{}),
)]
fn all_addr(testbed: &dyn Testbed) {}

#[template]
#[rstest(
    testbed,
    case::backend_organization_addr(&BackendOrganizationAddrTestbed{}),
    case::backend_organization_bootstrap_addr(&BackendOrganizationBootstrapAddrTestbed{}),
    case::backend_organization_file_link_addr(&BackendOrganizationFileLinkAddrTestbed{}),
    case::backend_invitation_addr(&BackendInvitationAddrTestbed{}),
)]
fn addr_with_org(testbed: &dyn Testbed) {}

/*
 * Actual tests
 */

#[apply(all_addr)]
fn test_good_addr(testbed: &dyn Testbed) {
    testbed.assert_addr_ok(&testbed.url());
}

#[rstest(value, path, expected)]
#[case::absolute_path(
    "parsec://example.com",
    Some("/foo/bar/"),
    "https://example.com/foo/bar/"
)]
#[case::relative_path("parsec://example.com", Some("foo/bar"), "https://example.com/foo/bar")]
#[case::root_path("parsec://example.com", Some("/"), "https://example.com/")]
#[case::empty_path("parsec://example.com", Some(""), "https://example.com/")]
#[case::no_path("parsec://example.com", None, "https://example.com/")]
#[case::ip_as_domain(
    "parsec://192.168.1.1:4242",
    Some("foo"),
    "https://192.168.1.1:4242/foo"
)]
#[case::no_ssl("parsec://example.com?no_ssl=true", None, "http://example.com/")]
#[case::no_ssl_false("parsec://example.com:443?no_ssl=false", None, "https://example.com/")]
#[case::no_ssl_with_port(
    "parsec://example.com:4242?no_ssl=true",
    None,
    "http://example.com:4242/"
)]
#[case::no_ssl_with_default_port(
    "parsec://example.com:80?no_ssl=true",
    None,
    "http://example.com/"
)]
#[case::default_port("parsec://example.com:443", None, "https://example.com/")]
#[case::non_default_port("parsec://example.com:80", None, "https://example.com:80/")]
#[case::unicode(
    "parsec://example.com",
    Some("你好"),
    "https://example.com/%E4%BD%A0%E5%A5%BD"
)]
#[case::unicode_with_space(
    "parsec://example.com",
    Some("/El Niño/"),
    "https://example.com/El%20Ni%C3%B1o/"
)]
fn test_backend_addr_to_http_domain_url(value: &str, path: Option<&str>, expected: &str) {
    let addr: BackendAddr = value.parse().unwrap();
    let result = addr.to_http_url_with_path(path);
    assert_eq!(result.as_str(), expected);
}

#[apply(all_addr)]
fn test_good_addr_with_port(testbed: &dyn Testbed) {
    let url = testbed.url().replace(DOMAIN, "example.com:4242");
    testbed.assert_addr_ok(&url);
}

#[apply(all_addr)]
fn test_addr_with_bad_port(testbed: &dyn Testbed, #[values("NaN", "999999")] bad_port: &str) {
    let url = testbed
        .url()
        .replace(DOMAIN, &format!("{}:{}", DOMAIN, bad_port));
    testbed.assert_addr_err(&url, "Invalid URL");
}

#[apply(all_addr)]
fn test_addr_with_no_hostname(testbed: &dyn Testbed, #[values("", ":4242")] bad_domain: &str) {
    let url = testbed.url().replace(DOMAIN, bad_domain);
    testbed.assert_addr_err(&url, "Invalid URL");
}

#[apply(all_addr)]
fn test_good_addr_with_unknown_field(testbed: &dyn Testbed) {
    let base_url = testbed.url();
    let mut url = url::Url::parse(&base_url).unwrap();
    url.query_pairs_mut().append_pair("unknown_field", "ok");
    testbed.assert_addr_ok_with_expected(url.as_str(), &base_url);
}

#[apply(addr_with_org)]
fn test_addr_with_unicode_organization_id(testbed: &dyn Testbed) {
    let org_name: OrganizationID = "康熙帝".parse().unwrap();
    let org_name_percent_quoted = "%E5%BA%B7%E7%86%99%E5%B8%9D";
    let url = testbed.url().replace(ORG, org_name_percent_quoted);
    testbed.assert_addr_ok(&url);
    assert_eq!(testbed.get_organization_id(&url), org_name);
}

#[apply(addr_with_org)]
fn test_addr_with_bad_unicode_organization_id(testbed: &dyn Testbed) {
    // Not a valid percent-encoded utf8 string
    let org_name_percent_quoted = "%E5%BA%B7%E7";
    let url = testbed.url().replace(ORG, org_name_percent_quoted);
    testbed.assert_addr_err(&url, "Path doesn't form a valid organization id");
}

#[apply(addr_with_org)]
fn test_addr_with_missing_organization_id(
    testbed: &dyn Testbed,
    #[values("/", "")] bad_path: &str,
) {
    let url = testbed.url().replace(
        &format!("{}/{}", DOMAIN, ORG),
        &format!("{}{}", DOMAIN, bad_path),
    );
    testbed.assert_addr_err(&url, "Path doesn't form a valid organization id");
}

#[rstest]
fn test_bootstrap_addr_bad_type(#[values(Some("dummy"), Some(""), None)] bad_type: Option<&str>) {
    let testbed = BackendOrganizationBootstrapAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace("bootstrap_organization", bad_type);
            testbed.assert_addr_err(&url, "Expected `action=bootstrap_organization` param value");
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace("action=bootstrap_organization&", "");
            testbed.assert_addr_err(&url, "Missing mandatory `action` param");
        }
    }
}

// Unlike for `BackendInvitationAddr`, here token is not required to be an UUID
#[test]
fn test_bootstrap_addr_unicode_token() {
    let testbed = BackendOrganizationBootstrapAddrTestbed {};
    let token = "康熙帝";
    let token_percent_quoted = "%E5%BA%B7%E7%86%99%E5%B8%9D";
    let url = testbed.url().replace(TOKEN, token_percent_quoted);
    testbed.assert_addr_ok(&url);

    let addr: BackendOrganizationBootstrapAddr = url.parse().unwrap();
    assert_eq!(addr.token(), Some(token));
}

// Unlike for `BackendInvitationAddr`, here token is not required to be an UUID
// Note: unlike with the Python implementation, invalid percent-encoding doesn't
// cause a failure (the replacement character EF BF BD is used instead)
#[test]
fn test_bootstrap_addr_bad_unicode_token() {
    let testbed = BackendOrganizationBootstrapAddrTestbed {};
    // Not a valid percent-encoded utf8 string
    let token_percent_quoted = "%E5%BA%B7%E7";
    let url = testbed.url().replace(TOKEN, token_percent_quoted);
    testbed.assert_addr_ok_with_expected(&url, &url.replace("%E7", "%EF%BF%BD"));
}

// Unlike for `BackendInvitationAddr`, here token is not required to be an UUID and can be missing
#[test]
fn test_bootstrap_addr_no_token() {
    let testbed = BackendOrganizationBootstrapAddrTestbed {};
    let url_with = testbed.url().replace(TOKEN, "");
    let url_without = testbed.url().replace(&format!("&token={}", TOKEN), "");

    // Token param present in the url but with an empty value
    testbed.assert_addr_ok_with_expected(&url_with, &url_without);
    let addr: BackendOrganizationBootstrapAddr = url_with.parse().unwrap();
    assert_eq!(addr.token(), None);

    // Token param not present in the url
    testbed.assert_addr_ok(&url_without);
    let addr: BackendOrganizationBootstrapAddr = url_without.parse().unwrap();
    assert_eq!(addr.token(), None);
}

#[rstest]
fn test_file_link_addr_bad_type(#[values(Some("dummy"), Some(""), None)] bad_type: Option<&str>) {
    let testbed = BackendOrganizationFileLinkAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace("file_link", bad_type);
            testbed.assert_addr_err(&url, "Expected `action=file_link` param value");
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace("action=file_link&", "");
            testbed.assert_addr_err(&url, "Missing mandatory `action` param");
        }
    }
}

#[rstest]
fn test_file_link_addr_bad_workspace(
    #[values(Some(""), Some("4def"), Some("康熙帝"), None)] bad_workspace: Option<&str>,
) {
    let testbed = BackendOrganizationFileLinkAddrTestbed {};

    match bad_workspace {
        Some(bad_workspace) => {
            // Workspace param present in the url but with bad and empty value
            let url = testbed.url().replace(WORKSPACE_ID, bad_workspace);
            testbed.assert_addr_err(&url, "Invalid `workspace_id` param value");
        }
        None => {
            // Workspace param not present in the url
            let url = testbed
                .url()
                .replace(&format!("workspace_id={}", WORKSPACE_ID), "");
            testbed.assert_addr_err(&url, "Missing mandatory `workspace_id` param");
        }
    }
}

#[rstest]
fn test_file_link_addr_bad_encrypted_path(
    #[values(Some("__notbase32__"), Some("康熙帝"), None)] bad_path: Option<&str>,
) {
    let testbed = BackendOrganizationFileLinkAddrTestbed {};

    match bad_path {
        Some(bad_path) => {
            // Path param present in the url but with bad and empty value
            let url = testbed.url().replace(ENCRYPTED_PATH, bad_path);
            testbed.assert_addr_err(&url, "Invalid `path` param value");
        }
        None => {
            // Path param not present in the url
            let url = testbed
                .url()
                .replace(&format!("path={}", ENCRYPTED_PATH), "");
            testbed.assert_addr_err(&url, "Missing mandatory `path` param");
        }
    }
}

#[test]
fn test_file_link_addr_get_encrypted_path() {
    let testbed = BackendOrganizationFileLinkAddrTestbed {};

    let serialized_encrypted_path = "HRSW4Y3SPFYHIZLEL5YGC6LMN5QWIPQs";
    let encrypted_path = b"<encrypted_payload>";

    let url = testbed
        .url()
        .replace(ENCRYPTED_PATH, serialized_encrypted_path);

    let addr: BackendOrganizationFileLinkAddr = url.parse().unwrap();
    assert_eq!(addr.encrypted_path(), encrypted_path);
}

#[rstest]
fn test_invitation_addr_bad_type(
    #[values(Some("claim"), Some("claim_foo"), None)] bad_type: Option<&str>,
) {
    let testbed = BackendInvitationAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace(INVITATION_TYPE, bad_type);
            testbed.assert_addr_err(
                &url,
                "Expected `action=claim_user` or `action=claim_device` param value",
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed
                .url()
                .replace(&format!("action={}&", INVITATION_TYPE), "");
            testbed.assert_addr_err(&url, "Missing mandatory `action` param");
        }
    }
}

#[rstest]
fn test_invitation_addr_bad_token(
    #[values(Some(""), Some("not_an_uuid"), Some("42"), Some("康熙帝"), None)] bad_token: Option<
        &str,
    >,
) {
    let testbed = BackendInvitationAddrTestbed {};

    match bad_token {
        Some(bad_token) => {
            // Token param present in the url but with and empty or bad value
            let url = testbed.url().replace(TOKEN, bad_token);
            testbed.assert_addr_err(&url, "Invalid `token` param value");
        }
        None => {
            // Token param not present in the url
            let url = testbed.url().replace(&format!("token={}", TOKEN), "");
            testbed.assert_addr_err(&url, "Missing mandatory `token` param");
        }
    }
}

#[test]
fn test_invitation_addr_types() {
    let testbed = BackendInvitationAddrTestbed {};

    let url = testbed.url().replace(INVITATION_TYPE, "claim_user");
    let addr: BackendInvitationAddr = url.parse().unwrap();
    assert_eq!(addr.invitation_type(), InvitationType::User);

    let url = testbed.url().replace(INVITATION_TYPE, "claim_device");
    let addr: BackendInvitationAddr = url.parse().unwrap();
    assert_eq!(addr.invitation_type(), InvitationType::Device);
}

#[rstest]
fn test_invitation_addr_to_redirection(#[values("http", "https")] redirection_scheme: &str) {
    let testbed = BackendInvitationAddrTestbed {};

    // `no_ssl` param should be ignored when build a redirection url given
    // this information is provided by the http/https scheme
    let mut url = testbed.url();
    if redirection_scheme == "http" {
        url.push_str("&no_ssl=true")
    }

    let addr: BackendInvitationAddr = url.parse().unwrap();
    let redirection_url = addr.to_http_redirection_url().to_string();

    let expected_redirection_url = testbed
        .url()
        .replacen("parsec", redirection_scheme, 1)
        .replace(ORG, &format!("redirect/{}", ORG));
    assert_eq!(redirection_url, expected_redirection_url);

    let addr2 = BackendInvitationAddr::from_http_redirection(&redirection_url).unwrap();
    assert_eq!(addr2, addr);
}

#[apply(all_addr)]
fn test_addr_to_redirection(
    testbed: &dyn Testbed,
    #[values("http", "https")] redirection_scheme: &str,
) {
    // `no_ssl` param should be ignored when build a redirection url given
    // this information is provided by the http/https scheme
    let mut url = testbed.url();
    if redirection_scheme == "http" {
        match url.find('?') {
            Some(_) => url.push('&'),
            None => url.push('?'),
        }
        url.push_str("no_ssl=true")
    }

    let redirection_url = testbed.to_redirection_url(&url);
    let expected_redirection_url = testbed
        .url()
        .replacen("parsec", redirection_scheme, 1)
        .replace(DOMAIN, &format!("{}/redirect", DOMAIN));
    assert_eq!(redirection_url, expected_redirection_url);

    testbed.assert_redirection_addr_ok(&redirection_url, &url);
}

macro_rules! test_redirection {
    ($addr_type:ty, $parsec_url:literal, $stable_parsec_url:literal, $redirection_url:literal, $stable_redirection_url:literal $(,)?) => {
        // From parsec scheme
        let addr: $addr_type = $parsec_url.parse().unwrap();
        assert_eq!(addr.clone().to_url().as_str(), $stable_parsec_url);
        assert_eq!(
            addr.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );

        // From redirection scheme
        let addr2 = <$addr_type>::from_http_redirection($redirection_url).unwrap();
        assert_eq!(&addr2, &addr);
        assert_eq!(addr2.clone().to_url().as_str(), $stable_parsec_url);
        assert_eq!(
            addr2.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );

        // From any
        let addr3 = <$addr_type>::from_any($parsec_url).unwrap();
        assert_eq!(&addr3, &addr);
        assert_eq!(addr3.clone().to_url().as_str(), $stable_parsec_url);
        assert_eq!(
            addr3.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );

        let addr4 = <$addr_type>::from_any($redirection_url).unwrap();
        assert_eq!(&addr4, &addr);
        assert_eq!(addr4.clone().to_url().as_str(), $stable_parsec_url);
        assert_eq!(
            addr4.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );
    };
    ($addr_type:ty, $parsec_url:literal, $stable_parsec_url:literal, $redirection_url:literal $(,)?) => {
        test_redirection!(
            $addr_type,
            $parsec_url,
            $stable_parsec_url,
            $redirection_url,
            $redirection_url
        );
    };
    ($addr_type:ty, $parsec_url:literal, $redirection_url:literal $(,)?) => {
        test_redirection!(
            $addr_type,
            $parsec_url,
            $parsec_url,
            $redirection_url,
            $redirection_url
        );
    };
}

#[test]
fn backend_addr_redirection() {
    test_redirection!(
        BackendAddr,
        "parsec://example.com",
        "https://example.com/redirect"
    );
    test_redirection!(
        BackendAddr,
        "parsec://example.com?no_ssl=false",
        "parsec://example.com",
        "https://example.com/redirect"
    );
    test_redirection!(
        BackendAddr,
        "parsec://example.com?no_ssl=true",
        "http://example.com/redirect"
    );

    test_redirection!(
        BackendOrganizationFileLinkAddr,
        "parsec://parsec.example.com/my_org?action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss",
        "https://parsec.example.com/redirect/my_org?action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss",
    );
    test_redirection!(
        BackendOrganizationFileLinkAddr,
        "parsec://parsec.example.com/my_org?action=file_link&no_ssl=true&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss",
        "parsec://parsec.example.com/my_org?no_ssl=true&action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss",
        "http://parsec.example.com/redirect/my_org?action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss",
    );
}

#[rstest]
#[case("https://foo.bar")]
#[case("https://foo.bar/redirection")]
#[case("https://foo.bar/not_valid")]
fn test_faulty_addr_redirection(#[case] raw_url: &str) {
    let res = dbg!(BackendAddr::from_http_redirection(raw_url));

    assert!(res.is_err());
}

#[rstest]
#[case("parsec://foo", 443, true)]
#[case("parsec://foo?no_ssl=false", 443, true)]
#[case("parsec://foo?no_ssl=true", 80, false)]
#[case("parsec://foo:42", 42, true)]
#[case("parsec://foo:42?dummy=", 42, true)]
#[case("parsec://foo:42?no_ssl=true", 42, false)]
#[case("parsec://foo:42?no_ssl=false&dummy=foo", 42, true)]
fn test_backend_addr_good(#[case] url: &str, #[case] port: u16, #[case] use_ssl: bool) {
    let addr = BackendAddr::from_str(url).unwrap();
    assert_eq!(addr.hostname(), "foo");
    assert_eq!(addr.port(), port);
    assert_eq!(addr.use_ssl(), use_ssl);
}

#[rstest]
#[case::empty("", "Invalid URL")]
#[case::invalid_url("foo", "Invalid URL")]
#[case::bad_scheme("xx://foo:42", "Must start with `parsec://`")]
#[case::path_not_allowed("parsec://foo:42/dummy", "Cannot have path")]
#[case::bad_parsing_in_valid_param(
    "parsec://foo:42?no_ssl",
    "Invalid `no_ssl` param value (must be true or false)"
)]
#[case::missing_value_for_param(
    "parsec://foo:42?no_ssl=",
    "Invalid `no_ssl` param value (must be true or false)"
)]
#[case::bad_value_for_param(
    "parsec://foo:42?no_ssl=nop",
    "Invalid `no_ssl` param value (must be true or false)"
)]
fn test_backend_addr_bad_value(#[case] url: &str, #[case] msg: &str) {
    assert_eq!(BackendAddr::from_str(url).unwrap_err().to_string(), msg);
}

#[rstest]
#[case("parsec://foo", 443, true)]
#[case("parsec://foo?no_ssl=false", 443, true)]
#[case("parsec://foo?no_ssl=true", 80, false)]
#[case("parsec://foo:42", 42, true)]
#[case("parsec://foo:42?dummy=", 42, true)]
#[case("parsec://foo:42?no_ssl=true", 42, false)]
#[case("parsec://foo:42?no_ssl=false", 42, true)]
#[case("parsec://foo:42?no_ssl=false&dummy=foo", 42, true)]
fn test_backend_organization_addr_good(
    #[case] base_url: &str,
    #[case] port: u16,
    #[case] use_ssl: bool,
) {
    let verify_key = SigningKey::generate().verify_key();
    let org = OrganizationID::from_str("org").unwrap();
    let backend_addr = BackendAddr::from_str(base_url).unwrap();
    let addr = BackendOrganizationAddr::new(backend_addr, org.clone(), verify_key.clone());

    assert_eq!(addr.hostname(), "foo");
    assert_eq!(addr.port(), port);
    assert_eq!(addr.use_ssl(), use_ssl);
    assert_eq!(addr.organization_id(), &org);
    assert_eq!(addr.root_verify_key(), &verify_key);

    let addr2 = BackendOrganizationAddr::from_str(addr.to_url().as_str()).unwrap();
    assert_eq!(addr, addr2);
}

#[rstest]
#[case::empty("", "Invalid URL")]
#[case::invalid_url("foo", "Invalid URL")]
#[case::missing_mandatory_rvk("parsec://foo:42/org", "Missing mandatory `rvk` param")]
#[case::missing_value_for_param("parsec://foo:42/org?rvk=", "Invalid `rvk` param value")]
#[case::bad_value_for_param("parsec://foo:42/org?rvk=nop", "Invalid `rvk` param value")]
#[case::missing_org_name(
    "parsec://foo:42?rvk=RAFI2CQYDHXMEY4NXEAJCTCBELJAUDE2OTYYLTVHGAGX57WS7LRQssss",
    "Path doesn't form a valid organization id"
)]
#[case::missing_org_name(
    "parsec://foo:42/?rvk=RAFI2CQYDHXMEY4NXEAJCTCBELJAUDE2OTYYLTVHGAGX57WS7LRQssss",
    "Path doesn't form a valid organization id"
)]
#[case::bad_org_name(
    "parsec://foo:42/bad/org?rvk=RAFI2CQYDHXMEY4NXEAJCTCBELJAUDE2OTYYLTVHGAGX57WS7LRQssss",
    "Path doesn't form a valid organization id"
)]
#[case::bad_org_name(
    "parsec://foo:42/~org?rvk=RAFI2CQYDHXMEY4NXEAJCTCBELJAUDE2OTYYLTVHGAGX57WS7LRQssss",
    "Path doesn't form a valid organization id"
)]
fn test_backend_organization_addr_bad_value(#[case] url: &str, #[case] msg: &str) {
    assert_eq!(
        BackendOrganizationAddr::from_str(url)
            .unwrap_err()
            .to_string(),
        msg
    );
}

#[rstest]
#[case("parsec://foo", 443, true)]
#[case("parsec://foo?no_ssl=false", 443, true)]
#[case("parsec://foo?no_ssl=true", 80, false)]
#[case("parsec://foo:42", 42, true)]
#[case("parsec://foo:42?dummy=foo", 42, true)]
#[case("parsec://foo:42?no_ssl=true", 42, false)]
#[case("parsec://foo:42?no_ssl=true&dummy=", 42, false)]
#[case("parsec://foo:42?no_ssl=false", 42, true)]
fn test_backend_organization_bootstrap_addr_good(
    #[case] base_url: &str,
    #[case] port: u16,
    #[case] use_ssl: bool,
) {
    let verify_key = SigningKey::generate().verify_key();
    let org = OrganizationID::from_str("org").unwrap();
    let backend_addr = BackendAddr::from_str(base_url).unwrap();
    let addr =
        BackendOrganizationBootstrapAddr::new(backend_addr, org.clone(), Some("token-123".into()));

    assert_eq!(addr.hostname(), "foo");
    assert_eq!(addr.port(), port);
    assert_eq!(addr.use_ssl(), use_ssl);
    assert_eq!(addr.organization_id(), &org);
    assert_eq!(addr.token().unwrap(), "token-123");

    let addr2 = BackendOrganizationBootstrapAddr::from_str(addr.to_url().as_str()).unwrap();
    assert_eq!(addr, addr2);

    let org_addr = addr.generate_organization_addr(verify_key.clone());
    assert_eq!(org_addr.root_verify_key(), &verify_key);
    assert_eq!(org_addr.hostname(), addr.hostname());
    assert_eq!(org_addr.port(), addr.port());
    assert_eq!(org_addr.use_ssl(), addr.use_ssl());
    assert_eq!(org_addr.organization_id(), addr.organization_id());
}

#[rstest]
#[case::empty("", "Invalid URL")]
#[case::invalid_url("foo", "Invalid URL")]
#[case::missing_action("parsec://foo:42/org?token=123", "Missing mandatory `action` param")]
#[case::bad_action(
    "parsec://foo:42/org?action=dummy&token=123",
    "Expected `action=bootstrap_organization` param value"
)]
#[case::org_name(
    "parsec://foo:42?action=bootstrap_organization&token=123",
    "Path doesn't form a valid organization id"
)]
#[case::missing_org_name(
    "parsec://foo:42?action=bootstrap_organization&token=123",
    "Path doesn't form a valid organization id"
)]
#[case::missing_org_name(
    "parsec://foo:42/?action=bootstrap_organization&token=123",
    "Path doesn't form a valid organization id"
)]
#[case::bad_org_name(
    "parsec://foo:42/bad/org?action=bootstrap_organization&token=123",
    "Path doesn't form a valid organization id"
)]
#[case::bad_org_name(
    "parsec://foo:42/~org?action=bootstrap_organization&token=123",
    "Path doesn't form a valid organization id"
)]
fn test_backend_organization_bootstrap_addr_bad_value(#[case] url: &str, #[case] msg: &str) {
    assert_eq!(
        BackendOrganizationBootstrapAddr::from_str(url)
            .unwrap_err()
            .to_string(),
        msg
    );
}
