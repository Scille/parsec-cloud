// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;

/* cSpell:disable */
/// TODO: Remove me once wasm doesn't need to mock
pub(crate) const _DEVICE: &str = "\
    iKpjaXBoZXJ0ZXh0xQIfhNS2QECtYYJ5y9JNoYLCP9a+wTUtposlJfl2fGZWJ0elnpNtTElz4Z\
    1odyuuMuuZmBdpjLOCPObima9F1MY++i5OBm3NzfaGrFTg+I30hTKNvHDsmpcusD38Pvw1RO8U\
    yJJwkUw6ZvezjKsaEAWo9CkiQca3CJRqJMkoEujYJdVBWrq/ANdyLqGMDxRUUf/9bD8U9WQCTL\
    q21rcJ2J9wQ0Y/K+6LVTkTl67YZEwpaLCG82e4jTpwMJ2J3F3r23Ri2QAltB6HuBffE1XzLTmj\
    0VGikvY4QFugMLiwF+XOMIHjBucIi7zxGlSFkNmKlYf7wfdz15FZCTAyIayD2nxik4KgfIJE2c\
    BAHDU1HV0I1FvJTRw8j+hIvPLZO6ZQ+I5FymyOoQvLNmkmfbp5mvftt5ybrkqO13ywMoKkFSFt\
    8XaCqPSoC1Cc6MbO/q0DN8GUtGRQEKHQ40toMbZKxUzA1cuf6yRizJutvmQtTWHpjAE+U0BI5I\
    0q/HmJ6rmastjx76DdMdJyBmX/sgQkCjY1IweD0Hthnn1yYbSaIAtbn75eLpNjVFATJzpYIyTl\
    1Pddtea6nW1VW0vUj4m9+++GQUS8NO5ZQqZBKYZ4bfZN2mEn8a6k6A+nd3CTh3+mGpqn1l391m\
    rZwTCf8u35Was0zkXGRFfRKTFcSmS/XVMrQanwjTR0ZO19LWT2ZX+qsjSsQUXU/GKrG1DhXLt4\
    7/qgqWRldmljZV9pZNlBODZiNjJjN2U5MjcxNDBjYzkwMzQ2MDVlNjBhYjdlN2JAOTdlYjY0OG\
    VkZjczNGE5MDk3NTRiMzVmYjJhNTBjNjCsZGV2aWNlX2xhYmVsrUFsaWNlIE1hY2hpbmWsaHVt\
    YW5faGFuZGxlkqxhbGljZUBkZXYuZnKlYWxpY2Wvb3JnYW5pemF0aW9uX2lkp1Rlc3RXZWKkc2\
    FsdMQQ5TAtOOkDAnTdiN8Vg6bhwKRzbHVn2VQyYTc3YzY5YmM4I1Rlc3RXZWIjODZiNjJjN2U5\
    MjcxNDBjYzkwMzQ2MDVlNjBhYjdlN2JAOTdlYjY0OGVkZjczNGE5MDk3NTRiMzVmYjJhNTBjNj\
    CkdHlwZahwYXNzd29yZA==\
    ";
/* cSpell:enable */

#[test]
fn test_device_wasm() {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use libparsec_client_types::{AvailableDevice, DeviceFile, DeviceFileType, StrPath};

    let devices = _DEVICE
        .split(':')
        .filter_map(|x| STANDARD.decode(x).ok())
        .filter_map(|x| DeviceFile::load(&x).ok())
        .map(|x| match x {
            DeviceFile::Password(device) => (
                DeviceFileType::Password,
                device.organization_id,
                device.device_id,
                device.human_handle,
                device.device_label,
                // There are no legacy device
                device.slug.unwrap(),
            ),
            DeviceFile::Recovery(device) => (
                DeviceFileType::Recovery,
                device.organization_id,
                device.device_id,
                device.human_handle,
                device.device_label,
                device.slug,
            ),
            DeviceFile::Smartcard(device) => (
                DeviceFileType::Smartcard,
                device.organization_id,
                device.device_id,
                device.human_handle,
                device.device_label,
                device.slug,
            ),
        })
        .map(
            |(ty, organization_id, device_id, human_handle, device_label, slug)| AvailableDevice {
                key_file_path: StrPath::from(""),
                organization_id,
                device_id,
                human_handle,
                device_label,
                slug,
                ty,
            },
        )
        .collect::<Vec<_>>();

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0], AvailableDevice {
        key_file_path: StrPath::from(""),
        organization_id: "TestWeb".parse().unwrap(),
        device_id: "86b62c7e927140cc9034605e60ab7e7b@97eb648edf734a909754b35fb2a50c60".parse().unwrap(),
        human_handle: Some("alice <alice@dev.fr>".parse().unwrap()),
        device_label: Some("Alice Machine".parse().unwrap()),
        slug: "2a77c69bc8#TestWeb#86b62c7e927140cc9034605e60ab7e7b@97eb648edf734a909754b35fb2a50c60".to_string(),
        ty: DeviceFileType::Password,
    });
}
