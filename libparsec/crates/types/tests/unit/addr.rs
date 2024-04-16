// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use rstest_reuse::{self, apply, template};
use serde_test::{assert_tokens, Token};
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

            // Test serde
            // `Token::Str` requires `&'static str` but we have a regular `&str` here...
            // the solution is to explicitly leak the memory *shocked*
            let static_expected_url = Box::leak(expected_url.to_string().into_boxed_str());
            assert_tokens(&addr, &[Token::Str(static_expected_url)]);
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

#[template]
#[rstest(
    testbed,
    case::server_addr(&AddrTestbed{}),
    case::organization_addr(&OrganizationAddrTestbed{}),
    case::organization_bootstrap_addr(&OrganizationBootstrapAddrTestbed{}),
    case::workspace_path_addr(&WorkspacePathAddrTestbed{}),
    case::invitation_addr(&InvitationAddrTestbed{}),
)]
fn all_addr(testbed: &dyn Testbed) {}

#[template]
#[rstest(
    testbed,
    case::organization_addr(&OrganizationAddrTestbed{}),
    case::organization_bootstrap_addr(&OrganizationBootstrapAddrTestbed{}),
    case::workspace_path_addr(&WorkspacePathAddrTestbed{}),
    case::invitation_addr(&InvitationAddrTestbed{}),
)]
fn addr_with_org(testbed: &dyn Testbed) {}

/*
 * Actual tests
 */

#[rstest_reuse::apply(all_addr)]
fn good_addr_base(testbed: &dyn Testbed) {
    testbed.assert_addr_ok(&testbed.url());
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
fn parsec_addr_to_http_domain_url(value: &str, path: Option<&str>, expected: &str) {
    let addr: ParsecAddr = value.parse().unwrap();
    let result = addr.to_http_url_with_path(path);
    p_assert_eq!(result.as_str(), expected);
}

#[apply(all_addr)]
fn good_addr_with_port(testbed: &dyn Testbed) {
    let url = testbed.url().replace(DOMAIN, "example.com:4242");
    testbed.assert_addr_ok(&url);
}

#[apply(all_addr)]
fn addr_with_bad_port(testbed: &dyn Testbed, #[values("NaN", "999999")] bad_port: &str) {
    let url = testbed
        .url()
        .replace(DOMAIN, &format!("{}:{}", DOMAIN, bad_port));
    testbed.assert_addr_err(
        &url,
        AddrError::InvalidUrl(url.clone(), url::ParseError::InvalidPort),
    );
}

#[apply(all_addr)]
fn addr_with_no_hostname(testbed: &dyn Testbed, #[values("", ":4242")] bad_domain: &str) {
    let (url, expected_error) = if bad_domain.is_empty() {
        // `http:///foo` is a valid url, so we also have to remove the path
        let url = match testbed.url().split('?').nth(1) {
            Some(extra) => format!("parsec3://?{}", extra),
            None => "parsec3://".to_string(),
        };
        let expected_error = AddrError::InvalidUrl(url.to_string(), url::ParseError::EmptyHost);
        (url, expected_error)
    } else {
        let url = testbed.url().replace(DOMAIN, bad_domain);
        let expected_error = AddrError::InvalidUrl(url.clone(), url::ParseError::EmptyHost);
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
        AddrError::InvalidUrl(url.to_string(), url::ParseError::InvalidIpv4Address),
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
    let url = testbed.url().replace(
        &format!("{}/{}", DOMAIN, ORG),
        &format!("{}{}", DOMAIN, bad_path),
    );
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
                    value: bad_type.to_string(),
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
    let expected_token = BootstrapToken::from_hex(TOKEN).unwrap();
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
            value: bad_payload.into(),
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
                    value: bad_type.to_string(),
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
            value: bad_payload.into(),
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
                    value: bad_type.to_string(),
                    help: "Expected `a=claim_user` or `a=claim_device`".to_string(),
                },
            );
        }
        None => {
            // Type param not present in the url
            let url = testbed
                .url()
                .replace(&format!("a={}&", INVITATION_TYPE), "");
            testbed.assert_addr_err(&url, AddrError::MissingParam("a"));
        }
    }
}

#[rstest]
fn invitation_addr_token() {
    let testbed = InvitationAddrTestbed {};
    let addr: ParsecInvitationAddr = testbed.url().parse().unwrap();
    let expected_token = InvitationToken::from_hex(TOKEN).unwrap();
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
            value: bad_payload.into(),
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
        .replace(ORG, &format!("redirect/{}", ORG));
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
        .replace(DOMAIN, &format!("{}/redirect", DOMAIN));
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
#[case("https://foo.bar/not_valid")]
#[case("http://1270.0.1/redirect")]
#[case("http://foo:99999/redirect")]
fn faulty_addr_redirection(#[case] raw_url: &str) {
    let res = ParsecAddr::from_http_redirection(raw_url);

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
#[case::empty("", AddrError::InvalidUrl("".to_string(), url::ParseError::RelativeUrlWithoutBase))]
#[case::invalid_url("foo", AddrError::InvalidUrl("foo".to_string(), url::ParseError::RelativeUrlWithoutBase))]
#[case::bad_scheme("xx://foo:42", AddrError::InvalidUrlScheme { got: "xx".to_string(), expected: PARSEC_SCHEME })]
#[case::path_not_allowed("parsec3://foo:42/dummy", AddrError::ShouldNotHaveAPath("https://foo:42/dummy".parse().unwrap()))]
#[case::bad_parsing_in_valid_param(
    "parsec3://foo:42?no_ssl",
    AddrError::InvalidParamValue {
         param: "no_ssl",
         value: "".to_string(),
         help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
    }
)]
#[case::missing_value_for_param(
    "parsec3://foo:42?no_ssl=",
    AddrError::InvalidParamValue {
         param: "no_ssl",
         value: "".to_string(),
         help: "Expected `no_ssl=true` or `no_ssl=false`".to_string(),
    }
)]
#[case::bad_value_for_param(
    "parsec3://foo:42?no_ssl=nop",
    AddrError::InvalidParamValue {
         param: "no_ssl",
         value: "nop".to_string(),
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
        value: "dummy".to_string(),
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_not_msgpack(
    // Base64(b"dummy")
    // cspell:disable-next-line
    "parsec3://foo:42/org?p=ZHVtbXk",
    AddrError::InvalidParamValue {
        param: "p",
        // cspell:disable-next-line
        value: "ZHVtbXk".to_string(),
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_empty_root_verify_key(
    // Base64(Msgpack(b""))
    // cspell:disable-next-line
    "parsec3://foo:42/org?p=xAA",
    AddrError::InvalidParamValue {
        param: "p",
        // cspell:disable-next-line
        value: "xAA".to_string(),
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_bad_root_verify_key(
    // Base64(Msgpack(b"\x00"))
    // cspell:disable-next-line
    "parsec3://foo:42/org?p=xAEA",
    AddrError::InvalidParamValue {
        param: "p",
        // cspell:disable-next-line
        value: "xAEA".to_string(),
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
    let token = BootstrapToken::from_hex(TOKEN).unwrap();
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
#[case::empty("", AddrError::InvalidUrl("".to_string(), url::ParseError::RelativeUrlWithoutBase))]
#[case::invalid_url("foo", AddrError::InvalidUrl("foo".to_string(), url::ParseError::RelativeUrlWithoutBase))]
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
        value: "dummy".to_string(),
        help: "Expected `a=bootstrap_organization`".to_string()
    }
)]
#[case::payload_not_b64(
    "parsec3://foo:42/org?a=bootstrap_organization&p=dummy",
    AddrError::InvalidParamValue {
        param: "p",
        value: "dummy".to_string(),
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_not_msgpack(
    // Base64(b"dummy")
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=bootstrap_organization&p=ZHVtbXk",
    AddrError::InvalidParamValue {
        param: "p",
        // cspell:disable-next-line
        value: "ZHVtbXk".to_string(),
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_empty_token(
    // Base64(Msgpack(b""))
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=bootstrap_organization&p=xAA",
    AddrError::InvalidParamValue {
        param: "p",
        // cspell:disable-next-line
        value: "xAA".to_string(),
        help: "Invalid `p` parameter".to_string()
    }
)]
#[case::payload_bad_token(
    // Base64(Msgpack(b"\x00"))
    // cspell:disable-next-line
    "parsec3://foo:42/org?a=bootstrap_organization&p=xAEA",
    AddrError::InvalidParamValue {
        param: "p",
        // cspell:disable-next-line
        value: "xAEA".to_string(),
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

#[test]
fn legacy_parsec_v2_server_addr() {
    const LEGACY_URL: &str = "parsec://example.com";
    let expected_error = AddrError::InvalidUrlScheme {
        got: "parsec".to_string(),
        expected: "parsec3",
    };

    // Simply no longer supported
    p_assert_matches!(
        ParsecAddr::from_str(LEGACY_URL),
        Err(e) if e == expected_error
    );

    serde_test::assert_de_tokens_error::<ParsecAddr>(
        &[Token::Str(LEGACY_URL)],
        &expected_error.to_string(),
    )
}

#[test]
fn legacy_parsec_v2_organization_addr() {
    // cspell:disable-next-line
    const LEGACY_URL: &str = "parsec://parsec.example.com/MyOrg?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss";

    // No longer supported...
    p_assert_matches!(
        ParsecOrganizationAddr::from_str(LEGACY_URL),
        Err(e) if e == AddrError::InvalidUrlScheme{ got: "parsec".to_string(), expected: "parsec3" }
    );

    // ...except in deserialization for backward compatibility (needed by `LocalDevice` schema)
    let expected_url: ParsecOrganizationAddr =
        // cspell:disable-next-line
        "parsec3://parsec.example.com/MyOrg?p=xCD7SjlysFv3d4mTkRu-ZddRjIZPGraSjUnoOHT9s8rmLA"
            .parse()
            .unwrap();
    serde_test::assert_de_tokens(&expected_url, &[Token::Str(LEGACY_URL)])
}

#[test]
fn legacy_parsec_v2_organization_bootstrap_addr() {
    const LEGACY_URL: &str =
        "parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD";
    let expected_error = AddrError::InvalidUrlScheme {
        got: "parsec".to_string(),
        expected: "parsec3",
    };

    // Simply no longer supported
    p_assert_matches!(
        ParsecOrganizationBootstrapAddr::from_str(LEGACY_URL),
        Err(e) if e == expected_error
    );

    serde_test::assert_de_tokens_error::<ParsecOrganizationBootstrapAddr>(
        &[Token::Str(LEGACY_URL)],
        &expected_error.to_string(),
    )
}

#[test]
fn legacy_parsec_v2_workspace_path_addr() {
    // cspell:disable-next-line
    const LEGACY_URL: &str = "parsec://parsec.example.com/my_org?action=path&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss";
    let expected_error = AddrError::InvalidUrlScheme {
        got: "parsec".to_string(),
        expected: "parsec3",
    };

    // Simply no longer supported
    p_assert_matches!(
        ParsecWorkspacePathAddr::from_str(LEGACY_URL),
        Err(e) if e == expected_error
    );

    serde_test::assert_de_tokens_error::<ParsecWorkspacePathAddr>(
        &[Token::Str(LEGACY_URL)],
        &expected_error.to_string(),
    )
}

#[test]
fn legacy_parsec_v2_invitation_addr() {
    const LEGACY_URL: &str = "parsec://parsec.example.com/my_org?action=claim_user&token=3a50b191122b480ebb113b10216ef343";
    let expected_error = AddrError::InvalidUrlScheme {
        got: "parsec".to_string(),
        expected: "parsec3",
    };

    // Simply no longer supported
    p_assert_matches!(
        ParsecInvitationAddr::from_str(LEGACY_URL),
        Err(e) if e == expected_error
    );

    serde_test::assert_de_tokens_error::<ParsecInvitationAddr>(
        &[Token::Str(LEGACY_URL)],
        &expected_error.to_string(),
    )
}

#[test]
fn legacy_parsec_v2_pki_enrollment_addr() {
    const LEGACY_URL: &str = "parsec://parsec.example.com/my_org?action=pki_enrollment";

    // No longer supported...
    p_assert_matches!(
        ParsecPkiEnrollmentAddr::from_str(LEGACY_URL),
        Err(e) if e == AddrError::InvalidUrlScheme{ got: "parsec".to_string(), expected: "parsec3" }
    );

    // ...except in deserialization for backward compatibility (needed by `LocalPendingEnrollment` schema)
    let expected_url: ParsecPkiEnrollmentAddr =
        "parsec3://parsec.example.com/my_org?a=pki_enrollment"
            .parse()
            .unwrap();
    serde_test::assert_de_tokens(&expected_url, &[Token::Str(LEGACY_URL)])
}
