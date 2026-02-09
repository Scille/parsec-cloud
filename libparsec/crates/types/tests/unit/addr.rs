// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
// cspell:ignore xBCgAAAAAAAAAAAAAAAAAAAB

use rstest_reuse::{self, apply, template};
use std::str::FromStr;

use libparsec_tests_lite::prelude::*;

use crate::prelude::*;

const ORG: &str = "MyOrg";
const TOKEN: &str = "a0000000000000000000000000000001";
const DOMAIN: &str = "parsec.cloud.com";
// cspell:disable-next-line
const ORGANIZATION_ADDR_PARAM: &str = "xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA";
// cspell:disable-next-line
const ORGANIZATION_BOOTSTRAP_ADDR_PARAM: &str = "xBCgAAAAAAAAAAAAAAAAAAAB";
// cspell:disable-next-line
const ORGANIZATION_BOOTSTRAP_ADDR_PARAM_NO_TOKEN: &str = "wA";
// cspell:disable-next-line
const INVITATION_PARAM: &str = "xBCgAAAAAAAAAAAAAAAAAAAB";
// cspell:disable-next-line
const WORKSPACE_PATH_PARAM: &str = "k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A";
// `/foo/bar.txt` encrypted with key `2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1`
// cspell:disable-next-line
const WORKSPACE_PATH_PARAM_ENCRYPTED_PATH: &[u8] = &hex!("e167d4e1d9bd33abbe067afba20daf765bbf6c238b2407d95a2e3217046dcc4080d2239d7f1ca63b3e6de6e838377e36223c5370");
const WORKSPACE_PATH_PARAM_KEY_INDEX: IndexInt = 1;
// cspell:disable-next-line
const WORKSPACE_PATH_PARAM_WORKSPACE_ID: &[u8] = &hex!("2d4ded1274064608833b7f57f01156e2");
const INVITATION_TYPE: &str = "claim_user";
const PKI_ENROLLMENT_TYPE: &str = "pki_enrollment";
const ASYNC_ENROLLMENT_TYPE: &str = "async_enrollment";

/*
 * Helpers to parametrize the tests on different addr types
 */

trait Testbed {
    fn url(&self) -> String;

    fn assert_addr_ok(&self, url: &str);
    fn assert_addr_ok_with_expected(&self, url: &str, expected_url: &str);
    fn assert_addr_err(&self, url: &str, err_msg: AddrError);

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
            p_assert_eq!(addr.to_url().as_str(), expected_url);

            // Also compare the objects
            p_assert_eq!(addr, expected_addr);

            // Finally also test from_any
            let addr2 = <$addr_type>::from_any(url).unwrap();
            p_assert_eq!(addr2, expected_addr);
        }
        fn assert_addr_err(&self, url: &str, err_msg: AddrError) {
            let ret = url.parse::<$addr_type>();
            p_assert_eq!(ret, Err(err_msg));
        }
        fn assert_redirection_addr_ok(&self, redirection_url: &str, expected_url: &str) {
            // From redirection scheme
            let expected_addr: $addr_type = expected_url.parse().unwrap();
            let addr = <$addr_type>::from_http_redirection(redirection_url).unwrap();
            p_assert_eq!(addr, expected_addr);

            // From any
            let addr2 = <$addr_type>::from_any(redirection_url).unwrap();
            p_assert_eq!(addr2, expected_addr);
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

struct AddrTestbed {}
impl Testbed for AddrTestbed {
    impl_testbed_common!(ParsecAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}")
    }
}

struct OrganizationAddrTestbed {}
impl Testbed for OrganizationAddrTestbed {
    impl_testbed_common!(ParsecOrganizationAddr);
    impl_testbed_with_org!(ParsecOrganizationAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}/{ORG}?p={ORGANIZATION_ADDR_PARAM}")
    }
}

struct OrganizationBootstrapAddrTestbed {}
impl Testbed for OrganizationBootstrapAddrTestbed {
    impl_testbed_common!(ParsecOrganizationBootstrapAddr);
    impl_testbed_with_org!(ParsecOrganizationBootstrapAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}/{ORG}?a=bootstrap_organization&p={ORGANIZATION_BOOTSTRAP_ADDR_PARAM}",)
    }
}

struct WorkspacePathAddrTestbed {}
impl Testbed for WorkspacePathAddrTestbed {
    impl_testbed_common!(ParsecWorkspacePathAddr);
    impl_testbed_with_org!(ParsecWorkspacePathAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}/{ORG}?a=path&p={WORKSPACE_PATH_PARAM}",)
    }
}

struct InvitationAddrTestbed {}
impl Testbed for InvitationAddrTestbed {
    impl_testbed_common!(ParsecInvitationAddr);
    impl_testbed_with_org!(ParsecInvitationAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}/{ORG}?a={INVITATION_TYPE}&p={INVITATION_PARAM}",)
    }
}

struct PKIEnrollmentAddrTestbed {}
impl Testbed for PKIEnrollmentAddrTestbed {
    impl_testbed_common!(ParsecPkiEnrollmentAddr);
    impl_testbed_with_org!(ParsecPkiEnrollmentAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}/{ORG}?a={PKI_ENROLLMENT_TYPE}",)
    }
}

struct AsyncEnrollmentAddrTestbed {}
impl Testbed for AsyncEnrollmentAddrTestbed {
    impl_testbed_common!(ParsecAsyncEnrollmentAddr);
    impl_testbed_with_org!(ParsecAsyncEnrollmentAddr);
    fn url(&self) -> String {
        format!("{PARSEC_SCHEME}://{DOMAIN}/{ORG}?a={ASYNC_ENROLLMENT_TYPE}",)
    }
}

#[template]
#[rstest(
    testbed,
    case::server_addr(&AddrTestbed{}),
    case::organization_addr(&OrganizationAddrTestbed{}),
    case::organization_bootstrap_addr(&OrganizationBootstrapAddrTestbed{}),
    case::workspace_path_addr(&WorkspacePathAddrTestbed{}),
    case::invitation_addr(&InvitationAddrTestbed{}),
    case::pki_enrollment_addr(&PKIEnrollmentAddrTestbed{}),
    case::async_enrollment_addr(&AsyncEnrollmentAddrTestbed{}),
)]
fn all_addr(testbed: &dyn Testbed) {}

#[template]
#[rstest(
    testbed,
    case::organization_addr(&OrganizationAddrTestbed{}),
    case::organization_bootstrap_addr(&OrganizationBootstrapAddrTestbed{}),
    case::workspace_path_addr(&WorkspacePathAddrTestbed{}),
    case::invitation_addr(&InvitationAddrTestbed{}),
    case::pki_enrollment_addr(&PKIEnrollmentAddrTestbed{}),
    case::async_enrollment_addr(&AsyncEnrollmentAddrTestbed{}),
)]
fn addr_with_org(testbed: &dyn Testbed) {}

/*
 * Actual tests
 */

#[rstest_reuse::apply(all_addr)]
fn good_addr_base(testbed: &dyn Testbed) {
    testbed.assert_addr_ok(&testbed.url());
}

#[rstest(value, assert_outcome)]
#[case::ok(
    "https://example.com",
    |outcome| { p_assert_eq!(outcome, Ok("parsec3://example.com".parse().unwrap())); },
)]
#[case::ok_http(
    "http://example.com",
    |outcome| { p_assert_eq!(outcome, Ok("parsec3://example.com?no_ssl=true".parse().unwrap())); },
)]
#[case::ok_trailing_slash(
    "https://example.com/",
    |outcome| { p_assert_eq!(outcome, Ok("parsec3://example.com".parse().unwrap())); },
)]
#[case::ok_unused_param(
    "https://example.com?foo=bar",
    |outcome| { p_assert_eq!(outcome, Ok("parsec3://example.com".parse().unwrap())); },
)]
#[case::invalid_url(
    "<dummy>",
    |outcome| { p_assert_matches!(outcome, Err(AddrError::InvalidUrl(_))); },
)]
#[case::no_scheme(
    "example.com/foo/",
    |outcome| { p_assert_matches!(outcome, Err(AddrError::InvalidUrl(_))); },
)]
#[case::should_not_have_path(
    "https://example.com/foo/",
    |outcome| { p_assert_matches!(outcome, Err(AddrError::ShouldNotHaveAPath)); },
)]
#[case::bad_scheme(
    "xxx://example.com/foo/",
    |outcome| { p_assert_matches!(outcome, Err(AddrError::InvalidUrlScheme{..})); },
)]
#[case::bad_scheme_parsec(
    "parsec3://example.com/foo/",
    |outcome| { p_assert_matches!(outcome, Err(AddrError::InvalidUrlScheme{..})); },
)]
fn parsec_addr_from_http_url(value: &str, assert_outcome: fn(Result<ParsecAddr, AddrError>)) {
    let outcome = ParsecAddr::from_http_url(value);
    assert_outcome(outcome);
}

#[rstest(value, path, expected)]
#[case::absolute_path(
    "parsec3://example.com",
    Some("/foo/bar/"),
    "https://example.com/foo/bar/"
)]
#[case::relative_path(
    "parsec3://example.com",
    Some("foo/bar"),
    "https://example.com/foo/bar"
)]
#[case::root_path("parsec3://example.com", Some("/"), "https://example.com/")]
#[case::empty_path("parsec3://example.com", Some(""), "https://example.com/")]
#[case::no_path("parsec3://example.com", None, "https://example.com/")]
#[case::ip_as_domain(
    "parsec3://192.168.1.1:4242",
    Some("foo"),
    "https://192.168.1.1:4242/foo"
)]
#[case::no_ssl("parsec3://example.com?no_ssl=true", None, "http://example.com/")]
#[case::no_ssl_false("parsec3://example.com:443?no_ssl=false", None, "https://example.com/")]
#[case::no_ssl_with_port(
    "parsec3://example.com:4242?no_ssl=true",
    None,
    "http://example.com:4242/"
)]
#[case::no_ssl_with_default_port(
    "parsec3://example.com:80?no_ssl=true",
    None,
    "http://example.com/"
)]
#[case::default_port("parsec3://example.com:443", None, "https://example.com/")]
#[case::non_default_port("parsec3://example.com:80", None, "https://example.com:80/")]
#[case::unicode(
    "parsec3://example.com",
    Some("你好"),
    "https://example.com/%E4%BD%A0%E5%A5%BD"
)]
#[case::unicode_with_space(
    "parsec3://example.com",
    Some("/El Niño/"),
    "https://example.com/El%20Ni%C3%B1o/"
)]
fn parsec_addr_to_http_url(value: &str, path: Option<&str>, expected: &str) {
    let addr: ParsecAddr = value.parse().unwrap();
    let result = addr.to_http_url(path);
    p_assert_eq!(result.as_str(), expected);
}

#[apply(all_addr)]
fn good_addr_with_port(testbed: &dyn Testbed) {
    let url = testbed.url().replace(DOMAIN, "example.com:4242");
    testbed.assert_addr_ok(&url);
}

// By specification, URL parser knowns how to handle common noise in the input
// (i.e. leading/trailing whitespaces & C0 control chars, tabs, newlines).
//
// see https://url.spec.whatwg.org/#concept-basic-url-parser
#[apply(all_addr)]
fn good_addr_with_noise_trimmed_by_url_parser(testbed: &dyn Testbed) {
    let expected_url = testbed.url();
    let url = {
        // Add noise to the url, those should be stripped by the url parser
        let mut url = String::with_capacity(expected_url.len() * 2);
        for (i, c) in expected_url.chars().enumerate() {
            url.push(c);
            if i % 2 == 0 {
                url.push('\t');
            } else {
                url.push('\n');
            }
        }
        format!(" {url} ")
    };
    testbed.assert_addr_ok_with_expected(&url, &expected_url);
}

#[apply(all_addr)]
fn addr_with_bad_port(testbed: &dyn Testbed, #[values("NaN", "999999")] bad_port: &str) {
    let url = testbed
        .url()
        .replace(DOMAIN, &format!("{DOMAIN}:{bad_port}"));
    testbed.assert_addr_err(&url, AddrError::InvalidUrl(url::ParseError::InvalidPort));
}

#[apply(all_addr)]
fn addr_with_bad_scheme(
    testbed: &dyn Testbed,
    #[values("xxx", "http", "https", "parsec4", "parsec", "parsec3x")] bad_scheme: &str,
) {
    let url = testbed.url().replacen(PARSEC_SCHEME, bad_scheme, 1);
    testbed.assert_addr_err(
        &url,
        AddrError::InvalidUrlScheme {
            expected: PARSEC_SCHEME,
        },
    );
}

#[apply(all_addr)]
fn addr_with_no_hostname(testbed: &dyn Testbed, #[values("", ":4242")] bad_domain: &str) {
    let (url, expected_error) = if bad_domain.is_empty() {
        // `http:///foo` is a valid url, so we also have to remove the path
        let url = match testbed.url().split('?').nth(1) {
            Some(extra) => format!("parsec3://?{extra}"),
            None => "parsec3://".to_string(),
        };
        let expected_error = AddrError::InvalidUrl(url::ParseError::EmptyHost);
        (url, expected_error)
    } else {
        let url = testbed.url().replace(DOMAIN, bad_domain);
        let expected_error = AddrError::InvalidUrl(url::ParseError::EmptyHost);
        (url, expected_error)
    };
    testbed.assert_addr_err(&url, expected_error);
}

// Based on a true debug story...
#[apply(all_addr)]
fn addr_with_bad_hostname(
    testbed: &dyn Testbed,
    #[values("1270.0.1", "1270.0.1:4242")] bad_domain: &str,
) {
    let url = testbed.url().replace(DOMAIN, bad_domain);
    testbed.assert_addr_err(
        &url,
        AddrError::InvalidUrl(url::ParseError::InvalidIpv4Address),
    );
}

#[apply(all_addr)]
fn good_addr_with_unknown_field(testbed: &dyn Testbed) {
    let base_url = testbed.url();
    let mut url = url::Url::parse(&base_url).unwrap();
    url.query_pairs_mut().append_pair("unknown_field", "ok");
    testbed.assert_addr_ok_with_expected(url.as_str(), &base_url);
}

#[apply(addr_with_org)]
fn addr_with_unicode_organization_id(testbed: &dyn Testbed) {
    let org_name: OrganizationID = "康熙帝".parse().unwrap();
    let org_name_percent_quoted = "%E5%BA%B7%E7%86%99%E5%B8%9D";
    let url = testbed.url().replace(ORG, org_name_percent_quoted);
    testbed.assert_addr_ok(&url);
    p_assert_eq!(testbed.get_organization_id(&url), org_name);
}

#[apply(addr_with_org)]
fn addr_with_bad_unicode_organization_id(testbed: &dyn Testbed) {
    // Not a valid percent-encoded utf8 string
    let org_name_percent_quoted = "%E5%BA%B7%E7";
    let url = testbed.url().replace(ORG, org_name_percent_quoted);
    testbed.assert_addr_err(&url, AddrError::InvalidOrganizationID);
}

#[apply(addr_with_org)]
fn addr_with_missing_organization_id(testbed: &dyn Testbed, #[values("/", "")] bad_path: &str) {
    let url = testbed
        .url()
        .replace(&format!("{DOMAIN}/{ORG}"), &format!("{DOMAIN}{bad_path}"));
    testbed.assert_addr_err(&url, AddrError::InvalidOrganizationID);
}

#[rstest]
fn bootstrap_addr_bad_type(#[values(Some("dummy"), Some(""), None)] bad_type: Option<&str>) {
    let testbed = OrganizationBootstrapAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace("bootstrap_organization", bad_type);
            testbed.assert_addr_err(
                &url,
                AddrError::InvalidParamValue {
                    param: "a",
                    help: "Expected `a=bootstrap_organization`".to_string(),
                },
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace("a=bootstrap_organization&", "");
            testbed.assert_addr_err(&url, AddrError::MissingParam("a"));
        }
    }
}

#[rstest]
fn bootstrap_addr_token() {
    let testbed = OrganizationBootstrapAddrTestbed {};

    // Url with token
    let addr: ParsecOrganizationBootstrapAddr = testbed.url().parse().unwrap();
    let expected_token = AccessToken::from_hex(TOKEN).unwrap();
    p_assert_eq!(addr.token(), Some(&expected_token));

    // Url without token (for spontaneous bootstrap)
    let url = testbed.url().replace(
        ORGANIZATION_BOOTSTRAP_ADDR_PARAM,
        ORGANIZATION_BOOTSTRAP_ADDR_PARAM_NO_TOKEN,
    );
    let addr: ParsecOrganizationBootstrapAddr = url.parse().unwrap();
    p_assert_eq!(addr.token(), None);
}

#[rstest]
#[case::empty("")]
#[case::not_base64("<dummy>")]
// cspell:disable-next-line
#[case::base64_over_not_msgpack("ZHVtbXk")] // Base64("dummy")
// cspell:disable-next-line
#[case::empty_token("xAA")] // Base64(Msgpack(b""))
// cspell:disable-next-line
#[case::token_too_short("xAEA")] // Base64(Msgpack(b"\x00"))
// cspell:disable-next-line
#[case::token_too_long("xBEAAAAAAAAAAAAAAAAAAAAAAA")] // Base64(Msgpack(b"\x00" * 17))
fn bootstrap_addr_bad_payload(#[case] bad_payload: &str) {
    let testbed = OrganizationBootstrapAddrTestbed {};
    let url = testbed
        .url()
        .replace(ORGANIZATION_BOOTSTRAP_ADDR_PARAM, bad_payload);
    testbed.assert_addr_err(
        &url,
        AddrError::InvalidParamValue {
            param: "p",
            help: "Invalid `p` parameter".into(),
        },
    );
}

#[rstest]
fn workspace_path_addr_bad_type(#[values(Some("dummy"), Some(""), None)] bad_type: Option<&str>) {
    let testbed = WorkspacePathAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace("path", bad_type);
            testbed.assert_addr_err(
                &url,
                AddrError::InvalidParamValue {
                    param: "a",
                    help: "Expected `a=path`".to_string(),
                },
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace("a=path&", "");
            testbed.assert_addr_err(&url, AddrError::MissingParam("a"));
        }
    }
}

#[test]
fn workspace_path_addr_get_params() {
    let testbed = WorkspacePathAddrTestbed {};

    let url = testbed.url();

    let addr: ParsecWorkspacePathAddr = url.parse().unwrap();
    p_assert_eq!(addr.encrypted_path(), WORKSPACE_PATH_PARAM_ENCRYPTED_PATH);
    p_assert_eq!(addr.key_index(), WORKSPACE_PATH_PARAM_KEY_INDEX);
    p_assert_eq!(
        addr.workspace_id(),
        WORKSPACE_PATH_PARAM_WORKSPACE_ID.try_into().unwrap()
    );
}

#[rstest]
#[case::empty("")]
#[case::not_base64("<dummy>")]
// cspell:disable-next-line
#[case::base64_over_not_msgpack("ZHVtbXk")] // Base64("dummy")
// cspell:disable-next-line
#[case::bad_data("xAA")] // Base64(Msgpack(b""))
fn workspace_path_addr_bad_payload(#[case] bad_payload: &str) {
    let testbed = WorkspacePathAddrTestbed {};
    let url = testbed.url().replace(WORKSPACE_PATH_PARAM, bad_payload);
    testbed.assert_addr_err(
        &url,
        AddrError::InvalidParamValue {
            param: "p",
            help: "Invalid `p` parameter".into(),
        },
    );
}

#[rstest]
fn invitation_addr_bad_type(
    #[values(Some("claim"), Some("claim_foo"), None)] bad_type: Option<&str>,
) {
    let testbed = InvitationAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace(INVITATION_TYPE, bad_type);
            testbed.assert_addr_err(
                &url,
                AddrError::InvalidParamValue {
                    param: "a",
                    help: "Expected `a=claim_user`, `a=claim_device` or `a=claim_shamir_recovery`"
                        .to_string(),
                },
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace(&format!("a={INVITATION_TYPE}&"), "");
            testbed.assert_addr_err(&url, AddrError::MissingParam("a"));
        }
    }
}

#[rstest]
fn invitation_addr_token() {
    let testbed = InvitationAddrTestbed {};
    let addr: ParsecInvitationAddr = testbed.url().parse().unwrap();
    let expected_token = AccessToken::from_hex(TOKEN).unwrap();
    p_assert_eq!(addr.token(), expected_token);
}

#[rstest]
#[case::empty("")]
#[case::not_base64("<dummy>")]
// cspell:disable-next-line
#[case::base64_over_not_msgpack("ZHVtbXk")] // Base64("dummy")
// cspell:disable-next-line
#[case::empty_token("xAA")] // Base64(Msgpack(b""))
// cspell:disable-next-line
#[case::token_too_short("xAEA")] // Base64(Msgpack(b"\x00"))
// cspell:disable-next-line
#[case::token_too_long("xBEAAAAAAAAAAAAAAAAAAAAAAA")] // Base64(Msgpack(b"\x00" * 17))
fn invitation_addr_bad_payload(#[case] bad_payload: &str) {
    let testbed = InvitationAddrTestbed {};
    let url = testbed.url().replace(INVITATION_PARAM, bad_payload);
    testbed.assert_addr_err(
        &url,
        AddrError::InvalidParamValue {
            param: "p",
            help: "Invalid `p` parameter".into(),
        },
    );
}

#[test]
fn invitation_addr_types() {
    let testbed = InvitationAddrTestbed {};

    let url = testbed.url().replace(INVITATION_TYPE, "claim_user");
    let addr: ParsecInvitationAddr = url.parse().unwrap();
    p_assert_eq!(addr.invitation_type(), InvitationType::User);

    let url = testbed.url().replace(INVITATION_TYPE, "claim_device");
    let addr: ParsecInvitationAddr = url.parse().unwrap();
    p_assert_eq!(addr.invitation_type(), InvitationType::Device);
}

#[rstest]
fn invitation_addr_to_redirection(#[values("http", "https")] redirection_scheme: &str) {
    let testbed = InvitationAddrTestbed {};

    // `no_ssl` param should be ignored when build a redirection url given
    // this information is provided by the http/https scheme
    let mut url = testbed.url();
    if redirection_scheme == "http" {
        url.push_str("&no_ssl=true")
    }

    let addr: ParsecInvitationAddr = url.parse().unwrap();
    let redirection_url = addr.to_http_redirection_url().to_string();

    let expected_redirection_url = testbed
        .url()
        .replacen(PARSEC_SCHEME, redirection_scheme, 1)
        .replace(ORG, &format!("redirect/{ORG}"));
    p_assert_eq!(redirection_url, expected_redirection_url);

    let addr2 = ParsecInvitationAddr::from_http_redirection(&redirection_url).unwrap();
    p_assert_eq!(addr2, addr);
}

#[apply(all_addr)]
fn addr_to_redirection(testbed: &dyn Testbed, #[values("http", "https")] redirection_scheme: &str) {
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
        .replacen(PARSEC_SCHEME, redirection_scheme, 1)
        .replace(DOMAIN, &format!("{DOMAIN}/redirect"));
    p_assert_eq!(redirection_url, expected_redirection_url);

    testbed.assert_redirection_addr_ok(&redirection_url, &url);
}

macro_rules! test_redirection {
    ($addr_type:ty, $parsec_url:literal, $stable_parsec_url:literal, $redirection_url:literal, $stable_redirection_url:literal $(,)?) => {
        // From parsec scheme
        let addr: $addr_type = $parsec_url.parse().unwrap();
        p_assert_eq!(addr.clone().to_url().as_str(), $stable_parsec_url);
        p_assert_eq!(
            addr.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );

        // From redirection scheme
        let addr2 = <$addr_type>::from_http_redirection($redirection_url).unwrap();
        p_assert_eq!(&addr2, &addr);
        p_assert_eq!(addr2.clone().to_url().as_str(), $stable_parsec_url);
        p_assert_eq!(
            addr2.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );

        // From any
        let addr3 = <$addr_type>::from_any($parsec_url).unwrap();
        p_assert_eq!(&addr3, &addr);
        p_assert_eq!(addr3.clone().to_url().as_str(), $stable_parsec_url);
        p_assert_eq!(
            addr3.clone().to_http_redirection_url().as_str(),
            $stable_redirection_url
        );

        let addr4 = <$addr_type>::from_any($redirection_url).unwrap();
        p_assert_eq!(&addr4, &addr);
        p_assert_eq!(addr4.clone().to_url().as_str(), $stable_parsec_url);
        p_assert_eq!(
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
fn parsec_addr_redirection() {
    test_redirection!(
        ParsecAddr,
        "parsec3://example.com",
        "https://example.com/redirect"
    );
    test_redirection!(
        ParsecAddr,
        "parsec3://example.com?no_ssl=false",
        "parsec3://example.com",
        "https://example.com/redirect"
    );
    test_redirection!(
        ParsecAddr,
        "parsec3://example.com?no_ssl=true",
        "http://example.com/redirect"
    );

    test_redirection!(
        ParsecWorkspacePathAddr,
        // cspell:disable-next-line
        "parsec3://parsec.example.com/my_org?a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",  // cspell:disable-line
        // cspell:disable-next-line
        "https://parsec.example.com/redirect/my_org?a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",  // cspell:disable-line
    );
    test_redirection!(
        ParsecWorkspacePathAddr,
        // cspell:disable-next-line
        "parsec3://parsec.example.com/my_org?a=path&no_ssl=true&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",  // cspell:disable-line
        // cspell:disable-next-line
        "parsec3://parsec.example.com/my_org?no_ssl=true&a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",  // cspell:disable-line
        // cspell:disable-next-line
        "http://parsec.example.com/redirect/my_org?a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",  // cspell:disable-line
    );
}

#[rstest]
#[case("https://foo.bar")]
#[case("https://foo.bar/redirection")]
#[case("parsec3://foo.bar/redirection/my_org")]
#[case("xxx://foo.bar/redirection/my_org")]
#[case("https://foo.bar/not_valid")]
#[case("http://1270.0.1/redirect")]
#[case("http://foo:99999/redirect")]
fn faulty_addr_redirection(#[case] raw_url: &str) {
    let res = ParsecAddr::from_http_redirection(raw_url);
    assert!(res.is_err());

    // Also test from_any

    let res = ParsecAddr::from_any(raw_url);
    assert!(res.is_err());
}

#[rstest]
#[case("parsec3://foo", 443, true)]
#[case("parsec3://foo?no_ssl=false", 443, true)]
#[case("parsec3://foo?no_ssl=true", 80, false)]
#[case("parsec3://foo:42", 42, true)]
#[case("parsec3://foo:42?dummy=", 42, true)]
#[case("parsec3://foo:42?no_ssl=true", 42, false)]
#[case("parsec3://foo:42?no_ssl=false&dummy=foo", 42, true)]
fn parsec_addr_good(#[case] url: &str, #[case] port: u16, #[case] use_ssl: bool) {
    let addr = ParsecAddr::from_str(url).unwrap();
    p_assert_eq!(addr.hostname(), "foo");
    p_assert_eq!(addr.port(), port);
    p_assert_eq!(addr.use_ssl(), use_ssl);
}

#[rstest]
#[case::empty("", AddrError::InvalidUrl(url::ParseError::RelativeUrlWithoutBase))]
#[case::invalid_url("foo", AddrError::InvalidUrl(url::ParseError::RelativeUrlWithoutBase))]
#[case::bad_scheme("xx://foo:42", AddrError::InvalidUrlScheme { expected: PARSEC_SCHEME })]
#[case::path_not_allowed("parsec3://foo:42/dummy", AddrError::ShouldNotHaveAPath)]
#[case::bad_parsing_in_valid_param(
    "parsec3://foo:42?no_ssl",
    AddrError::InvalidParamValue {
         param: "no_ssl",
         help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
    }
)]
#[case::missing_value_for_param(
    "parsec3://foo:42?no_ssl=",
    AddrError::InvalidParamValue {
         param: "no_ssl",
         help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
    }
)]
#[case::bad_value_for_param(
    "parsec3://foo:42?no_ssl=nop",
    AddrError::InvalidParamValue {
         param: "no_ssl",
         help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
    }
)]
fn parsec_addr_bad_value(#[case] url: &str, #[case] msg: AddrError) {
    p_assert_eq!(ParsecAddr::from_str(url).unwrap_err(), msg);
}

#[rstest]
#[case("parsec3://foo", 443, true)]
#[case("parsec3://foo?no_ssl=false", 443, true)]
#[case("parsec3://foo?no_ssl=true", 80, false)]
#[case("parsec3://foo:42", 42, true)]
#[case("parsec3://foo:42?dummy=", 42, true)]
#[case("parsec3://foo:42?no_ssl=true", 42, false)]
#[case("parsec3://foo:42?no_ssl=false", 42, true)]
#[case("parsec3://foo:42?no_ssl=false&dummy=foo", 42, true)]
fn organization_addr_good(#[case] base_url: &str, #[case] port: u16, #[case] use_ssl: bool) {
    let verify_key = SigningKey::generate().verify_key();
    let org = OrganizationID::from_str("org").unwrap();
    let server_addr = ParsecAddr::from_str(base_url).unwrap();
    let addr = ParsecOrganizationAddr::new(server_addr, org.clone(), verify_key.clone());

    p_assert_eq!(addr.hostname(), "foo");
    p_assert_eq!(addr.port(), port);
    p_assert_eq!(addr.use_ssl(), use_ssl);
    p_assert_eq!(addr.organization_id(), &org);
    p_assert_eq!(addr.root_verify_key(), &verify_key);

    let addr2 = ParsecOrganizationAddr::from_str(addr.to_url().as_str()).unwrap();
    p_assert_eq!(addr, addr2);
}

#[rstest]
#[case::missing_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42?p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA",
    AddrError::InvalidOrganizationID
)]
#[case::missing_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42/?p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA",
    AddrError::InvalidOrganizationID
)]
#[case::bad_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42/bad/org?p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA",
    AddrError::InvalidOrganizationID
)]
#[case::bad_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42/~org?p=xCBs8zpdIwovR8EdliVVo2vUOmtumnfsI6Fdndjm0WconA",
    AddrError::InvalidOrganizationID
)]
#[case::payload_not_b64(
    "parsec3://foo:42/org?p=dummy",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_not_msgpack(
    // Base64(b"dummy")
    // cspell:disable-next-line
    "parsec3://foo:42/org?p=ZHVtbXk",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_empty_root_verify_key(
    // Base64(Msgpack(b""))
    // cspell:disable-next-line
    "parsec3://foo:42/org?p=xAA",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_bad_root_verify_key(
    // Base64(Msgpack(b"\x00"))
    // cspell:disable-next-line
    "parsec3://foo:42/org?p=xAEA",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
fn organization_addr_bad_value(#[case] url: &str, #[case] msg: AddrError) {
    p_assert_eq!(ParsecOrganizationAddr::from_str(url).unwrap_err(), msg);
}

#[rstest]
#[case("parsec3://foo", 443, true)]
#[case("parsec3://foo?no_ssl=false", 443, true)]
#[case("parsec3://foo?no_ssl=true", 80, false)]
#[case("parsec3://foo:42", 42, true)]
#[case("parsec3://foo:42?dummy=foo", 42, true)]
#[case("parsec3://foo:42?no_ssl=true", 42, false)]
#[case("parsec3://foo:42?no_ssl=true&dummy=", 42, false)]
#[case("parsec3://foo:42?no_ssl=false", 42, true)]
fn organization_bootstrap_addr_good(
    #[case] base_url: &str,
    #[case] port: u16,
    #[case] use_ssl: bool,
) {
    let verify_key = SigningKey::generate().verify_key();
    let org = OrganizationID::from_str("org").unwrap();
    let server_addr = ParsecAddr::from_str(base_url).unwrap();
    let token = AccessToken::from_hex(TOKEN).unwrap();
    let addr = ParsecOrganizationBootstrapAddr::new(server_addr, org.clone(), Some(token));

    p_assert_eq!(addr.hostname(), "foo");
    p_assert_eq!(addr.port(), port);
    p_assert_eq!(addr.use_ssl(), use_ssl);
    p_assert_eq!(addr.organization_id(), &org);
    p_assert_eq!(addr.token(), Some(&token));

    let addr2 = ParsecOrganizationBootstrapAddr::from_str(addr.to_url().as_str()).unwrap();
    p_assert_eq!(addr, addr2);

    let org_addr = addr.generate_organization_addr(verify_key.clone());
    p_assert_eq!(org_addr.root_verify_key(), &verify_key);
    p_assert_eq!(org_addr.hostname(), addr.hostname());
    p_assert_eq!(org_addr.port(), addr.port());
    p_assert_eq!(org_addr.use_ssl(), addr.use_ssl());
    p_assert_eq!(org_addr.organization_id(), addr.organization_id());
}

#[rstest]
#[case::empty("", AddrError::InvalidUrl(url::ParseError::RelativeUrlWithoutBase))]
#[case::invalid_url("foo", AddrError::InvalidUrl(url::ParseError::RelativeUrlWithoutBase))]
#[case::bad_scheme("xxx://foo:42/org?a=bootstrap_organization&p=wA", AddrError::InvalidUrlScheme{ expected: "parsec3" })]
// cspell:disable-next-line
#[case::missing_action("parsec3://foo:42/org?p=wA", AddrError::MissingParam("a"))]
#[case::missing_payload(
    "parsec3://foo:42/org?a=bootstrap_organization",
    AddrError::MissingParam("p")
)]
#[case::bad_action(
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=dummy&p=wA",
    AddrError::InvalidParamValue {
        param: "a",
        help: "Expected `a=bootstrap_organization`".to_string()
    }
)]
#[case::payload_not_b64(
    "parsec3://foo:42/org?a=bootstrap_organization&p=dummy",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_not_msgpack(
    // Base64(b"dummy")
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=bootstrap_organization&p=ZHVtbXk",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_empty_token(
    // Base64(Msgpack(b""))
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=bootstrap_organization&p=xAA",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_bad_token(
    // Base64(Msgpack(b"\x00"))
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=bootstrap_organization&p=xAEA",
    AddrError::InvalidParamValue {
        param: "p",
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::missing_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42?a=bootstrap_organization&p=wA",
    AddrError::InvalidOrganizationID
)]
#[case::missing_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42/?a=bootstrap_organization&p=wA",
    AddrError::InvalidOrganizationID
)]
#[case::bad_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42/bad/org?a=bootstrap_organization&p=wA",
    AddrError::InvalidOrganizationID
)]
#[case::bad_org_name(
    // cspell:disable-next-line
    "parsec3://foo:42/~org?a=bootstrap_organization&p=wA",
    AddrError::InvalidOrganizationID
)]
fn organization_bootstrap_addr_bad_value(#[case] url: &str, #[case] msg: AddrError) {
    p_assert_eq!(
        ParsecOrganizationBootstrapAddr::from_str(url).unwrap_err(),
        msg
    );
}

#[rstest]
#[case::bootstrap_organization(
    // cspell:disable-next-line
    "parsec3://parsec.example.com/my_org?a=bootstrap_organization&p=xBCgAAAAAAAAAAAAAAAAAAAB",
    // cspell:disable-next-line
    "https://parsec.example.com/redirect/my_org?a=bootstrap_organization&p=xBCgAAAAAAAAAAAAAAAAAAAB",
)]
#[case::path(
    // cspell:disable-next-line
    "parsec3://parsec.example.com/my_org?a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",
    // cspell:disable-next-line
    "https://parsec.example.com/redirect/my_org?a=path&p=k9gCLU3tEnQGRgiDO39X8BFW4gHcADTM4WfM1MzhzNnMvTPMq8y-BnrM-8yiDcyvdlvMv2wjzIskB8zZWi4yFwRtzMxAzIDM0iPMnX8czKY7Pm3M5szoODd-NiI8U3A",
)]
#[case::claim_user(
    // cspell:disable-next-line
    "parsec3://parsec.example.com/my_org?a=claim_user&p=xBCgAAAAAAAAAAAAAAAAAAAB",
    // cspell:disable-next-line
    "https://parsec.example.com/redirect/my_org?a=claim_user&p=xBCgAAAAAAAAAAAAAAAAAAAB"
)]
#[case::pki_enrollment(
    "parsec3://parsec.example.com/my_org?a=pki_enrollment",
    "https://parsec.example.com/redirect/my_org?a=pki_enrollment"
)]
#[case::async_enrollment(
    "parsec3://parsec.example.com/my_org?a=async_enrollment",
    "https://parsec.example.com/redirect/my_org?a=async_enrollment"
)]
fn action_addr_good(#[case] url: &str, #[case] redirect_url: &str) {
    let addr: ParsecActionAddr = url.parse().unwrap();

    let addr2 = ParsecActionAddr::from_http_redirection(redirect_url).unwrap();
    p_assert_eq!(addr2, addr);

    let addr3 = ParsecActionAddr::from_any(url).unwrap();
    p_assert_eq!(addr3, addr);

    let addr4 = ParsecActionAddr::from_any(redirect_url).unwrap();
    p_assert_eq!(addr4, addr);
}

#[rstest]
#[case::empty("")]
#[case::dummy("dummy")]
// cspell:disable-next-line
#[case::bad_scheme("xxx://parsec.example.com/my_org?a=claim_user&p=xBCgAAAAAAAAAAAAAAAAAAAB")]
#[case::bad_redirect("https://parsec.example.com/redir/my_org?a=pki_enrollment")]
#[case::missing_param("parsec3://parsec.example.com/my_org?a=claim_user")]
fn action_addr_from_any_bad(#[case] url: &str) {
    let err = ParsecActionAddr::from_any(url);
    assert!(err.is_err());
}

#[test]
fn comparison_with_implicit_port() {
    let https_implicit_parse: ParsecAddr = "parsec3://parsec.example.com".parse().unwrap();
    let https_explicit_parse: ParsecAddr = "parsec3://parsec.example.com:443".parse().unwrap();
    let https_implicit_new: ParsecAddr =
        ParsecAddr::new("parsec.example.com".to_string(), None, true);
    let https_explicit_new: ParsecAddr =
        ParsecAddr::new("parsec.example.com".to_string(), Some(443), true);

    p_assert_eq!(https_implicit_parse, https_explicit_parse);
    p_assert_eq!(https_implicit_parse, https_implicit_new);
    p_assert_eq!(https_implicit_parse, https_explicit_new);

    let http_implicit_parse: ParsecAddr =
        "parsec3://parsec.example.com?no_ssl=true".parse().unwrap();
    let http_explicit_parse: ParsecAddr = "parsec3://parsec.example.com:80?no_ssl=true"
        .parse()
        .unwrap();
    let http_implicit_new: ParsecAddr =
        ParsecAddr::new("parsec.example.com".to_string(), None, false);
    let http_explicit_new: ParsecAddr =
        ParsecAddr::new("parsec.example.com".to_string(), Some(80), false);

    p_assert_eq!(http_implicit_parse, http_explicit_parse);
    p_assert_eq!(http_implicit_parse, http_implicit_new);
    p_assert_eq!(http_implicit_parse, http_explicit_new);

    p_assert_ne!(http_implicit_parse, https_implicit_parse);
}

#[rstest]
#[case::simple("parsec3://parsec.example.com".parse().unwrap(), "https://parsec.example.com/")]
#[case::with_port("parsec3://parsec.example.com:888".parse().unwrap(), "https://parsec.example.com:888/")]
#[case::explicit_default_http_port("parsec3://parsec.example.com:80?no_ssl=true".parse().unwrap(), "http://parsec.example.com/")]
#[case::explicit_default_https_port("parsec3://parsec.example.com:443".parse().unwrap(), "https://parsec.example.com/")]
#[case::no_ssl("parsec3://parsec.example.com?no_ssl=true".parse().unwrap(), "http://parsec.example.com/")]
#[case::yes_ssl("parsec3://parsec.example.com?no_ssl=false".parse().unwrap(), "https://parsec.example.com/")]
fn server_addr_serialize_ok(#[case] addr: ParsecAddr, #[case] serialized: &'static str) {
    serde_test::assert_tokens(&addr, &[serde_test::Token::BorrowedStr(serialized)]);
}

#[test]
fn server_addr_deserialize_no_trailing_slash_ok() {
    // We normally serialize the URL with a trailing slash (e.g. `parsec3://a.b` -> `https://a.b/`),
    // however we also accept without trailing slash during deserialization.
    let serialized_without_trailing_slash = "https://parsec.example.com";
    let expected: ParsecAddr = "parsec3://parsec.example.com".parse().unwrap();
    serde_test::assert_de_tokens(
        &expected,
        &[serde_test::Token::BorrowedStr(
            serialized_without_trailing_slash,
        )],
    );
}

#[rstest]
#[case::empty("")]
#[case::no_scheme("parsec.example.com")]
#[case::bad_port("parsec.example.com:0")]
#[case::unknown_scheme("dummy://parsec.example.com")]
#[case::parsec3_scheme_not_allowed("parsec3://parsec.example.com")]
#[case::with_path("parsec3://parsec.example.com/CoolOrg")]
#[case::with_params("parsec3://parsec.example.com/?foo=bar")]
fn server_addr_serialize_ko(#[case] bad: &'static str) {
    serde_test::assert_de_tokens_error::<ParsecAddr>(
        &[serde_test::Token::Str(bad)],
        "Invalid server URL",
    );
}

#[rstest]
fn pki_enrollment_addr_bad_type(#[values(Some("dummy"), Some(""), None)] bad_type: Option<&str>) {
    let testbed = PKIEnrollmentAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace("pki_enrollment", bad_type);
            testbed.assert_addr_err(
                &url,
                AddrError::InvalidParamValue {
                    param: "a",
                    help: "Expected `a=pki_enrollment`".to_string(),
                },
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace("a=pki_enrollment", "");
            testbed.assert_addr_err(&url, AddrError::MissingParam("a"));
        }
    }
}

#[rstest]
fn async_enrollment_addr_bad_type(#[values(Some("dummy"), Some(""), None)] bad_type: Option<&str>) {
    let testbed = AsyncEnrollmentAddrTestbed {};

    match bad_type {
        Some(bad_type) => {
            // Type param present in the url but with bad and empty value
            let url = testbed.url().replace("async_enrollment", bad_type);
            testbed.assert_addr_err(
                &url,
                AddrError::InvalidParamValue {
                    param: "a",
                    help: "Expected `a=async_enrollment`".to_string(),
                },
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed.url().replace("a=async_enrollment", "");
            testbed.assert_addr_err(&url, AddrError::MissingParam("a"));
        }
    }
}

macro_rules! test_anonymous_addr {
    ($testbed:expr, $addr_ty:ty) => {
        let testbed = $testbed;
        let addr: $addr_ty = testbed.url().parse().unwrap();
        let anonymous_addr: ParsecAnonymousAddr = addr.into();
        assert_eq!(
            anonymous_addr.to_anonymous_http_url(),
            format!("https://{DOMAIN}/anonymous/{ORG}").parse().unwrap()
        );
        assert_eq!(anonymous_addr.organization_id(), &ORG.parse().unwrap(),);
        let base: ParsecAddr = anonymous_addr.into();
        assert_eq!(base, format!("{PARSEC_SCHEME}://{DOMAIN}").parse().unwrap(),);
    };
}

#[rstest]
fn anonymous_addr() {
    test_anonymous_addr!(
        OrganizationBootstrapAddrTestbed {},
        ParsecOrganizationBootstrapAddr
    );
    test_anonymous_addr!(PKIEnrollmentAddrTestbed {}, ParsecPkiEnrollmentAddr);
    test_anonymous_addr!(AsyncEnrollmentAddrTestbed {}, ParsecAsyncEnrollmentAddr);
}
