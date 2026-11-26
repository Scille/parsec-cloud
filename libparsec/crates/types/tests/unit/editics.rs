// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::num::NonZeroUsize;

use libparsec_tests_lite::prelude::*;

use super::*;

#[rstest]
#[case::basic(
    OrganizationID::try_from("Corp").unwrap(),
    VlobID::from_hex("9eb55180-3ac8-4b06-bdef-4d382f028044").unwrap(),
    VlobID::from_hex("d5cfaa28-10c8-4e18-a2b7-8b3e609e0d45").unwrap(),
    &[0, 0, 0, 0, 4, 29, 100]
)]
#[case::non_ascii_org_id(
    OrganizationID::try_from("黑衣社").unwrap(),
    VlobID::from_hex("69860677-f056-492d-bb8e-8c3d70fc74ea").unwrap(),
    VlobID::from_hex("56d7ea6d-ad9d-4f1e-bcc2-7bd57aa36bf2").unwrap(),
    &[0, 0, 0, 2, 2, 84, 330]
)]
#[case::extreme_uuids(
    OrganizationID::try_from("OrgWithAMuchLongerName").unwrap(),
    VlobID::from_hex("00000000-0000-0000-0000-000000000001").unwrap(),
    VlobID::from_hex("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap(),
    &[0, 0, 2, 0, 4, 61, 148]
)]
fn sharding(
    #[case] organization_id: OrganizationID,
    #[case] realm_id: VlobID,
    #[case] document_id: VlobID,
    #[case] expected: &[usize],
) {
    for (servers, expected_index) in [1usize, 2, 3, 4, 8, 97, 1000]
        .into_iter()
        .map(|n| NonZeroUsize::new(n).unwrap())
        .zip(expected)
    {
        p_assert_eq!(
            editics_compute_sharding_index(servers, &organization_id, realm_id, document_id),
            *expected_index
        );
    }
}
