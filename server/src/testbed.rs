// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict, PyList},
};
use std::sync::Arc;

use crate::{
    BlockID, DateTime, DeviceCertificate, DeviceID, DeviceLabel, HumanHandle, PrivateKey,
    RealmRole, RealmRoleCertificate, RevokedUserCertificate, SecretKey,
    SequesterAuthorityCertificate, SequesterPrivateKeyDer, SequesterPublicKeyDer,
    SequesterServiceCertificate, SequesterServiceID, SequesterSigningKeyDer, SequesterVerifyKeyDer,
    SigningKey, UserCertificate, UserID, UserProfile, UserUpdateCertificate, VlobID,
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
        first_user_human_handle: Option<HumanHandle>,
        first_user_private_key: PrivateKey,
        first_user_first_device_label: Option<DeviceLabel>,
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
        raw_redacted_certificate: Py<PyBytes>,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventNewSequesterService| -> PyResult<String> {
        Ok(format!("timestamp={:?}, id={:?}", x.timestamp.0, x.id.0))
    }
);

event_wrapper!(
    TestbedEventNewUser,
    [
        timestamp: DateTime,
        author: DeviceID,
        device_id: DeviceID,
        human_handle: Option<HumanHandle>,
        private_key: PrivateKey,
        first_device_label: Option<DeviceLabel>,
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
        device_label: Option<DeviceLabel>,
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
        raw_redacted_certificate: Py<PyBytes>,
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
        raw_redacted_certificate: Py<PyBytes>,
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
    TestbedEventNewRealm,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm_id: VlobID,
        realm_key: SecretKey,
        certificate: RealmRoleCertificate,
        raw_redacted_certificate: Py<PyBytes>,
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
        recipient_message: Py<PyBytes>,
        certificate: RealmRoleCertificate,
        raw_redacted_certificate: Py<PyBytes>,
        raw_certificate: Py<PyBytes>,
    ],
    |py, x: &TestbedEventShareRealm| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user={:?}, role={}",
            x.timestamp.0,
            x.author.0,
            x.user.0,
            match &x.role {
                None => "None",
                Some(p) => p.as_ref(py).repr()?.to_str()?,
            }
        ))
    }
);

event_wrapper!(
    TestbedEventStartRealmReencryption,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        encryption_revision: libparsec_types::IndexInt,
        per_participant_message: Py<PyList>,
    ],
    |_py, x: &TestbedEventStartRealmReencryption| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, encryption_revision={}",
            x.timestamp.0, x.author.0, x.realm.0, x.encryption_revision,
        ))
    }
);

event_wrapper!(
    TestbedEventFinishRealmReencryption,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        encryption_revision: libparsec_types::IndexInt,
    ],
    |_py, x: &TestbedEventFinishRealmReencryption| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, encryption_revision={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.encryption_revision,
        ))
    }
);

event_wrapper!(
    TestbedEventCreateOrUpdateOpaqueVlob,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        encryption_revision: libparsec_types::IndexInt,
        vlob_id: VlobID,
        version: libparsec_types::VersionInt,
        encrypted: Py<PyBytes>,
        sequestered: PyObject,
    ],
    |_py, x: &TestbedEventCreateOrUpdateOpaqueVlob| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, vlob={:?}, version={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.vlob_id.0, x.version
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
                            libparsec_types::AnyArcCertificate::SequesterAuthority(c) => {
                                SequesterAuthorityCertificate::from(c).into_py(py)
                            }
                            libparsec_types::AnyArcCertificate::SequesterService(c) => {
                                SequesterServiceCertificate::from(c).into_py(py)
                            }
                        };
                        (
                            py_certif,
                            PyBytes::new(py, &certif.raw),
                            PyBytes::new(py, &certif.raw_redacted),
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
            let raw = PyBytes::new($py, &certif.raw).into_py($py);
            let raw_redacted = PyBytes::new($py, &certif.raw_redacted).into_py($py);
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
                first_user_human_handle: x.first_user_human_handle.clone().map(HumanHandle::from),
                first_user_private_key: x.first_user_private_key.clone().into(),
                first_user_first_device_label: x
                    .first_user_first_device_label
                    .clone()
                    .map(DeviceLabel::from),
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
                    .map(|x| PyBytes::new(py, &x.raw).into_py(py)),
                first_user_certificate: UserCertificate::from(match &user_certif.certificate {
                    libparsec_types::AnyArcCertificate::User(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_user_raw_certificate: PyBytes::new(py, &user_certif.raw).into_py(py),
                first_user_raw_redacted_certificate: PyBytes::new(py, &user_certif.raw_redacted)
                    .into_py(py),
                first_user_first_device_certificate: DeviceCertificate::from(match &device_certif
                    .certificate
                {
                    libparsec_types::AnyArcCertificate::Device(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_user_first_device_raw_certificate: PyBytes::new(py, &device_certif.raw)
                    .into_py(py),
                first_user_first_device_raw_redacted_certificate: PyBytes::new(
                    py,
                    &device_certif.raw_redacted,
                )
                .into_py(py),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewSequesterService(x) => {
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate!(py, x, template, SequesterService);
            let obj = TestbedEventNewSequesterService {
                timestamp: x.timestamp.into(),
                id: x.id.into(),
                label: x.label.clone(),
                encryption_private_key: x.encryption_private_key.clone().into(),
                encryption_public_key: x.encryption_public_key.clone().into(),
                raw_certificate,
                raw_redacted_certificate,
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
                human_handle: x.human_handle.clone().map(HumanHandle::from),
                private_key: x.private_key.clone().into(),
                first_device_label: x.first_device_label.clone().map(DeviceLabel::from),
                first_device_signing_key: x.first_device_signing_key.clone().into(),
                initial_profile: UserProfile::convert(x.initial_profile),
                user_realm_id: x.user_realm_id.into(),
                user_realm_key: x.user_realm_key.clone().into(),
                local_symkey: x.local_symkey.clone().into(),
                local_password: x.local_password,
                user_raw_certificate: PyBytes::new(py, &user_certif.raw).into_py(py),
                user_raw_redacted_certificate: PyBytes::new(py, &user_certif.raw_redacted)
                    .into_py(py),
                user_certificate: UserCertificate::from(match &user_certif.certificate {
                    libparsec_types::AnyArcCertificate::User(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_device_raw_certificate: PyBytes::new(py, &device_certif.raw).into_py(py),
                first_device_raw_redacted_certificate: PyBytes::new(
                    py,
                    &device_certif.raw_redacted,
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
                single_certificate!(py, x, template, Device);
            let obj = TestbedEventNewDevice {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                device_id: x.device_id.clone().into(),
                device_label: x.device_label.clone().map(|x| x.into()),
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
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate!(py, x, template, UserUpdate);
            let obj = TestbedEventUpdateUserProfile {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                user: x.user.clone().into(),
                profile: UserProfile::convert(x.profile),
                raw_certificate,
                raw_redacted_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::RevokeUser(x) => {
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate!(py, x, template, RevokedUser);
            let obj = TestbedEventRevokeUser {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                user: x.user.clone().into(),
                raw_certificate,
                raw_redacted_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::NewRealm(x) => {
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate!(py, x, template, RealmRole);
            let obj = TestbedEventNewRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm_id: x.realm_id.into(),
                realm_key: x.realm_key.clone().into(),
                raw_certificate,
                raw_redacted_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::ShareRealm(x) => {
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate!(py, x, template, RealmRole);
            let recipient_message = x.recipient_message(template);
            let obj = TestbedEventShareRealm {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                user: x.user.clone().into(),
                role: x.role.map(RealmRole::convert),
                recipient_message: PyBytes::new(py, &recipient_message.raw).into_py(py),
                raw_certificate,
                raw_redacted_certificate,
                certificate,
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::StartRealmReencryption(x) => {
            let per_participant_message = x.per_participant_message(template);
            let obj = TestbedEventStartRealmReencryption {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                encryption_revision: x.encryption_revision,
                per_participant_message: PyList::new(
                    py,
                    per_participant_message
                        .items
                        .iter()
                        .map(|(user_id, _, raw)| {
                            (
                                UserID::from(user_id.to_owned()).into_py(py),
                                PyBytes::new(py, raw),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
                .into_py(py),
            };
            Some(obj.into_py(py))
        }

        libparsec_testbed::TestbedEvent::FinishRealmReencryption(x) => {
            let obj = TestbedEventFinishRealmReencryption {
                timestamp: x.timestamp.into(),
                author: x.author.clone().into(),
                realm: x.realm.into(),
                encryption_revision: x.encryption_revision,
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
                encryption_revision: x.encryption_revision,
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
                encryption_revision: 1,
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
                encryption_revision: 1,
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
                encryption_revision: 1,
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
                encryption_revision: 1,
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
