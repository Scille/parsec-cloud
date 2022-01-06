// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use pretty_assertions::assert_eq;
use serde_test::{assert_tokens, Token};

use parsec_api_types::*;

#[test]
fn test_addr_good() {
    macro_rules! test_addr_good {
        ($addr_type:ident, $url:expr) => {
            test_addr_good_with_expected!($addr_type, $url, $url);
        };
    }
    macro_rules! test_addr_good_with_expected {
        ($addr_type:ident, $url:expr, $expected_url:expr) => {
            let addr = $url.parse::<$addr_type>().unwrap();
            // Test identity
            assert_eq!(addr.to_url().as_str(), $expected_url);
            // Test serde
            assert_tokens(&addr, &[Token::Str($expected_url)]);
        };
    }

    test_addr_good!(BackendAddr, "parsec://example.com");
    test_addr_good_with_expected!(BackendAddr, "parsec://example.com/", "parsec://example.com");
    test_addr_good!(BackendOrganizationAddr, "parsec://parsec.example.com/MyOrg?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss");
    test_addr_good!(
        BackendOrganizationBootstrapAddr,
        "parsec://parsec.example.com/my_org?action=bootstrap_organization&token=1234ABCD"
    );
    test_addr_good!(
        BackendOrganizationFileLinkAddr,
        "parsec://parsec.example.com/my_org?action=file_link&workspace_id=3a50b191122b480ebb113b10216ef343&path=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss"
    );
    test_addr_good!(BackendInvitationAddr, "parsec://parsec.example.com/my_org?action=claim_user&token=3a50b191122b480ebb113b10216ef343");
}

#[test]
fn test_addr_with_unicode_organization_id() {
    let addr: BackendOrganizationAddr = "parsec://parsec.example.com/%E5%BA%B7%E7%86%99%E5%B8%9D?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss".parse().unwrap();
    assert_eq!(
        addr.organization_id(),
        &"康熙帝".parse::<OrganizationID>().unwrap()
    );
}

#[test]
fn test_addr_with_bad_unicode_organization_id() {
    // Not a valid percent-encoded utf8 string
    let ret = "parsec://parsec.example.com/%E5%BA%B7%E7?rvk=7NFDS4VQLP3XPCMTSEN34ZOXKGGIMTY2W2JI2SPIHB2P3M6K4YWAssss".parse::<BackendOrganizationAddr>();
    assert!(ret.is_err());
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
