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
