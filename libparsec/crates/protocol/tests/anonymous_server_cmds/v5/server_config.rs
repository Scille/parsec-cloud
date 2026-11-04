// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::anonymous_server_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.5.3-a.0+dev
    // Content:
    //   cmd: 'server_config'
    let raw: &[u8] = hex!("81a3636d64ad7365727665725f636f6e666967").as_ref();
    let req = anonymous_server_cmds::server_config::Req;

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = anonymous_server_cmds::AnyCmdReq::ServerConfig(req);
    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::ServerConfig(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Legacy format from Parsec < 3.9, missing `cryptpad` & `advisory_device_file_protection` fields
            //
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "84a6737461747573a26f6ba76163636f756e74a844495341424c4544a76f70656e6261"
                "6f81a474797065a844495341424c4544b66f7267616e697a6174696f6e5f626f6f7473"
                "74726170b4574954485f424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                cryptpad: libparsec_types::Maybe::Absent,
                advisory_device_file_protection: libparsec_types::Maybe::Absent,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Legacy format from Parsec < 3.8, missing the `transit_mount_path` field
            // Legacy format from Parsec < 3.9, missing `cryptpad` & `advisory_device_file_protection` fields
            // Legacy format from API < 5.7, missing `name`, `help_link` & `icon` fields in `auths`
            //
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'ENABLED_WITH_VAULT'
            //   openbao: {
            //     type: 'ENABLED',
            //     auths: [
            //       { id: 'HEXAGONE', mount_path: 'auth/hexagone', },
            //       { id: 'PRO_CONNECT', mount_path: 'auth/pro_connect', },
            //     ],
            //     secret: { type: 'KV2', mount_path: 'secrets', },
            //     server_url: 'https://openbao.parsec.invalid',
            //   }
            //   organization_bootstrap: 'SPONTANEOUS'
            hex!(
                "84a6737461747573a26f6ba76163636f756e74b2454e41424c45445f574954485f5641"
                "554c54a76f70656e62616f84a474797065a7454e41424c4544a561757468739282a269"
                "64a848455841474f4e45aa6d6f756e745f70617468ad617574682f68657861676f6e65"
                "82a26964ab50524f5f434f4e4e454354aa6d6f756e745f70617468b0617574682f7072"
                "6f5f636f6e6e656374a673656372657482a474797065a34b5632aa6d6f756e745f7061"
                "7468a773656372657473aa7365727665725f75726cbe68747470733a2f2f6f70656e62"
                "616f2e7061727365632e696e76616c6964b66f7267616e697a6174696f6e5f626f6f74"
                "7374726170ab53504f4e54414e454f5553"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::EnabledWithVault,
                cryptpad: libparsec_types::Maybe::Absent,
                advisory_device_file_protection: libparsec_types::Maybe::Absent,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::Spontaneous,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Enabled {
                    server_url: "https://openbao.parsec.invalid".to_string(),
                    secret: anonymous_server_cmds::server_config::OpenBaoSecretConfig::KV2 {
                        mount_path: "secrets".to_string(),
                    },
                    transit_mount_path: libparsec_types::Maybe::Absent,
                    auths: vec![
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "HEXAGONE".to_string(),
                            mount_path: "auth/hexagone".to_string(),
                            name: libparsec_types::Maybe::Absent,
                            help_link: libparsec_types::Maybe::Absent,
                            icon: libparsec_types::Maybe::Absent,
                        },
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "PRO_CONNECT".to_string(),
                            mount_path: "auth/pro_connect".to_string(),
                            name: libparsec_types::Maybe::Absent,
                            help_link: libparsec_types::Maybe::Absent,
                            icon: libparsec_types::Maybe::Absent,
                        }
                    ]
                },
            }
        ),
        (
            // Legacy format from Parsec < 3.9, missing `cryptpad` & `advisory_device_file_protection` fields
            // Legacy format from API < 5.7, missing `name`, `help_link` & `icon` fields in `auths`
            //
            // Generated from Parsec 3.7.2-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'ENABLED_WITH_VAULT'
            //   openbao: {
            //     type: 'ENABLED',
            //     auths: [
            //       { id: 'HEXAGONE', mount_path: 'auth/hexagone', },
            //       { id: 'PRO_CONNECT', mount_path: 'auth/pro_connect', },
            //     ],
            //     secret: { type: 'KV2', mount_path: 'secrets', },
            //     server_url: 'https://openbao.parsec.invalid',
            //     transit_mount_path: 'transit',
            //   }
            //   organization_bootstrap: 'SPONTANEOUS'
            hex!(
                "84a6737461747573a26f6ba76163636f756e74b2454e41424c45445f574954485f5641"
                "554c54a76f70656e62616f85a474797065a7454e41424c4544a561757468739282a269"
                "64a848455841474f4e45aa6d6f756e745f70617468ad617574682f68657861676f6e65"
                "82a26964ab50524f5f434f4e4e454354aa6d6f756e745f70617468b0617574682f7072"
                "6f5f636f6e6e656374a673656372657482a474797065a34b5632aa6d6f756e745f7061"
                "7468a773656372657473aa7365727665725f75726cbe68747470733a2f2f6f70656e62"
                "616f2e7061727365632e696e76616c6964b27472616e7369745f6d6f756e745f706174"
                "68a77472616e736974b66f7267616e697a6174696f6e5f626f6f747374726170ab5350"
                "4f4e54414e454f5553"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::EnabledWithVault,
                cryptpad: libparsec_types::Maybe::Absent,
                advisory_device_file_protection: libparsec_types::Maybe::Absent,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::Spontaneous,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Enabled {
                    server_url: "https://openbao.parsec.invalid".to_string(),
                    secret: anonymous_server_cmds::server_config::OpenBaoSecretConfig::KV2 {
                        mount_path: "secrets".to_string(),
                    },
                    transit_mount_path: libparsec_types::Maybe::Present("transit".to_string()),
                    auths: vec![
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "HEXAGONE".to_string(),
                            mount_path: "auth/hexagone".to_string(),
                            name: libparsec_types::Maybe::Absent,
                            help_link: libparsec_types::Maybe::Absent,
                            icon: libparsec_types::Maybe::Absent,
                        },
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "PRO_CONNECT".to_string(),
                            mount_path: "auth/pro_connect".to_string(),
                            name: libparsec_types::Maybe::Absent,
                            help_link: libparsec_types::Maybe::Absent,
                            icon: libparsec_types::Maybe::Absent,
                        }
                    ]
                },
            }
        ),
        (
            // Legacy format from Parsec < 3.9, missing `cryptpad` & `advisory_device_file_protection` fields
            //
            // Generated from Parsec 3.8.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   cryptpad: { type: 'DISABLED', }
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "85a6737461747573a26f6ba76163636f756e74a844495341424c4544a8637279707470"
                "616481a474797065a844495341424c4544a76f70656e62616f81a474797065a8444953"
                "41424c4544b66f7267616e697a6174696f6e5f626f6f747374726170b4574954485f42"
                "4f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                cryptpad: libparsec_types::Maybe::Present(anonymous_server_cmds::server_config::CryptPadConfig::Disabled),
                advisory_device_file_protection: libparsec_types::Maybe::Absent,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Legacy format from Parsec < 3.9, missing `cryptpad` & `advisory_device_file_protection` fields
            //
            // Generated from Parsec 3.8.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   cryptpad: { type: 'ENABLED', server_url: 'https://cryptpad.parsec.invalid', }
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "85a6737461747573a26f6ba76163636f756e74a844495341424c4544a8637279707470"
                "616482a474797065a7454e41424c4544aa7365727665725f75726cbf68747470733a2f"
                "2f63727970747061642e7061727365632e696e76616c6964a76f70656e62616f81a474"
                "797065a844495341424c4544b66f7267616e697a6174696f6e5f626f6f747374726170"
                "b4574954485f424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                cryptpad: libparsec_types::Maybe::Present(anonymous_server_cmds::server_config::CryptPadConfig::Enabled {
                    server_url: "https://cryptpad.parsec.invalid".to_string(),
                }),
                advisory_device_file_protection: libparsec_types::Maybe::Absent,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Generated from Parsec 3.8.2-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   advisory_device_file_protection: [ ]
            //   cryptpad: { type: 'DISABLED', }
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "86a6737461747573a26f6ba76163636f756e74a844495341424c4544bf61647669736f"
                "72795f6465766963655f66696c655f70726f74656374696f6e90a86372797074706164"
                "81a474797065a844495341424c4544a76f70656e62616f81a474797065a84449534142"
                "4c4544b66f7267616e697a6174696f6e5f626f6f747374726170b4574954485f424f4f"
                "5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                cryptpad: libparsec_types::Maybe::Present(anonymous_server_cmds::server_config::CryptPadConfig::Disabled),
                advisory_device_file_protection: libparsec_types::Maybe::Present(vec![]),
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Generated from Parsec 3.8.2-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   advisory_device_file_protection: [ 'PASSWORD+TOTP', 'PKI', ]
            //   cryptpad: { type: 'DISABLED', }
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "86a6737461747573a26f6ba76163636f756e74a844495341424c4544bf61647669736f"
                "72795f6465766963655f66696c655f70726f74656374696f6e92ad50415353574f5244"
                "2b544f5450a3504b49a8637279707470616481a474797065a844495341424c4544a76f"
                "70656e62616f81a474797065a844495341424c4544b66f7267616e697a6174696f6e5f"
                "626f6f747374726170b4574954485f424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                cryptpad: libparsec_types::Maybe::Present(anonymous_server_cmds::server_config::CryptPadConfig::Disabled),
                advisory_device_file_protection: libparsec_types::Maybe::Present(vec![
                    "PASSWORD+TOTP".to_string(),
                    "PKI".to_string(),
                ]),
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Generated from Parsec 3.10.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   advisory_device_file_protection: [ ]
            //   cryptpad: { type: 'DISABLED', }
            //   openbao: {
            //     type: 'ENABLED',
            //     auths: [
            //       {
            //         help_link: 'https://proconnect.parsec.invalid/help',
            //         icon: 'https://proconnect.parsec.invalid/icon.svg',
            //         id: 'PRO_CONNECT',
            //         mount_path: 'auth/pro_connect',
            //         name: 'ProConnect',
            //       },
            //       {
            //         help_link: None,
            //         icon: None,
            //         id: 'ACME',
            //         mount_path: 'auth/acme',
            //         name: 'Acme Corp SSO',
            //       },
            //     ],
            //     secret: { type: 'KV2', mount_path: 'secrets', },
            //     server_url: 'https://openbao.parsec.invalid',
            //     transit_mount_path: 'transit',
            //   }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "86a6737461747573a26f6ba76163636f756e74a844495341424c4544bf61647669736f"
                "72795f6465766963655f66696c655f70726f74656374696f6e90a86372797074706164"
                "81a474797065a844495341424c4544a76f70656e62616f85a474797065a7454e41424c"
                "4544a561757468739285a968656c705f6c696e6bd92668747470733a2f2f70726f636f"
                "6e6e6563742e7061727365632e696e76616c69642f68656c70a469636f6ed92a687474"
                "70733a2f2f70726f636f6e6e6563742e7061727365632e696e76616c69642f69636f6e"
                "2e737667a26964ab50524f5f434f4e4e454354aa6d6f756e745f70617468b061757468"
                "2f70726f5f636f6e6e656374a46e616d65aa50726f436f6e6e65637485a968656c705f"
                "6c696e6bc0a469636f6ec0a26964a441434d45aa6d6f756e745f70617468a961757468"
                "2f61636d65a46e616d65ad41636d6520436f72702053534fa673656372657482a47479"
                "7065a34b5632aa6d6f756e745f70617468a773656372657473aa7365727665725f7572"
                "6cbe68747470733a2f2f6f70656e62616f2e7061727365632e696e76616c6964b27472"
                "616e7369745f6d6f756e745f70617468a77472616e736974b66f7267616e697a617469"
                "6f6e5f626f6f747374726170b4574954485f424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                cryptpad: libparsec_types::Maybe::Present(anonymous_server_cmds::server_config::CryptPadConfig::Disabled),
                advisory_device_file_protection: libparsec_types::Maybe::Present(vec![]),
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Enabled {
                    server_url: "https://openbao.parsec.invalid".to_string(),
                    secret: anonymous_server_cmds::server_config::OpenBaoSecretConfig::KV2 {
                        mount_path: "secrets".to_string(),
                    },
                    transit_mount_path: libparsec_types::Maybe::Present("transit".to_string()),
                    auths: vec![
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "PRO_CONNECT".to_string(),
                            mount_path: "auth/pro_connect".to_string(),
                            name: libparsec_types::Maybe::Present("ProConnect".to_string()),
                            help_link: libparsec_types::Maybe::Present(Some("https://proconnect.parsec.invalid/help".to_string())),
                            icon: libparsec_types::Maybe::Present(Some("https://proconnect.parsec.invalid/icon.svg".to_string())),
                        },
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "ACME".to_string(),
                            mount_path: "auth/acme".to_string(),
                            name: libparsec_types::Maybe::Present("Acme Corp SSO".to_string()),
                            help_link: libparsec_types::Maybe::Present(None),
                            icon: libparsec_types::Maybe::Present(None),
                        }
                    ]
                },
            }
        )
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());

        let data = anonymous_server_cmds::server_config::Rep::load(raw).unwrap();
        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = anonymous_server_cmds::server_config::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}
