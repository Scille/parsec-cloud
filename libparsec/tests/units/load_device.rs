// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use libparsec_platform_device_loader::{
    get_default_data_base_dir, AvailableDevice, AvailableDeviceType,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// Generated from Parsec Client (Parsec v3.8.1)
// Content:
//   type: 'pki'
//   created_on: ext(1, 1778579885248564) i.e. 2026-05-12T11:58:05.248564Z
//   protected_on: ext(1, 1778579885248566) i.e. 2026-05-12T11:58:05.248566Z
//   server_url: 'https://saas-demo-v3-mightyfairy.parsec.cloud/'
//   organization_id: 'TestPKI'
//   user_id: ext(2, 0xb6d02d1b22494a9d97839191ce835765)
//   device_id: ext(2, 0xc91f53597b0a4c88860de825ef00685f)
//   human_handle: [ 'foo@example.com', 'foo', ]
//   device_label: 'Windows'
//   certificate_ref: {
//       uris: [
//         {
//           windowscng: {
//             issuer:
//               0x302d3116301406035504030c0d426c61636b204d65736120434131133011060355040a0c0a426c
//               61636b204d657361,
//             serial_number: 0x149e3eda0553d3e5063ca8b71a8220292b4daa5d,
//           },
//         },
//       ],
//       hash: 'sha256-FH45Rn8sFI5XTBxE1inkRUxeVmBA9jtsdSkY+6w3+gQ=',
//     }
//   algorithm: 'RSAES-OAEP-SHA256'
//   encrypted_key:
//       0x8499241ebd78d1ee30f94cdff31bd63ce0d6a84978907705857752e49925902e301150f5a5aa78
//       43eee4edf4b0b278f0c3e8a10fdcc2e0af17f1c35705f5a8845ce22da8ea5ae287429dde27ee64cc
//       1eb82ad36d8e9ff96f605535e01b8ac00d8472032a4fadc8a9c1e47aeba73c641ec12c7ea798f183
//       6919d7742d118804068b6022a01b045d880539e8229b5b52f345b3f6f6bce888df6f00238e4c52c3
//       05a3b5d6cf7db10924eac35556e2cd5978c4fb8a772462224c3e30ff77a857e6b27ac0cdce6ee733
//       f1806bcf53fc50fd6099e7bf3618ad6a3268d8a984abbb2ce2ee032b583523605edcbd08e8adcc96
//       49eac48d15d29ef9645139b252938db94a
//   ciphertext:
//       0x4982224e247d39f937b1b055b8e956e2f5786d9a296301dd72874d836eeda609c1a52ef20796f4
//       98d25ee8ab096f6a65486295b22c6471929d779bd0aa57acebf939d72df211dbf248839f0e4ddb36
//       2221e6b64289d09047a7bf7ae1d504a19cb2e2e40abff84b263899e3c98ebf0c7d56caa8fb6e3407
//       ef0eabaadb1faf724d19bbac63f597c7f3f63735505f6ea78059ffefd5e523eafa39f0857cd8aaf2
//       6494e1a98f3aeef3091f74e8a1e97efb669044e7345f08b7250906fec644037fbbe3c51c5d8862ca
//       8145f6395c2138fda65b6e523f891ec21ac61b356aa59472981e8c7dc8d66f3c4f9cc2b42b9eac70
//       674ca0e05124cea778cd3b5fe470c5361e7340353e38dbafe63aa5b4b479c0571dc846dee0f4314e
//       a673d7693e2767072f819a6346957b1cb9b2e1c4ec1f0d6e642b01a2e0cfb24d1d973239553477e1
//       5f809b1c32b451353cc723e8c5156883c7255bbfee64217aa0e24a4c9b2cf4f37d75a39d62277a71
//       d08af0045d96ea99a3bb0e9d886f2f17e7f94eb8086ed580af0bdb0d1f7c36d0de995956eb39143e
//       05b40091bee973d5c2fe331bd925dd340c5af4b5b0e1cdfd7f873a4b1e0f9ce49b3633bbfa2be259
//       609c6cd3ac9472576f1b5396cef11393a5520b0d3c7e46ee2257fb7c6d981225bcb328bf636ce490
//       d42789d1bb20c5db3e5d927449886b664a884686ba7c797a235985bfa56ed9b2b3d2a8612b8b64b7
//       14683025f881f327ebc966d7f57f02f0ba2d569ab4b9e14c
//   totp_opaque_key_id: None
const PKI_DEVICE_381: &[u8] = &hex!(
    "8ea474797065a3706b69aa637265617465645f6f6ed7010006519be643d034ac70726f"
    "7465637465645f6f6ed7010006519be643d036aa7365727665725f75726cd92e687474"
    "70733a2f2f736161732d64656d6f2d76332d6d696768747966616972792e7061727365"
    "632e636c6f75642faf6f7267616e697a6174696f6e5f6964a754657374504b49a77573"
    "65725f6964d802b6d02d1b22494a9d97839191ce835765a96465766963655f6964d802"
    "c91f53597b0a4c88860de825ef00685fac68756d616e5f68616e646c6592af666f6f40"
    "6578616d706c652e636f6da3666f6fac6465766963655f6c6162656ca757696e646f77"
    "73af63657274696669636174655f72656682a4757269739181aa77696e646f7773636e"
    "6782a6697373756572c42f302d3116301406035504030c0d426c61636b204d65736120"
    "434131133011060355040a0c0a426c61636b204d657361ad73657269616c5f6e756d62"
    "6572c414149e3eda0553d3e5063ca8b71a8220292b4daa5da468617368d93373686132"
    "35362d46483435526e3873464935585442784531696e6b52557865566d4241396a7473"
    "64536b592b3677332b67513da9616c676f726974686db152534145532d4f4145502d53"
    "4841323536ad656e637279707465645f6b6579c501008499241ebd78d1ee30f94cdff3"
    "1bd63ce0d6a84978907705857752e49925902e301150f5a5aa7843eee4edf4b0b278f0"
    "c3e8a10fdcc2e0af17f1c35705f5a8845ce22da8ea5ae287429dde27ee64cc1eb82ad3"
    "6d8e9ff96f605535e01b8ac00d8472032a4fadc8a9c1e47aeba73c641ec12c7ea798f1"
    "836919d7742d118804068b6022a01b045d880539e8229b5b52f345b3f6f6bce888df6f"
    "00238e4c52c305a3b5d6cf7db10924eac35556e2cd5978c4fb8a772462224c3e30ff77"
    "a857e6b27ac0cdce6ee733f1806bcf53fc50fd6099e7bf3618ad6a3268d8a984abbb2c"
    "e2ee032b583523605edcbd08e8adcc9649eac48d15d29ef9645139b252938db94aaa63"
    "697068657274657874c5021f4982224e247d39f937b1b055b8e956e2f5786d9a296301"
    "dd72874d836eeda609c1a52ef20796f498d25ee8ab096f6a65486295b22c6471929d77"
    "9bd0aa57acebf939d72df211dbf248839f0e4ddb362221e6b64289d09047a7bf7ae1d5"
    "04a19cb2e2e40abff84b263899e3c98ebf0c7d56caa8fb6e3407ef0eabaadb1faf724d"
    "19bbac63f597c7f3f63735505f6ea78059ffefd5e523eafa39f0857cd8aaf26494e1a9"
    "8f3aeef3091f74e8a1e97efb669044e7345f08b7250906fec644037fbbe3c51c5d8862"
    "ca8145f6395c2138fda65b6e523f891ec21ac61b356aa59472981e8c7dc8d66f3c4f9c"
    "c2b42b9eac70674ca0e05124cea778cd3b5fe470c5361e7340353e38dbafe63aa5b4b4"
    "79c0571dc846dee0f4314ea673d7693e2767072f819a6346957b1cb9b2e1c4ec1f0d6e"
    "642b01a2e0cfb24d1d973239553477e15f809b1c32b451353cc723e8c5156883c7255b"
    "bfee64217aa0e24a4c9b2cf4f37d75a39d62277a71d08af0045d96ea99a3bb0e9d886f"
    "2f17e7f94eb8086ed580af0bdb0d1f7c36d0de995956eb39143e05b40091bee973d5c2"
    "fe331bd925dd340c5af4b5b0e1cdfd7f873a4b1e0f9ce49b3633bbfa2be259609c6cd3"
    "ac9472576f1b5396cef11393a5520b0d3c7e46ee2257fb7c6d981225bcb328bf636ce4"
    "90d42789d1bb20c5db3e5d927449886b664a884686ba7c797a235985bfa56ed9b2b3d2"
    "a8612b8b64b714683025f881f327ebc966d7f57f02f0ba2d569ab4b9e14cb2746f7470"
    "5f6f70617175655f6b65795f6964c0"
);

#[parsec_test]
async fn load_old_pki_device_12800(tmp_path: TmpPath) {
    let key_file: PathBuf = tmp_path.join("devices/pki_381_windows.keys");
    libparsec_platform_filesystem::save_content(&key_file, PKI_DEVICE_381)
        .await
        .unwrap();
    let mut devices = libparsec_platform_device_loader::list_available_devices(&tmp_path)
        .await
        .unwrap();

    p_assert_eq!(
        devices,
        [AvailableDevice {
            key_file_path: key_file,
            created_on: "2026-05-12T09:58:05.248564Z".parse().unwrap(),
            protected_on: "2026-05-12T09:58:05.248566Z".parse().unwrap(),
            server_addr: "parsec3://saas-demo-v3-mightyfairy.parsec.cloud/"
                .parse()
                .unwrap(),
            organization_id: "TestPKI".parse().unwrap(),
            user_id: UserID::from_hex("b6d02d1b-2249-4a9d-9783-9191ce835765").unwrap(),
            device_id: DeviceID::from_hex("c91f5359-7b0a-4c88-860d-e825ef00685f").unwrap(),
            human_handle: HumanHandle::new("foo@example.com".parse().unwrap(), "foo").unwrap(),
            device_label: "Windows".parse().unwrap(),
            totp_opaque_key_id: None,
            ty: AvailableDeviceType::PKI {
                certificate_ref: X509CertificateReference::from("sha256-FH45Rn8sFI5XTBxE1inkRUxeVmBA9jtsdSkY+6w3+gQ="
                    .parse::<X509CertificateHash>()
                    .unwrap())
                    .add_or_replace_uri(X509WindowsCngURI {
                        issuer: hex!("302d3116301406035504030c0d426c61636b204d65736120434131133011060355040a0c0a426c61636b204d657361").to_vec(),
                        serial_number: hex!("149e3eda0553d3e5063ca8b71a8220292b4daa5d").to_vec()
                    })
            }
        }]
    );
    let AvailableDevice {
        key_file_path, ty, ..
    } = devices.pop().unwrap();
    let AvailableDeviceType::PKI {
        certificate_ref: cert_ref,
    } = ty
    else {
        unreachable!("Tested in the assert above");
    };

    #[cfg(not(windows))]
    {
        drop(key_file_path);
        drop(cert_ref);
    };

    #[cfg(windows)]
    {
        use std::sync::Arc;

        let pki = libparsec_platform_pki::PkiSystem::init(&tmp_path, None)
            .await
            .unwrap();
        let cert = Arc::new(pki.open_certificate(&cert_ref).await.unwrap());
        let pkey = Arc::new(cert.request_private_key().await.unwrap());

        let access = libparsec_platform_device_loader::DeviceAccessStrategy {
            key_file: key_file_path,
            totp_protection: None,
            primary_protection:
                libparsec_platform_device_loader::DevicePrimaryProtectionStrategy::PKI {
                    operations: Arc::new(crate::device::strategy::PkiOperations {
                        pki_certificate: cert,
                        pki_private_key: pkey,
                        certificate_ref: cert_ref,
                    }),
                },
        };
        let dev = libparsec_platform_device_loader::load_device(&tmp_path, &access)
            .await
            .unwrap();

        p_assert_eq!(dev.organization_id(), &"TestPKI".parse().unwrap());
    }
}

#[parsec_test]
async fn load_old_pki_device_12800_highlvl(tmp_path: TmpPath) {
    let key_file: PathBuf = tmp_path.join("devices/pki_381_windows.keys");
    libparsec_platform_filesystem::save_content(&key_file, PKI_DEVICE_381)
        .await
        .unwrap();
    let mut devices = libparsec_platform_device_loader::list_available_devices(&tmp_path)
        .await
        .unwrap();

    p_assert_eq!(
        devices,
        [AvailableDevice {
            key_file_path: key_file,
            created_on: "2026-05-12T09:58:05.248564Z".parse().unwrap(),
            protected_on: "2026-05-12T09:58:05.248566Z".parse().unwrap(),
            server_addr: "parsec3://saas-demo-v3-mightyfairy.parsec.cloud/"
                .parse()
                .unwrap(),
            organization_id: "TestPKI".parse().unwrap(),
            user_id: UserID::from_hex("b6d02d1b-2249-4a9d-9783-9191ce835765").unwrap(),
            device_id: DeviceID::from_hex("c91f5359-7b0a-4c88-860d-e825ef00685f").unwrap(),
            human_handle: HumanHandle::new("foo@example.com".parse().unwrap(), "foo").unwrap(),
            device_label: "Windows".parse().unwrap(),
            totp_opaque_key_id: None,
            ty: AvailableDeviceType::PKI {
                certificate_ref: X509CertificateReference::from("sha256-FH45Rn8sFI5XTBxE1inkRUxeVmBA9jtsdSkY+6w3+gQ="
                    .parse::<X509CertificateHash>()
                    .unwrap())
                    .add_or_replace_uri(X509WindowsCngURI {
                        issuer: hex!("302d3116301406035504030c0d426c61636b204d65736120434131133011060355040a0c0a426c61636b204d657361").to_vec(),
                        serial_number: hex!("149e3eda0553d3e5063ca8b71a8220292b4daa5d").to_vec()
                    })
            }
        }]
    );

    let AvailableDevice {
        key_file_path, ty, ..
    } = devices.pop().unwrap();
    let AvailableDeviceType::PKI {
        certificate_ref: cert_ref,
    } = ty
    else {
        unreachable!("Tested in the assert above");
    };

    #[cfg(not(windows))]
    {
        drop(key_file_path);
        drop(cert_ref);
    };

    #[cfg(windows)]
    {
        crate::pki_init_for_native(&tmp_path).await.unwrap();
        let cert_handle = crate::pki_open_user_certificate_private_key(&cert_ref)
            .await
            .unwrap();
        let client_config = crate::ClientConfig {
            data_base_dir: tmp_path.join("/data"),
            config_dir: tmp_path.to_path_buf(),
            mountpoint_mount_strategy: crate::MountpointMountStrategy::Disabled,
            workspace_storage_cache_size: crate::WorkspaceStorageCacheSize::Default,
            with_monitors: false,
            prevent_sync_pattern: None,
            log_level: Some(log::Level::Trace),
        };
        let access = crate::DeviceAccessStrategy {
            key_file: key_file_path,
            totp_protection: None,
            primary_protection: crate::DevicePrimaryProtectionStrategy::PKI {
                pki_private_key_handle: cert_handle,
            },
        };
        let client_handle = crate::client_start(client_config, access).await.unwrap();

        assert_ne!(client_handle, 0);
    }
}
