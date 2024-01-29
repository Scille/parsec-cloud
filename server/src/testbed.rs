// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict, PyList},
};
use std::collections::HashMap;
use std::num::NonZeroU64;
use std::sync::Arc;

use crate::{
    data::{
        DeviceCertificate, RealmArchivingCertificate, RealmKeyRotationCertificate,
        RealmNameCertificate, RealmRoleCertificate, RevokedUserCertificate,
        SequesterAuthorityCertificate, SequesterRevokedServiceCertificate,
        SequesterServiceCertificate, ShamirRecoveryBriefCertificate,
        ShamirRecoveryShareCertificate, UserCertificate, UserUpdateCertificate,
    },
    BlockID, DateTime, DeviceID, DeviceLabel, HumanHandle, InvitationToken, PrivateKey, RealmRole,
    SecretKey, SequesterPrivateKeyDer, SequesterPublicKeyDer, SequesterServiceID,
    SequesterSigningKeyDer, SequesterVerifyKeyDer, SigningKey, UserID, UserProfile, VlobID,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct TestbedTemplateContent {
    template: Arc<libparsec_testbed::TestbedTemplate>,
    #[pyo3(get)]
    id: &'static str,
    #[pyo3(get)]
    events: Py<PyList>,
    #[pyo3(get)]
    certificates: Py<PyList>,
}

#[pymethods]
impl TestbedTemplateContent {
    fn compute_crc(&self) -> u32 {
        self.template.compute_crc()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "TestbedTemplateContent(events={})",
            self.events.as_ref(py).repr()?.to_str()?
        ))
    }
}

macro_rules! event_wrapper {
    ($name: ident, [ $($field_name:ident: $field_type:ty),* $(,)? ], $repr_fn: expr $(,)? ) => {
        #[pyclass(frozen)]
        pub(crate) struct $name {
            $(
                #[pyo3(get)]
                $field_name: $field_type,
            )*
        }
        #[pymethods]
        impl $name {
            fn __repr__(&self, py: Python) -> PyResult<String> {
                Ok(format!(
                    concat!(
                        stringify!($name),
                        "({})"
                    ),
                    &$repr_fn(py, self)?
                ))
            }
        }
    };
}

event_wrapper!(
    TestbedEventBootstrapOrganization,
    [
        timestamp: DateTime,
        root_signing_key: SigningKey,
        sequester_authority_signing_key: Option<SequesterSigningKeyDer>,
        sequester_authority_verify_key: Option<SequesterVerifyKeyDer>,
        first_user_device_id: DeviceID,
        first_user_human_handle: HumanHandle,
        first_user_private_key: PrivateKey,
        first_user_first_device_label: DeviceLabel,
        first_user_first_device_signing_key: SigningKey,
        first_user_user_realm_id: VlobID,
        first_user_user_realm_key: SecretKey,
        first_user_local_symkey: SecretKey,
        first_user_local_password: &'static str,
        sequester_authority_certificate: Option<SequesterAuthorityCertificate>,
        sequester_authority_raw_certificate: Option<Py<PyBytes>>,
        first_user_certificate: UserCertificate,
        first_user_raw_certificate: Py<PyBytes>,
        first_user_raw_redacted_certificate: Py<PyBytes>,
        first_user_first_device_certificate: DeviceCertificate,
        first_user_first_device_raw_certificate: Py<PyBytes>,
        first_user_first_device_raw_redacted_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventBootstrapOrganization| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, sequestered={}, first_user={:?}",
            x.timestamp.0,
            if x.sequester_authority_signing_key.is_some() {
                "true"
            } else {
                "false"
            },
            x.first_user_device_id.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewSequesterService,
    [
        timestamp: DateTime,
        id: SequesterServiceID,
        label: String,
        encryption_private_key: SequesterPrivateKeyDer,
        encryption_public_key: SequesterPublicKeyDer,
        certificate: SequesterServiceCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventNewSequesterService| -> PyResult<String> {
        Ok(format!("timestamp={:?}, id={:?}", x.timestamp.0, x.id.0))
    }
);

event_wrapper!(
    TestbedEventRevokeSequesterService,
    [
        timestamp: DateTime,
        id: SequesterServiceID,
        certificate: SequesterRevokedServiceCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventRevokeSequesterService| -> PyResult<String> {
        Ok(format!("timestamp={:?}, id={:?}", x.timestamp.0, x.id.0))
    }
);

event_wrapper!(
    TestbedEventNewUser,
    [
        timestamp: DateTime,
        author: DeviceID,
        device_id: DeviceID,
        human_handle: HumanHandle,
        private_key: PrivateKey,
        first_device_label: DeviceLabel,
        first_device_signing_key: SigningKey,
        initial_profile: &'static PyObject,
        user_realm_id: VlobID,
        user_realm_key: SecretKey,
        local_symkey: SecretKey,
        local_password: &'static str,
        user_certificate: UserCertificate,
        user_raw_redacted_certificate: Py<PyBytes>,
        user_raw_certificate: Py<PyBytes>,
        first_device_certificate: DeviceCertificate,
        first_device_raw_redacted_certificate: Py<PyBytes>,
        first_device_raw_certificate: Py<PyBytes>,
    ],
    |py, x: &TestbedEventNewUser| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user={:?}, profile={}",
            x.timestamp.0,
            x.author.0,
            x.device_id.0,
            x.initial_profile.as_ref(py).repr()?.to_str()?,
        ))
    }
);

event_wrapper!(
    TestbedEventNewDevice,
    [
        timestamp: DateTime,
        author: DeviceID,
        device_id: DeviceID,
        device_label: DeviceLabel,
        signing_key: SigningKey,
        local_symkey: SecretKey,
        local_password: &'static str,
        certificate: DeviceCertificate,
        raw_redacted_certificate: Py<PyBytes>,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventNewDevice| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, device={:?}",
            x.timestamp.0, x.author.0, x.device_id.0,
        ))
    }
);

event_wrapper!(
    TestbedEventUpdateUserProfile,
    [
        timestamp: DateTime,
        author: DeviceID,
        user: UserID,
        profile: &'static PyObject,
        certificate: UserUpdateCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |py, x: &TestbedEventUpdateUserProfile| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user={:?}, profile={}",
            x.timestamp.0,
            x.author.0,
            x.user.0,
            x.profile.as_ref(py).repr()?.to_str()?,
        ))
    }
);

event_wrapper!(
    TestbedEventRevokeUser,
    [
        timestamp: DateTime,
        author: DeviceID,
        user: UserID,
        certificate: RevokedUserCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventRevokeUser| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user={:?}",
            x.timestamp.0, x.author.0, x.user.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewDeviceInvitation,
    [
        greeter_user_id: UserID,
        created_on: DateTime,
        token: InvitationToken,
    ],
    |_py, x: &TestbedEventNewDeviceInvitation| -> PyResult<String> {
        Ok(format!(
            "greeter_user_id={:?}, created_on={:?}, token={:?}",
            x.greeter_user_id.0, x.created_on.0, x.token.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewUserInvitation,
    [
        claimer_email: String,
        greeter_user_id: UserID,
        created_on: DateTime,
        token: InvitationToken,
    ],
    |_py, x: &TestbedEventNewUserInvitation| -> PyResult<String> {
        Ok(format!(
            "claimer_email={:?}, greeter_user_id={:?}, created_on={:?}, token={:?}",
            x.claimer_email, x.greeter_user_id.0, x.created_on.0, x.token.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm_id: VlobID,
        certificate: RealmRoleCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventNewRealm| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}",
            x.timestamp.0, x.author.0, x.realm_id.0,
        ))
    }
);

event_wrapper!(
    TestbedEventShareRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        user: UserID,
        role: Option<&'static PyObject>,
        key_index: Option<libparsec_types::IndexInt>,
        recipient_keys_bundle_access: Option<Py<PyBytes>>,
        certificate: RealmRoleCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |py, x: &TestbedEventShareRealm| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, user={:?}, role={}",
            x.timestamp.0,
            x.author.0,
            x.realm.0,
            x.user.0,
            match &x.role {
                None => "None",
                Some(p) => p.as_ref(py).repr()?.to_str()?,
            }
        ))
    }
);

event_wrapper!(
    TestbedEventRenameRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        certificate: RealmNameCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventRenameRealm| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}",
            x.timestamp.0,
            x.author.0,
            x.realm.0,
        ))
    }
);

event_wrapper!(
    TestbedEventRotateKeyRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        key_index: u64,
        per_participant_keys_bundle_access: Py<PyDict>,
        keys_bundle: Py<PyBytes>,
        certificate: RealmKeyRotationCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventRotateKeyRealm| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, key_index={:?}",
            x.timestamp.0,
            x.author.0,
            x.realm.0,
            x.key_index,
        ))
    }
);

event_wrapper!(
    TestbedEventArchiveRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        certificate: RealmArchivingCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventArchiveRealm| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}",
            x.timestamp.0,
            x.author.0,
            x.realm.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewShamirRecovery,
    [
        timestamp: DateTime,
        author: DeviceID,
        threshold: NonZeroU64,
        per_recipient_shares: HashMap<UserID, NonZeroU64>,
        brief_certificate: ShamirRecoveryBriefCertificate,
        raw_brief_certificate: Py<PyBytes>,
        share_certificates: Py<PyList>,
        raw_share_certificates: Py<PyList>,
    ],
    |_py, x: &TestbedEventNewShamirRecovery| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, threshold={:?}, per_recipient_shares={:?}",
            x.timestamp.0,
            x.author.0,
            x.threshold,
            x.per_recipient_shares.iter().map(|(k, v)| (k.0.clone(), v)).collect::<HashMap<_, _>>(),
        ))
    }
);

event_wrapper!(
    TestbedEventCreateOrUpdateOpaqueVlob,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        vlob_id: VlobID,
        key_index: libparsec_types::IndexInt,
        version: libparsec_types::VersionInt,
        encrypted: Py<PyBytes>,
        sequestered: PyObject,
    ],
    |_py, x: &TestbedEventCreateOrUpdateOpaqueVlob| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, vlob={:?}, key_index={:?}, version={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.vlob_id.0, x.key_index, x.version
        ))
    }
);

event_wrapper!(
    TestbedEventCreateOpaqueBlock,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        encrypted: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventCreateOpaqueBlock| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, block={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.block_id.0,
        ))
    }
);

#[pyfunction]
pub(crate) fn test_get_testbed_template(
    py: Python,
    id: &str,
) -> PyResult<Option<TestbedTemplateContent>> {
    match libparsec_testbed::test_get_template(id) {
        None => Ok(None),
        Some(template) => {
            let events = {
                let events = PyList::empty(py);
                for event in template.events.iter() {
                    if let Some(pyobj) = event_to_pyobject(py, &template, event)? {
                        events.append(pyobj)?;
                    }
                }
                events
            };
            let certificates = PyList::new(
                py,
                template
                    .certificates()
                    .map(|certif| {
                        let py_certif = match certif.certificate {
                            libparsec_types::AnyArcCertificate::User(c) => {
                                UserCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::Device(c) => {
                                DeviceCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::UserUpdate(c) => {
                                UserUpdateCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::RevokedUser(c) => {
                                RevokedUserCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::RealmRole(c) => {
                                RealmRoleCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::RealmName(c) => {
                                RealmNameCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::RealmKeyRotation(c) => {
                                RealmKeyRotationCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::RealmArchiving(c) => {
                                RealmArchivingCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::ShamirRecoveryBrief(c) => {
                                ShamirRecoveryBriefCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::ShamirRecoveryShare(c) => {
                                ShamirRecoveryShareCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::SequesterAuthority(c) => {
                                SequesterAuthorityCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::SequesterRevokedService(c) => {
                                SequesterRevokedServiceCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::SequesterService(c) => {
                                SequesterServiceCertificate::from(c).into_py(py)
                            }
                        };
                        (
                            py_certif,
                            PyBytes::new(py, &certif.signed),
                            PyBytes::new(py, &certif.signed_redacted),
                        )
                    })
                    .collect::<Vec<_>>(),
            );
            Ok(Some(TestbedTemplateContent {
                id: template.id,
                template,
                events: events.into_py(py),
                certificates: certificates.into_py(py),
            }))
        }
    }
}

fn event_to_pyobject(
    py: Python,
    template: &libparsec_testbed::TestbedTemplate,
    event: &libparsec_testbed::TestbedEvent,
) -> PyResult<Option<PyObject>> {
    macro_rules! single_certificate {
        ($py: expr, $event: expr, $template: expr, $name: ident) => {{
            let certif = $event
                .certificates($template)
                .next()
                .expect("Must be present");
            let raw = PyBytes::new($py, &certif.signed).into_py($py);
            let wrapped_certif = paste::paste! {
                [< $name Certificate >]::from(match &certif.certificate {
                    libparsec_types::AnyArcCertificate::$name(x) => x.to_owned(),
                    _ => unreachable!(),
                })
            };
            (wrapped_certif, raw)
        }};
    }
    macro_rules! single_certificate_with_redacted {
        ($py: expr, $event: expr, $template: expr, $name: ident) => {{
            let certif = $event
                .certificates($template)
                .next()
                .expect("Must be present");
            let raw = PyBytes::new($py, &certif.signed).into_py($py);
            let raw_redacted = PyBytes::new($py, &certif.signed_redacted).into_py($py);
            let wrapped_certif = paste::paste! {
                [< $name Certificate >]::from(match &certif.certificate {
                    libparsec_types::AnyArcCertificate::$name(x) => x.to_owned(),
                    _ => unreachable!(),
                })
            };
            (wrapped_certif, raw, raw_redacted)
        }};
    }
    Ok(match event {
        libparsec_testbed::TestbedEvent::BootstrapOrganization(x) => {
            let mut certifs = x.certificates(template);
            let (user_certif, device_certif, sequester_authority_certif) =
                match (certifs.next(), certifs.next(), certifs.next()) {
                    (Some(sequester_authority_certif), Some(user_certif), Some(device_certif)) => {
                        (user_certif, device_certif, Some(sequester_authority_certif))
                    }
                    (Some(user_certif), Some(device_certif), None) => {
                        (user_certif, device_certif, None)
                    }
                    _ => unreachable!(),
                };
            let obj = TestbedEventBootstrapOrganization {
                timestamp: x.timestamp.into(),
                root_signing_key: x.root_signing_key.clone().into(),
                sequester_authority_signing_key: x
                    .sequester_authority
                    .as_ref()
                    .map(|sequester_authority| sequester_authority.signing_key.clone().into()),
                sequester_authority_verify_key: x
                    .sequester_authority
                    .as_ref()
                    .map(|sequester_authority| sequester_authority.verify_key.clone().into()),
                first_user_device_id: x.first_user_device_id.clone().into(),
                first_user_human_handle: x.first_user_human_handle.clone().into(),
                first_user_private_key: x.first_user_private_key.clone().into(),
                first_user_first_device_label: x.first_user_first_device_label.clone().into(),
                first_user_first_device_signing_key: x
                    .first_user_first_device_signing_key
                    .clone()
                    .into(),
                first_user_user_realm_id: x.first_user_user_realm_id.into(),
                first_user_user_realm_key: x.first_user_user_realm_key.clone().into(),
                first_user_local_symkey: x.first_user_local_symkey.clone().into(),
                first_user_local_password: x.first_user_local_password,
                sequester_authority_certificate: sequester_authority_certif.as_ref().map(|x| {
                    SequesterAuthorityCertificate::from(match &x.certificate {
                        libparsec_types::AnyArcCertificate::SequesterAuthority(x) => x.to_owned(),
                        _ => unreachable!(),
                    })
                }),
                sequester_authority_raw_certificate: sequester_authority_certif
                    .as_ref()
                    .map(|x| PyBytes::new(py, &x.signed).into_py(py)),
                first_user_certificate: UserCertificate::from(match &user_certif.certificate {
                    libparsec_types::AnyArcCertificate::User(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_user_raw_certificate: PyBytes::new(py, &user_certif.signed).into_py(py),
                first_user_raw_redacted_certificate: PyBytes::new(py, &user_certif.signed_redacted)
                    .into_py(py),
                first_user_first_device_certificate: DeviceCertificate::from(match &device_certif
                    .certificate
                {
                    libparsec_types::AnyArcCertificate::Device(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_user_first_device_raw_certificate: PyBytes::new(py, &device_certif.signed)
                    .into_py(py),
                first_user_first_device_raw_redacted_certificate: PyBytes::new(
                    py,
                    &device_certif.signed_redacted,
                )
                .into_py(py),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewSequesterService(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, SequesterService);
            let obj = TestbedEventNewSequesterService {
                timestamp: x.timestamp.into(),
                id: x.id.into(),
                label: x.label.clone(),
                encryption_private_key: x.encryption_private_key.clone().into(),
                encryption_public_key: x.encryption_public_key.clone().into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::RevokeSequesterService(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, SequesterRevokedService);
            let obj = TestbedEventRevokeSequesterService {
                timestamp: x.timestamp.into(),
                id: x.id.into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewUser(x) => {
            let (user_certif, device_certif) = {
                let mut certifs = x.certificates(template);
                (
                    certifs.next().expect("Must be present"),
                    certifs.next().expect("Must be present"),
                )
            };
            let obj = TestbedEventNewUser {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                device_id: x.device_id.clone().into(),
                human_handle: x.human_handle.clone().into(),
                private_key: x.private_key.clone().into(),
                first_device_label: x.first_device_label.clone().into(),
                first_device_signing_key: x.first_device_signing_key.clone().into(),
                initial_profile: UserProfile::convert(x.initial_profile),
                user_realm_id: x.user_realm_id.into(),
                user_realm_key: x.user_realm_key.clone().into(),
                local_symkey: x.local_symkey.clone().into(),
                local_password: x.local_password,
                user_raw_certificate: PyBytes::new(py, &user_certif.signed).into_py(py),
                user_raw_redacted_certificate: PyBytes::new(py, &user_certif.signed_redacted)
                    .into_py(py),
                user_certificate: UserCertificate::from(match &user_certif.certificate {
                    libparsec_types::AnyArcCertificate::User(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_device_raw_certificate: PyBytes::new(py, &device_certif.signed).into_py(py),
                first_device_raw_redacted_certificate: PyBytes::new(
                    py,
                    &device_certif.signed_redacted,
                )
                .into_py(py),
                first_device_certificate: DeviceCertificate::from(
                    match &device_certif.certificate {
                        libparsec_types::AnyArcCertificate::Device(x) => x.to_owned(),
                        _ => unreachable!(),
                    },
                ),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewDevice(x) => {
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate_with_redacted!(py, x, template, Device);
            let obj = TestbedEventNewDevice {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                device_id: x.device_id.clone().into(),
                device_label: x.device_label.clone().into(),
                signing_key: x.signing_key.clone().into(),
                local_symkey: x.local_symkey.clone().into(),
                local_password: x.local_password,
                raw_certificate,
                raw_redacted_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::UpdateUserProfile(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, UserUpdate);
            let obj = TestbedEventUpdateUserProfile {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                user: x.user.clone().into(),
                profile: UserProfile::convert(x.profile),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::RevokeUser(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RevokedUser);
            let obj = TestbedEventRevokeUser {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                user: x.user.clone().into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewDeviceInvitation(x) => {
            let obj = TestbedEventNewDeviceInvitation {
                greeter_user_id: x.greeter_user_id.clone().into(),
                created_on: x.created_on.into(),
                token: x.token.clone().into(),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewUserInvitation(x) => {
            let obj = TestbedEventNewUserInvitation {
                claimer_email: x.claimer_email.clone(),
                greeter_user_id: x.greeter_user_id.clone().into(),
                created_on: x.created_on.into(),
                token: x.token.clone().into(),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewRealm(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RealmRole);
            let obj = TestbedEventNewRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm_id: x.realm_id.into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::ShareRealm(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RealmRole);
            let obj = TestbedEventShareRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                user: x.user.clone().into(),
                role: x.role.map(RealmRole::convert),
                key_index: x.key_index,
                recipient_keys_bundle_access: x
                    .recipient_keys_bundle_access(template)
                    .map(|x| PyBytes::new(py, &x).into_py(py)),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::RenameRealm(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RealmName);
            let obj = TestbedEventRenameRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::RotateKeyRealm(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, RealmKeyRotation);
            let obj = TestbedEventRotateKeyRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                per_participant_keys_bundle_access: {
                    let pyobj = PyDict::new(py);
                    for (participant, access) in x.per_participant_keys_bundle_access() {
                        pyobj.set_item(
                            UserID::from(participant).into_py(py),
                            PyBytes::new(py, &access),
                        )?;
                    }
                    pyobj.into_py(py)
                },
                keys_bundle: PyBytes::new(py, &x.keys_bundle(template)).into_py(py),
                key_index: x.key_index.into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::ArchiveRealm(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, RealmArchiving);
            let obj = TestbedEventArchiveRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                raw_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewShamirRecovery(x) => {
            let mut certifs = x.certificates(template);

            let (brief_certificate, raw_brief_certificate) = {
                let certif = certifs.next().expect("Must be present");
                let raw_brief_certificate = PyBytes::new(py, &certif.signed).into_py(py);
                let brief_certificate =
                    ShamirRecoveryBriefCertificate::from(match &certif.certificate {
                        libparsec_types::AnyArcCertificate::ShamirRecoveryBrief(x) => x.to_owned(),
                        _ => unreachable!(),
                    });
                (brief_certificate, raw_brief_certificate)
            };

            let (share_certificates, raw_share_certificates) = {
                let share_certificates = PyList::empty(py);
                let raw_share_certificates = PyList::empty(py);
                for certif in certifs {
                    let raw = PyBytes::new(py, &certif.signed);
                    let wrapped_certif =
                        ShamirRecoveryShareCertificate::from(match &certif.certificate {
                            libparsec_types::AnyArcCertificate::ShamirRecoveryShare(x) => {
                                x.to_owned()
                            }
                            _ => unreachable!(),
                        });
                    share_certificates.append(wrapped_certif.into_py(py))?;
                    raw_share_certificates.append(raw)?;
                }
                (share_certificates, raw_share_certificates)
            };

            let obj = TestbedEventNewShamirRecovery {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                threshold: x.threshold,
                per_recipient_shares: x
                    .per_recipient_shares
                    .iter()
                    .map(|(k, v)| (k.clone().into(), *v))
                    .collect(),
                brief_certificate,
                raw_brief_certificate,
                share_certificates: share_certificates.into_py(py),
                raw_share_certificates: raw_share_certificates.into_py(py),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateOpaqueVlob(x) => {
            let sequestered = match &x.sequestered {
                None => py.None().into_py(py),
                Some(sequestered) => {
                    let pyobj = PyDict::new(py);
                    for (id, blob) in sequestered {
                        pyobj.set_item(
                            SequesterServiceID::from(id.to_owned()).into_py(py),
                            PyBytes::new(py, blob),
                        )?;
                    }
                    pyobj.into_py(py)
                }
            };
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                key_index: x.key_index,
                version: x.version,
                vlob_id: x.vlob_id.into(),
                encrypted: PyBytes::new(py, &x.encrypted).into(),
                sequestered,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateUserManifestVlob(x) => {
            let sequestered = match x.sequestered(template) {
                None => py.None().into_py(py),
                Some(sequestered) => {
                    let pyobj = PyDict::new(py);
                    for (id, blob) in sequestered {
                        pyobj.set_item(
                            SequesterServiceID::from(id.to_owned()).into_py(py),
                            PyBytes::new(py, &blob),
                        )?;
                    }
                    pyobj.into_py(py)
                }
            };
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.clone().into(),
                realm: x.manifest.id.into(),
                key_index: 0,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
                sequestered,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(x) => {
            let sequestered = match x.sequestered(template) {
                None => py.None().into_py(py),
                Some(sequestered) => {
                    let pyobj = PyDict::new(py);
                    for (id, blob) in sequestered {
                        pyobj.set_item(
                            SequesterServiceID::from(id.to_owned()).into_py(py),
                            PyBytes::new(py, &blob),
                        )?;
                    }
                    pyobj.into_py(py)
                }
            };
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.clone().into(),
                realm: x.manifest.id.into(),
                key_index: x.key_index,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
                sequestered,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateFileManifestVlob(x) => {
            let sequestered = match x.sequestered(template) {
                None => py.None().into_py(py),
                Some(sequestered) => {
                    let pyobj = PyDict::new(py);
                    for (id, blob) in sequestered {
                        pyobj.set_item(
                            SequesterServiceID::from(id.to_owned()).into_py(py),
                            PyBytes::new(py, &blob),
                        )?;
                    }
                    pyobj.into_py(py)
                }
            };
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.clone().into(),
                realm: x.realm.into(),
                key_index: x.key_index,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
                sequestered,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateFolderManifestVlob(x) => {
            let sequestered = match x.sequestered(template) {
                None => py.None().into_py(py),
                Some(sequestered) => {
                    let pyobj = PyDict::new(py);
                    for (id, blob) in sequestered {
                        pyobj.set_item(
                            SequesterServiceID::from(id.to_owned()).into_py(py),
                            PyBytes::new(py, &blob),
                        )?;
                    }
                    pyobj.into_py(py)
                }
            };
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.clone().into(),
                realm: x.realm.into(),
                key_index: x.key_index,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
                sequestered,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateOpaqueBlock(x) => {
            let obj = TestbedEventCreateOpaqueBlock {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                block_id: x.block_id.into(),
                encrypted: PyBytes::new(py, &x.encrypted).into(),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::CreateBlock(x) => {
            let obj = TestbedEventCreateOpaqueBlock {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                block_id: x.block_id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
            };
            Some(obj.into_py(py))
        }

        // Ignore non-server events
        _ => None,
    })
}
