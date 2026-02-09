// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyBool, PyBytes, PyDict, PyList, PyString},
    IntoPyObjectExt,
};
use std::collections::HashMap;
use std::num::NonZeroU8;
use std::sync::Arc;

use crate::{
    data::{
        DeviceCertificate, RealmArchivingCertificate, RealmKeyRotationCertificate,
        RealmNameCertificate, RealmRoleCertificate, RevokedUserCertificate,
        SequesterAuthorityCertificate, SequesterRevokedServiceCertificate,
        SequesterServiceCertificate, ShamirRecoveryBriefCertificate,
        ShamirRecoveryDeletionCertificate, ShamirRecoveryShareCertificate, UserCertificate,
        UserUpdateCertificate,
    },
    protocol::ActiveUsersLimit,
    AccessToken, BlockID, DateTime, DeviceID, DeviceLabel, EmailAddress, HumanHandle, PrivateKey,
    RealmRole, SecretKey, SequesterPrivateKeyDer, SequesterPublicKeyDer, SequesterServiceID,
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
}

#[pymethods]
impl TestbedTemplateContent {
    fn compute_crc(&self) -> u32 {
        self.template.compute_crc()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "TestbedTemplateContent(events={})",
            self.events.bind(py).repr()?.to_str()?
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
        first_user_id: UserID,
        first_user_human_handle: HumanHandle,
        first_user_private_key: PrivateKey,
        first_user_first_device_id: DeviceID,
        first_user_first_device_label: DeviceLabel,
        first_user_first_device_signing_key: SigningKey,
        first_user_user_realm_id: VlobID,
        first_user_user_realm_key: SecretKey,
        first_user_local_symkey: SecretKey,
        first_user_local_password: Py<PyString>,
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
            "timestamp={:?}, sequestered={}, first_user={:?}, first_device={:?}",
            x.timestamp.0,
            if x.sequester_authority_signing_key.is_some() {
                "true"
            } else {
                "false"
            },
            x.first_user_id.0,
            x.first_user_first_device_id.0,
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
        user_id: UserID,
        human_handle: HumanHandle,
        private_key: PrivateKey,
        first_device_id: DeviceID,
        first_device_label: DeviceLabel,
        first_device_signing_key: SigningKey,
        initial_profile: &'static PyObject,
        user_realm_id: VlobID,
        user_realm_key: SecretKey,
        local_symkey: SecretKey,
        local_password: Py<PyString>,
        user_certificate: UserCertificate,
        user_raw_redacted_certificate: Py<PyBytes>,
        user_raw_certificate: Py<PyBytes>,
        first_device_certificate: DeviceCertificate,
        first_device_raw_redacted_certificate: Py<PyBytes>,
        first_device_raw_certificate: Py<PyBytes>,
    ],
    |py, x: &TestbedEventNewUser| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user={:?}, profile={}, first_device={:?}",
            x.timestamp.0,
            x.author.0,
            x.user_id.0,
            x.initial_profile.bind(py).repr()?.to_str()?,
            x.first_device_id.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewDevice,
    [
        timestamp: DateTime,
        author: DeviceID,
        user_id: UserID,
        device_id: DeviceID,
        device_label: DeviceLabel,
        signing_key: SigningKey,
        local_symkey: SecretKey,
        local_password: Py<PyString>,
        certificate: DeviceCertificate,
        raw_redacted_certificate: Py<PyBytes>,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventNewDevice| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user={:?}, device={:?}",
            x.timestamp.0, x.author.0, x.user_id.0, x.device_id.0,
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
            x.profile.bind(py).repr()?.to_str()?,
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
        created_by: DeviceID,
        created_on: DateTime,
        token: AccessToken,
    ],
    |_py, x: &TestbedEventNewDeviceInvitation| -> PyResult<String> {
        Ok(format!(
            "created_by={:?}, created_on={:?}, token={:?}",
            x.created_by.0, x.created_on.0, x.token.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewUserInvitation,
    [
        claimer_email: EmailAddress,
        created_by: DeviceID,
        created_on: DateTime,
        token: AccessToken,
    ],
    |_py, x: &TestbedEventNewUserInvitation| -> PyResult<String> {
        Ok(format!(
            "claimer_email={:?}, created_by={:?}, created_on={:?}, token={:?}",
            x.claimer_email.0, x.created_by.0, x.created_on.0, x.token.0,
        ))
    }
);

event_wrapper!(
    TestbedEventNewShamirRecoveryInvitation,
    [
        claimer: UserID,
        created_by: DeviceID,
        created_on: DateTime,
        token: AccessToken,
    ],
    |_py, x: &TestbedEventNewShamirRecoveryInvitation| -> PyResult<String> {
        Ok(format!(
            "claimer={:?}, created_by={:?}, created_on={:?}, token={:?}",
            x.claimer.0, x.created_by.0, x.created_on.0, x.token.0,
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
            &match &x.role {
                None => "None".to_owned(),
                Some(p) => {
                    p.bind(py).repr()?.to_str()?.to_owned()
                },
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
        per_sequester_service_keys_bundle_access: Py<PyAny>,
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
        user_id: UserID,
        threshold: NonZeroU8,
        per_recipient_shares: HashMap<UserID, NonZeroU8>,
        recovery_device: DeviceID,
        data_key: SecretKey,
        reveal_token: AccessToken,
        ciphered_data: Py<PyBytes>,
        brief_certificate: ShamirRecoveryBriefCertificate,
        raw_brief_certificate: Py<PyBytes>,
        shares_certificates: Py<PyList>,
        raw_shares_certificates: Py<PyList>,
    ],
    |_py, x: &TestbedEventNewShamirRecovery| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, user_id={:?}, threshold={:?}, per_recipient_shares={:?}",
            x.timestamp.0,
            x.author.0,
            x.user_id.0,
            x.threshold,
            x.per_recipient_shares.iter().map(|(k, v)| (k.0, v)).collect::<HashMap<_, _>>(),
        ))
    }
);
event_wrapper!(
    TestbedEventDeleteShamirRecovery,
    [
        timestamp: DateTime,
        author: DeviceID,
        setup_to_delete_timestamp: DateTime,
        setup_to_delete_user_id: UserID,
        share_recipients: Vec<UserID>,
        certificate: ShamirRecoveryDeletionCertificate,
        raw_certificate: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventDeleteShamirRecovery| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, setup_to_delete_user_id={:?}, setup_to_delete_timestamp={:?}",
            x.timestamp.0,
            x.author.0,
            x.setup_to_delete_user_id.0,
            x.setup_to_delete_timestamp.0,
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
    ],
    |_py, x: &TestbedEventCreateOrUpdateOpaqueVlob| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, vlob={:?}, key_index={:?}, version={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.vlob_id.0, x.key_index, x.version
        ))
    }
);

event_wrapper!(
    TestbedEventCreateBlock,
    [
        timestamp: DateTime,
        author: DeviceID,
        realm: VlobID,
        block_id: BlockID,
        key_index: libparsec_types::IndexInt,
        cleartext: Py<PyBytes>,
        encrypted: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventCreateBlock| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, block={:?}, key_index={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.block_id.0, x.key_index,
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
        key_index: libparsec_types::IndexInt,
        encrypted: Py<PyBytes>,
    ],
    |_py, x: &TestbedEventCreateOpaqueBlock| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, author={:?}, realm={:?}, block={:?}, key_index={:?}",
            x.timestamp.0, x.author.0, x.realm.0, x.block_id.0, x.key_index,
        ))
    }
);

event_wrapper!(
    TestbedEventFreezeUser,
    [
        user: UserID,
    ],
    |_py, x: &TestbedEventFreezeUser| -> PyResult<String> {
        Ok(format!("user={:?}", x.user.0))
    }
);

event_wrapper!(
    TestbedEventUpdateOrganization,
    [
        timestamp: DateTime,
        is_expired: PyObject,
        active_users_limit: PyObject,
        user_profile_outsider_allowed: PyObject,
        minimum_archiving_period: PyObject,
        tos: PyObject,
    ],
    |py, x: &TestbedEventUpdateOrganization| -> PyResult<String> {
        Ok(format!(
            "timestamp={:?}, is_expired={}, active_users_limit={}, user_profile_outsider_allowed={}, minimum_archiving_period={}, tos={}",
            x.timestamp.0,
            x.is_expired.bind(py).repr()?.to_str()?,
            x.active_users_limit.bind(py).repr()?.to_str()?,
            x.user_profile_outsider_allowed.bind(py).repr()?.to_str()?,
            x.minimum_archiving_period.bind(py).repr()?.to_str()?,
            x.tos.bind(py).repr()?.to_str()?,
        ))
    }
);

#[pyfunction]
pub(crate) fn test_load_testbed_customization<'py>(
    py: Python<'py>,
    template: &'py TestbedTemplateContent,
    customization: Bound<'py, PyBytes>,
) -> PyResult<Bound<'py, PyList>> {
    let template = &template.template;
    let customization = customization.as_bytes();

    // To convert the event to a Python object, we need the template (as the
    // certificate will be generated during conversion).
    // However the template itself must be updated with the customization !
    let customized_template = {
        let mut customized_template = template.clone();

        // 1) Load the events constituting the customization
        let mut customization_events =
            rmp_serde::from_slice::<Vec<libparsec_testbed::TestbedEvent>>(customization)
                .map_err(|err| PyValueError::new_err(err.to_string()))?;

        // 2) Apply the customization to the template
        let customized_template_mut = Arc::make_mut(&mut customized_template);
        customized_template_mut
            .events
            .append(&mut customization_events);

        customized_template
    };

    // 3) Convert the events constituting the customization to Python objects

    let py_events = PyList::empty(py);
    for event in customized_template
        .events
        .iter()
        .skip(template.events.len())
    {
        if let Some(pyobj) = event_to_pyobject(py, &customized_template, event)? {
            py_events.append(pyobj)?;
        }
    }

    Ok(py_events)
}

#[pyfunction]
pub(crate) fn test_get_testbed_template(
    py: Python,
    id: &str,
) -> PyResult<Option<TestbedTemplateContent>> {
    match libparsec_testbed::test_get_template(id) {
        None => Ok(None),
        Some(template) => {
            let py_events = PyList::empty(py);
            for event in template.events.iter() {
                if let Some(pyobj) = event_to_pyobject(py, &template, event)? {
                    py_events.append(pyobj)?;
                }
            }

            Ok(Some(TestbedTemplateContent {
                id: template.id,
                template,
                events: py_events.unbind(),
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
            let raw = PyBytes::new($py, &certif.signed).unbind();
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
            let raw = PyBytes::new($py, &certif.signed).unbind();
            let raw_redacted = PyBytes::new($py, &certif.signed_redacted).unbind();
            let wrapped_certif = paste::paste! {
                [< $name Certificate >]::from(match &certif.certificate {
                    libparsec_types::AnyArcCertificate::$name(x) => x.to_owned(),
                    _ => unreachable!(),
                })
            };
            (wrapped_certif, raw, raw_redacted)
        }};
    }
    match event {
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
                first_user_id: x.first_user_id.into(),
                first_user_first_device_id: x.first_user_first_device_id.into(),
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
                first_user_local_password: PyString::new(py, &x.first_user_local_password).unbind(),
                sequester_authority_certificate: sequester_authority_certif.as_ref().map(|x| {
                    SequesterAuthorityCertificate::from(match &x.certificate {
                        libparsec_types::AnyArcCertificate::SequesterAuthority(x) => x.to_owned(),
                        _ => unreachable!(),
                    })
                }),
                sequester_authority_raw_certificate: sequester_authority_certif
                    .as_ref()
                    .map(|x| PyBytes::new(py, &x.signed).unbind()),
                first_user_certificate: UserCertificate::from(match &user_certif.certificate {
                    libparsec_types::AnyArcCertificate::User(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_user_raw_certificate: PyBytes::new(py, &user_certif.signed).unbind(),
                first_user_raw_redacted_certificate: PyBytes::new(py, &user_certif.signed_redacted)
                    .unbind(),
                first_user_first_device_certificate: DeviceCertificate::from(match &device_certif
                    .certificate
                {
                    libparsec_types::AnyArcCertificate::Device(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_user_first_device_raw_certificate: PyBytes::new(py, &device_certif.signed)
                    .unbind(),
                first_user_first_device_raw_redacted_certificate: PyBytes::new(
                    py,
                    &device_certif.signed_redacted,
                )
                .unbind(),
            };
            obj.into_py_any(py).map(Some)
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
            obj.into_py_any(py).map(Some)
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
            obj.into_py_any(py).map(Some)
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
                author: x.author.into(),
                user_id: x.user_id.into(),
                human_handle: x.human_handle.clone().into(),
                private_key: x.private_key.clone().into(),
                first_device_id: x.first_device_id.into(),
                first_device_label: x.first_device_label.clone().into(),
                first_device_signing_key: x.first_device_signing_key.clone().into(),
                initial_profile: UserProfile::convert(x.initial_profile),
                user_realm_id: x.user_realm_id.into(),
                user_realm_key: x.user_realm_key.clone().into(),
                local_symkey: x.local_symkey.clone().into(),
                local_password: PyString::new(py, &x.local_password).unbind(),
                user_raw_certificate: PyBytes::new(py, &user_certif.signed).unbind(),
                user_raw_redacted_certificate: PyBytes::new(py, &user_certif.signed_redacted)
                    .unbind(),
                user_certificate: UserCertificate::from(match &user_certif.certificate {
                    libparsec_types::AnyArcCertificate::User(x) => x.to_owned(),
                    _ => unreachable!(),
                }),
                first_device_raw_certificate: PyBytes::new(py, &device_certif.signed).unbind(),
                first_device_raw_redacted_certificate: PyBytes::new(
                    py,
                    &device_certif.signed_redacted,
                )
                .unbind(),
                first_device_certificate: DeviceCertificate::from(
                    match &device_certif.certificate {
                        libparsec_types::AnyArcCertificate::Device(x) => x.to_owned(),
                        _ => unreachable!(),
                    },
                ),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::NewDevice(x) => {
            let (certificate, raw_certificate, raw_redacted_certificate) =
                single_certificate_with_redacted!(py, x, template, Device);
            let obj = TestbedEventNewDevice {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                user_id: x.user_id.into(),
                device_id: x.device_id.into(),
                device_label: x.device_label.clone().into(),
                signing_key: x.signing_key.clone().into(),
                local_symkey: x.local_symkey.clone().into(),
                local_password: PyString::new(py, &x.local_password).unbind(),
                raw_certificate,
                raw_redacted_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::UpdateUserProfile(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, UserUpdate);
            let obj = TestbedEventUpdateUserProfile {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                user: x.user.into(),
                profile: UserProfile::convert(x.profile),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::RevokeUser(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RevokedUser);
            let obj = TestbedEventRevokeUser {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                user: x.user.into(),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::NewDeviceInvitation(x) => {
            let obj = TestbedEventNewDeviceInvitation {
                created_by: x.created_by.into(),
                created_on: x.created_on.into(),
                token: x.token.into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::NewUserInvitation(x) => {
            let obj = TestbedEventNewUserInvitation {
                claimer_email: EmailAddress(x.claimer_email.clone()),
                created_by: x.created_by.into(),
                created_on: x.created_on.into(),
                token: x.token.into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::NewShamirRecoveryInvitation(x) => {
            let obj = TestbedEventNewShamirRecoveryInvitation {
                claimer: x.claimer.into(),
                created_by: x.created_by.into(),
                created_on: x.created_on.into(),
                token: x.token.into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::NewRealm(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RealmRole);
            let obj = TestbedEventNewRealm {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm_id: x.realm_id.into(),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::ShareRealm(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RealmRole);
            let obj = TestbedEventShareRealm {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                user: x.user.into(),
                role: x.role.map(RealmRole::convert),
                key_index: x.key_index,
                recipient_keys_bundle_access: x
                    .recipient_keys_bundle_access(template)
                    .map(|x| PyBytes::new(py, &x).unbind()),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::RenameRealm(x) => {
            let (certificate, raw_certificate) = single_certificate!(py, x, template, RealmName);
            let obj = TestbedEventRenameRealm {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::RotateKeyRealm(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, RealmKeyRotation);
            let obj = TestbedEventRotateKeyRealm {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                per_participant_keys_bundle_access: {
                    let pyobj = PyDict::new(py);
                    for (participant, access) in x.per_participant_keys_bundle_access() {
                        pyobj.set_item(UserID::from(participant), PyBytes::new(py, &access))?;
                    }
                    pyobj.unbind()
                },
                per_sequester_service_keys_bundle_access: {
                    match x.per_sequester_service_keys_bundle_access() {
                        None => py.None(),
                        Some(x) => {
                            let pyobj = PyDict::new(py);
                            for (id, access) in x {
                                pyobj.set_item(
                                    SequesterServiceID::from(id),
                                    PyBytes::new(py, &access),
                                )?;
                            }
                            pyobj.into_py_any(py)?
                        }
                    }
                },
                keys_bundle: PyBytes::new(py, &x.keys_bundle(template)).unbind(),
                key_index: x.key_index.into(),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::ArchiveRealm(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, RealmArchiving);
            let obj = TestbedEventArchiveRealm {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                raw_certificate,
                certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::NewShamirRecovery(x) => {
            let mut certifs = x.certificates(template);

            let (brief_certificate, raw_brief_certificate) = {
                let certif = certifs.next().expect("Must be present");
                let raw_brief_certificate = PyBytes::new(py, &certif.signed).unbind();
                let brief_certificate =
                    ShamirRecoveryBriefCertificate::from(match &certif.certificate {
                        libparsec_types::AnyArcCertificate::ShamirRecoveryBrief(x) => x.to_owned(),
                        _ => unreachable!(),
                    });
                (brief_certificate, raw_brief_certificate)
            };

            let (shares_certificates, raw_shares_certificates) = {
                let shares_certificates = PyList::empty(py);
                let raw_shares_certificates = PyList::empty(py);
                for certif in certifs {
                    let raw = PyBytes::new(py, &certif.signed);
                    let wrapped_certif =
                        ShamirRecoveryShareCertificate::from(match &certif.certificate {
                            libparsec_types::AnyArcCertificate::ShamirRecoveryShare(x) => {
                                x.to_owned()
                            }
                            _ => unreachable!(),
                        });
                    shares_certificates.append(wrapped_certif)?;
                    raw_shares_certificates.append(raw)?;
                }
                (shares_certificates, raw_shares_certificates)
            };

            let obj = TestbedEventNewShamirRecovery {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                user_id: x.user_id.into(),
                threshold: x.threshold,
                per_recipient_shares: x
                    .per_recipient_shares
                    .iter()
                    .map(|(k, v)| ((*k).into(), *v))
                    .collect(),
                recovery_device: x.recovery_device.into(),
                data_key: x.data_key.clone().into(),
                reveal_token: x.reveal_token.into(),
                ciphered_data: PyBytes::new(py, &x.ciphered_data(template)).into(),
                brief_certificate,
                raw_brief_certificate,
                shares_certificates: shares_certificates.unbind(),
                raw_shares_certificates: raw_shares_certificates.unbind(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::DeleteShamirRecovery(x) => {
            let (certificate, raw_certificate) =
                single_certificate!(py, x, template, ShamirRecoveryDeletion);

            let obj = TestbedEventDeleteShamirRecovery {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                setup_to_delete_timestamp: x.setup_to_delete_timestamp.into(),
                setup_to_delete_user_id: x.setup_to_delete_user_id.into(),
                share_recipients: x.share_recipients.iter().map(|u| (*u).into()).collect(),
                certificate,
                raw_certificate,
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateOpaqueVlob(x) => {
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                key_index: x.key_index,
                version: x.version,
                vlob_id: x.vlob_id.into(),
                encrypted: PyBytes::new(py, &x.encrypted).into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateUserManifestVlob(x) => {
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.into(),
                realm: x.manifest.id.into(),
                key_index: 0,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateFileManifestVlob(x) => {
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.into(),
                realm: x.realm.into(),
                key_index: x.key_index,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::CreateOrUpdateFolderManifestVlob(x) => {
            let obj = TestbedEventCreateOrUpdateOpaqueVlob {
                timestamp: x.manifest.timestamp.into(),
                author: x.manifest.author.into(),
                realm: x.realm.into(),
                key_index: x.key_index,
                version: x.manifest.version,
                vlob_id: x.manifest.id.into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::CreateOpaqueBlock(x) => {
            let obj = TestbedEventCreateOpaqueBlock {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                block_id: x.block_id.into(),
                key_index: x.key_index,
                encrypted: PyBytes::new(py, &x.encrypted).into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::CreateBlock(x) => {
            let obj = TestbedEventCreateBlock {
                timestamp: x.timestamp.into(),
                author: x.author.into(),
                realm: x.realm.into(),
                block_id: x.block_id.into(),
                key_index: x.key_index,
                cleartext: PyBytes::new(py, &x.cleartext).into(),
                encrypted: PyBytes::new(py, &x.encrypted(template)).into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::FreezeUser(x) => {
            let obj = TestbedEventFreezeUser {
                user: x.user.into(),
            };
            obj.into_py_any(py).map(Some)
        }

        libparsec_testbed::TestbedEvent::UpdateOrganization(x) => {
            let unset_tag = PyModule::import(py, "parsec.types")?.getattr("Unset")?;
            let obj = TestbedEventUpdateOrganization {
                timestamp: x.timestamp.into(),
                is_expired: match x.is_expired {
                    None => unset_tag.clone().unbind(),
                    Some(is_expired) => PyBool::new(py, is_expired).into_py_any(py)?,
                },
                active_users_limit: match x.active_users_limit {
                    None => unset_tag.clone().unbind(),
                    Some(active_users_limit) => {
                        ActiveUsersLimit(active_users_limit).into_py_any(py)?
                    }
                },
                user_profile_outsider_allowed: match x.user_profile_outsider_allowed {
                    None => unset_tag.clone().unbind(),
                    Some(user_profile_outsider_allowed) => {
                        user_profile_outsider_allowed.into_py_any(py)?
                    }
                },
                minimum_archiving_period: match x.minimum_archiving_period {
                    None => unset_tag.clone().unbind(),
                    Some(minimum_archiving_period) => minimum_archiving_period.into_py_any(py)?,
                },
                tos: match &x.tos {
                    None => unset_tag.clone().unbind(),
                    Some(tos) if tos.is_empty() => py.None(),
                    Some(tos) => {
                        let pyobj = PyDict::new(py);
                        for (locale, url) in tos {
                            pyobj.set_item(PyString::new(py, locale), PyString::new(py, url))?;
                        }
                        pyobj.unbind().into_any()
                    }
                },
            };
            obj.into_py_any(py).map(Some)
        }

        // Ignore non-server events
        _ => Ok(None),
    }
}
