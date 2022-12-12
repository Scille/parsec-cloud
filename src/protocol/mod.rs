// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod block;
mod cmds;
mod error;
mod events;
mod invite;
mod message;
mod organization;
mod ping;
mod pki;
mod realm;
mod user;
mod vlob;

pub(crate) use block::*;
pub(crate) use cmds::*;
pub(crate) use error::*;
pub(crate) use events::*;
pub(crate) use invite::*;
pub(crate) use message::*;
pub(crate) use organization::*;
pub(crate) use ping::*;
pub(crate) use pki::*;
pub(crate) use realm::*;
pub(crate) use user::*;
pub(crate) use vlob::*;

use pyo3::{types::PyModule, PyResult, Python};

// We use this type because we can't match Option<String> in macro_rules
pub(crate) type Reason = Option<String>;
pub(crate) type Bytes = Vec<u8>;
pub(crate) type ListOfBytes = Vec<Vec<u8>>;
pub(crate) type OptionalFloat = Option<f64>;
pub(crate) type OptionalDateTime = Option<crate::time::DateTime>;

macro_rules! rs_to_py {
    ($v: ident, Reason, $py: ident) => {
        $v.as_ref().map(|x| ::pyo3::types::PyString::new($py, x))
    };
    ($v: ident, String, $py: ident) => {
        ::pyo3::types::PyString::new($py, $v)
    };

    ($v: ident, Bytes, $py: ident) => {
        ::pyo3::types::PyBytes::new($py, $v)
    };
    ($v: ident, ListOfBytes, $py: ident) => {
        ::pyo3::types::PyTuple::new($py, $v.iter().map(|x| ::pyo3::types::PyBytes::new($py, x)))
    };
    ($v: ident, DateTime, $py: ident) => {
        DateTime(*$v)
    };
    ($v: ident, SequesterServiceID, $py: ident) => {
        SequesterServiceID(*$v)
    };
    ($v: ident, OptionalDateTime, $py: ident) => {
        match *$v {
            libparsec::types::Maybe::Present(v) => Some(DateTime(v)),
            libparsec::types::Maybe::Absent => None,
        }
    };
    ($v: ident, OptionalFloat, $py: ident) => {
        match *$v {
            libparsec::types::Maybe::Present(v) => Some(v),
            libparsec::types::Maybe::Absent => None,
        }
    };
    ($v: ident, $ty: ty, $py: ident) => {
        *$v
    };
}

macro_rules! py_to_rs {
    ($v: ident, Reason) => {
        $v
    };
    ($v: ident, Bytes) => {
        $v
    };
    ($v: ident, ListOfBytes) => {
        $v
    };
    ($v: ident, f64) => {
        $v
    };
    ($v: ident, String) => {
        $v
    };
    ($v: ident, OptionalFloat) => {
        match $v {
            Some(v) => libparsec::types::Maybe::Present(v),
            None => libparsec::types::Maybe::Absent,
        }
    };
    ($v: ident, OptionalDateTime) => {
        match $v {
            Some(v) => libparsec::types::Maybe::Present(v.0),
            None => libparsec::types::Maybe::Absent,
        }
    };
    ($v: ident, $ty: ty) => {
        $v.0
    };
}

macro_rules! rs_to_py_ty {
    (Reason) => {
        Option<&'py ::pyo3::types::PyString>
    };
    (String) => {
        &'py ::pyo3::types::PyString
    };
    (Bytes) => {
        &'py ::pyo3::types::PyBytes
    };
    (ListOfBytes) => {
        &'py ::pyo3::types::PyTuple
    };
    ($ty: ty) => {
        $ty
    };
}

macro_rules! gen_rep {
    (
        $mod: path,
        $base_class: ident
        $(, { $($tt: tt)+ })?
        $(, [$variant: ident $($(, $field: ident : $ty: ty)+)? $(,)?])*
        $(,)?
    ) => {
        paste::paste! {
            #[::pyo3::pyclass(subclass)]
            #[derive(Clone)]
            pub(crate) struct $base_class(pub $mod::Rep);

            crate::binding_utils::gen_proto!($base_class, __repr__);
            crate::binding_utils::gen_proto!($base_class, __richcmp__, eq);

            #[::pyo3::pymethods]
            impl $base_class {
                fn dump<'py>(&self, py: ::pyo3::Python<'py>) -> ProtocolResult<&'py ::pyo3::types::PyBytes> {
                    self.0.clone().dump()
                        .map(|bytes| ::pyo3::types::PyBytes::new(py, bytes.as_slice()))
                        .map_err(|e| ProtocolErrorFields(libparsec::protocol::ProtocolError::EncodingError { exc: e.to_string() }))
                }

                #[classmethod]
                fn load<'py>(_cls: &::pyo3::types::PyType, buf: Vec<u8>, py: Python<'py>) -> PyResult<PyObject> {
                    use pyo3::{pyclass_init::PyObjectInit, PyTypeInfo};

                    let rep = $mod::Rep::load(&buf)
                        .map_err(|e| ProtocolErrorFields(libparsec::protocol::ProtocolError::DecodingError { exc: e.to_string() }))
                        .map_err(|e| ProtocolError::new_err(e))?;

                    let ret = match rep {
                        $mod::Rep::Ok $({ $($tt)+ })? => {
                            crate::binding_utils::py_object!(rep, [<$base_class Ok>], py)
                        }
                        $(
                            $mod::Rep::$variant $({ $($field: _,)+ })? => {
                                crate::binding_utils::py_object!(rep, [<$base_class $variant>], py)
                            }
                        )*
                        $mod::Rep::UnknownStatus { .. } => {
                            crate::binding_utils::py_object!(rep, [<$base_class UnknownStatus>], py)
                        },
                    };

                    Ok(match _cls.getattr("_post_load") {
                        Ok(post_load) => post_load.call1((ret.as_ref(py), ))?.into_py(py),
                        _ => ret,
                    })
                }
            }

            $(
                #[::pyo3::pyclass(extends=$base_class)]
                pub(crate) struct [<$base_class $variant>];

                #[::pyo3::pymethods]
                impl [<$base_class $variant>] {
                    #[new]
                    fn new($($($field: $ty,)+)?) -> ::pyo3::PyResult<(Self, $base_class)> {
                        $($( let $field = crate::protocol::py_to_rs!($field, $ty); )+)?
                        Ok((Self, $base_class($mod::Rep::$variant $({ $($field, )+ })?)))
                    }

                    $($(
                        #[getter]
                        fn $field<'py>(
                            _self: ::pyo3::PyRef<'py, Self>,
                            _py: ::pyo3::Python<'py>
                        ) -> ::pyo3::PyResult<crate::protocol::rs_to_py_ty!($ty)> {
                            Ok(match &_self.as_ref().0 {
                                $mod::Rep::$variant { $field, .. } => crate::protocol::rs_to_py!($field, $ty, _py),
                                _ => return Err(::pyo3::exceptions::PyNotImplementedError::new_err("")),
                            })
                        }
                    )+)?
                }
            )*

            #[::pyo3::pyclass(extends=$base_class)]
            pub(crate) struct [<$base_class UnknownStatus>];

            #[::pyo3::pymethods]
            impl [<$base_class UnknownStatus>] {
                #[getter]
                fn status<'py>(
                    _self: ::pyo3::PyRef<'py, Self>,
                    py: ::pyo3::Python<'py>
                ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
                    Ok(match &_self.as_ref().0 {
                        $mod::Rep::UnknownStatus { unknown_status, .. } => ::pyo3::types::PyString::new(py, unknown_status),
                        _ => return Err(::pyo3::exceptions::PyNotImplementedError::new_err("")),
                    })
                }

                #[getter]
                fn reason<'py>(
                    _self: ::pyo3::PyRef<'py, Self>,
                    py: ::pyo3::Python<'py>
                ) -> ::pyo3::PyResult<Option<&'py ::pyo3::types::PyString>> {
                    Ok(match &_self.as_ref().0 {
                        $mod::Rep::UnknownStatus { reason, .. } => reason.as_ref().map(|x| ::pyo3::types::PyString::new(py, x)),
                        _ => return Err(::pyo3::exceptions::PyNotImplementedError::new_err("")),
                    })
                }
            }
        }
    };
}

pub(crate) use gen_rep;
use py_to_rs;
use rs_to_py;
use rs_to_py_ty;

pub(crate) fn add_mod(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Block
    m.add_class::<BlockCreateReq>()?;
    m.add_class::<BlockCreateRep>()?;
    m.add_class::<BlockCreateRepOk>()?;
    m.add_class::<BlockCreateRepAlreadyExists>()?;
    m.add_class::<BlockCreateRepNotFound>()?;
    m.add_class::<BlockCreateRepTimeout>()?;
    m.add_class::<BlockCreateRepNotAllowed>()?;
    m.add_class::<BlockCreateRepInMaintenance>()?;
    m.add_class::<BlockCreateRepUnknownStatus>()?;
    m.add_class::<BlockReadReq>()?;
    m.add_class::<BlockReadRep>()?;
    m.add_class::<BlockReadRepOk>()?;
    m.add_class::<BlockReadRepNotFound>()?;
    m.add_class::<BlockReadRepTimeout>()?;
    m.add_class::<BlockReadRepNotAllowed>()?;
    m.add_class::<BlockReadRepInMaintenance>()?;
    m.add_class::<BlockReadRepUnknownStatus>()?;

    // Events
    m.add_class::<EventsListenReq>()?;
    m.add_class::<EventsListenRep>()?;
    m.add_class::<EventsListenRepOk>()?;
    m.add_class::<EventsListenRepNoEvents>()?;
    m.add_class::<EventsListenRepUnknownStatus>()?;
    m.add_class::<EventsListenRepCancelled>()?;
    m.add_class::<EventsListenRepOkPinged>()?;
    m.add_class::<EventsListenRepOkMessageReceived>()?;
    m.add_class::<EventsListenRepOkInviteStatusChanged>()?;
    m.add_class::<EventsListenRepOkRealmMaintenanceFinished>()?;
    m.add_class::<EventsListenRepOkRealmMaintenanceStarted>()?;
    m.add_class::<EventsListenRepOkRealmVlobsUpdated>()?;
    m.add_class::<EventsListenRepOkRealmRolesUpdated>()?;
    m.add_class::<EventsListenRepOkPkiEnrollmentUpdated>()?;
    m.add_class::<EventsSubscribeReq>()?;
    m.add_class::<EventsSubscribeRep>()?;
    m.add_class::<EventsSubscribeRepOk>()?;
    m.add_class::<EventsSubscribeRepUnknownStatus>()?;

    // Message
    m.add_class::<MessageGetReq>()?;
    m.add_class::<MessageGetRep>()?;
    m.add_class::<MessageGetRepOk>()?;
    m.add_class::<MessageGetRepUnknownStatus>()?;
    m.add_class::<Message>()?;

    // Pki commands
    m.add_class::<PkiEnrollmentAcceptReq>()?;
    m.add_class::<PkiEnrollmentAcceptRep>()?;
    m.add_class::<PkiEnrollmentAcceptRepOk>()?;
    m.add_class::<PkiEnrollmentAcceptRepInvalidPayloadData>()?;
    m.add_class::<PkiEnrollmentAcceptRepInvalidCertification>()?;
    m.add_class::<PkiEnrollmentAcceptRepInvalidData>()?;
    m.add_class::<PkiEnrollmentAcceptRepNotAllowed>()?;
    m.add_class::<PkiEnrollmentAcceptRepNotFound>()?;
    m.add_class::<PkiEnrollmentAcceptRepNoLongerAvailable>()?;
    m.add_class::<PkiEnrollmentAcceptRepAlreadyExists>()?;
    m.add_class::<PkiEnrollmentAcceptRepActiveUsersLimitReached>()?;
    m.add_class::<PkiEnrollmentInfoReq>()?;
    m.add_class::<PkiEnrollmentInfoRep>()?;
    m.add_class::<PkiEnrollmentInfoRepOk>()?;
    m.add_class::<PkiEnrollmentInfoRepNotFound>()?;
    m.add_class::<PkiEnrollmentInfoStatus>()?;
    m.add_class::<PkiEnrollmentListReq>()?;
    m.add_class::<PkiEnrollmentListItem>()?;
    m.add_class::<PkiEnrollmentListRep>()?;
    m.add_class::<PkiEnrollmentListRepOk>()?;
    m.add_class::<PkiEnrollmentListRepNotAllowed>()?;
    m.add_class::<PkiEnrollmentRejectReq>()?;
    m.add_class::<PkiEnrollmentRejectRep>()?;
    m.add_class::<PkiEnrollmentRejectRepOk>()?;
    m.add_class::<PkiEnrollmentRejectRepNotAllowed>()?;
    m.add_class::<PkiEnrollmentRejectRepNotFound>()?;
    m.add_class::<PkiEnrollmentRejectRepNoLongerAvailable>()?;
    m.add_class::<PkiEnrollmentAcceptRepUnknownStatus>()?;
    m.add_class::<PkiEnrollmentInfoRepUnknownStatus>()?;
    m.add_class::<PkiEnrollmentListRepUnknownStatus>()?;
    m.add_class::<PkiEnrollmentRejectRepUnknownStatus>()?;
    m.add_class::<PkiEnrollmentSubmitReq>()?;
    m.add_class::<PkiEnrollmentSubmitRep>()?;
    m.add_class::<PkiEnrollmentSubmitRepOk>()?;
    m.add_class::<PkiEnrollmentSubmitRepInvalidPayloadData>()?;
    m.add_class::<PkiEnrollmentSubmitRepAlreadySubmitted>()?;
    m.add_class::<PkiEnrollmentSubmitRepIdAlreadyUsed>()?;
    m.add_class::<PkiEnrollmentSubmitRepEmailAlreadyUsed>()?;
    m.add_class::<PkiEnrollmentSubmitRepAlreadyEnrolled>()?;
    m.add_class::<PkiEnrollmentSubmitRepUnknownStatus>()?;
    m.add_class::<PkiEnrollmentStatus>()?;

    // Organization
    m.add_class::<OrganizationStatsReq>()?;
    m.add_class::<OrganizationStatsRep>()?;
    m.add_class::<OrganizationStatsRepOk>()?;
    m.add_class::<OrganizationStatsRepNotAllowed>()?;
    m.add_class::<OrganizationStatsRepNotFound>()?;
    m.add_class::<OrganizationStatsRepUnknownStatus>()?;
    m.add_class::<OrganizationConfigReq>()?;
    m.add_class::<OrganizationConfigRep>()?;
    m.add_class::<OrganizationConfigRepOk>()?;
    m.add_class::<OrganizationConfigRepNotFound>()?;
    m.add_class::<OrganizationConfigRepUnknownStatus>()?;
    m.add_class::<UsersPerProfileDetailItem>()?;

    // Realm
    m.add_class::<RealmCreateReq>()?;
    m.add_class::<RealmCreateRep>()?;
    m.add_class::<RealmCreateRepOk>()?;
    m.add_class::<RealmCreateRepInvalidCertification>()?;
    m.add_class::<RealmCreateRepInvalidData>()?;
    m.add_class::<RealmCreateRepNotFound>()?;
    m.add_class::<RealmCreateRepAlreadyExists>()?;
    m.add_class::<RealmCreateRepBadTimestamp>()?;
    m.add_class::<RealmCreateRepUnknownStatus>()?;
    m.add_class::<RealmStatusReq>()?;
    m.add_class::<RealmStatusRep>()?;
    m.add_class::<RealmStatusRepOk>()?;
    m.add_class::<RealmStatusRepNotAllowed>()?;
    m.add_class::<RealmStatusRepNotFound>()?;
    m.add_class::<RealmStatusRepUnknownStatus>()?;
    m.add_class::<RealmStatsReq>()?;
    m.add_class::<RealmStatsRep>()?;
    m.add_class::<RealmStatsRepOk>()?;
    m.add_class::<RealmStatsRepNotAllowed>()?;
    m.add_class::<RealmStatsRepNotFound>()?;
    m.add_class::<RealmStatsRepUnknownStatus>()?;
    m.add_class::<RealmGetRoleCertificatesReq>()?;
    m.add_class::<RealmGetRoleCertificatesRep>()?;
    m.add_class::<RealmGetRoleCertificatesRepOk>()?;
    m.add_class::<RealmGetRoleCertificatesRepNotAllowed>()?;
    m.add_class::<RealmGetRoleCertificatesRepNotFound>()?;
    m.add_class::<RealmGetRoleCertificatesRepUnknownStatus>()?;
    m.add_class::<RealmUpdateRolesReq>()?;
    m.add_class::<RealmUpdateRolesRep>()?;
    m.add_class::<RealmUpdateRolesRepOk>()?;
    m.add_class::<RealmUpdateRolesRepNotAllowed>()?;
    m.add_class::<RealmUpdateRolesRepInvalidCertification>()?;
    m.add_class::<RealmUpdateRolesRepInvalidData>()?;
    m.add_class::<RealmUpdateRolesRepAlreadyGranted>()?;
    m.add_class::<RealmUpdateRolesRepIncompatibleProfile>()?;
    m.add_class::<RealmUpdateRolesRepNotFound>()?;
    m.add_class::<RealmUpdateRolesRepInMaintenance>()?;
    m.add_class::<RealmUpdateRolesRepUserRevoked>()?;
    m.add_class::<RealmUpdateRolesRepRequireGreaterTimestamp>()?;
    m.add_class::<RealmUpdateRolesRepBadTimestamp>()?;
    m.add_class::<RealmUpdateRolesRepUnknownStatus>()?;
    m.add_class::<RealmStartReencryptionMaintenanceReq>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRep>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepOk>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepNotAllowed>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepNotFound>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepBadEncryptionRevision>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepParticipantMismatch>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepMaintenanceError>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepInMaintenance>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepBadTimestamp>()?;
    m.add_class::<RealmStartReencryptionMaintenanceRepUnknownStatus>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceReq>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRep>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepOk>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepNotAllowed>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepNotFound>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepBadEncryptionRevision>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepNotInMaintenance>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepMaintenanceError>()?;
    m.add_class::<RealmFinishReencryptionMaintenanceRepUnknownStatus>()?;
    m.add_class::<MaintenanceType>()?;

    // Ping
    m.add_class::<AuthenticatedPingReq>()?;
    m.add_class::<AuthenticatedPingRep>()?;
    m.add_class::<AuthenticatedPingRepOk>()?;
    m.add_class::<AuthenticatedPingRepUnknownStatus>()?;
    m.add_class::<InvitedPingReq>()?;
    m.add_class::<InvitedPingRep>()?;
    m.add_class::<InvitedPingRepOk>()?;
    m.add_class::<InvitedPingRepUnknownStatus>()?;

    // Invite
    m.add_class::<InviteNewReq>()?;
    m.add_class::<InviteNewRep>()?;
    m.add_class::<InviteNewRepNotAllowed>()?;
    m.add_class::<InviteNewRepAlreadyMember>()?;
    m.add_class::<InviteNewRepNotAvailable>()?;
    m.add_class::<InviteNewRepUnknownStatus>()?;
    m.add_class::<InviteNewRepOk>()?;

    m.add_class::<InviteDeleteReq>()?;
    m.add_class::<InviteDeleteRep>()?;
    m.add_class::<InviteDeleteRepAlreadyDeleted>()?;
    m.add_class::<InviteDeleteRepNotFound>()?;
    m.add_class::<InviteDeleteRepUnknownStatus>()?;
    m.add_class::<InviteDeleteRepOk>()?;

    m.add_class::<InviteListReq>()?;
    m.add_class::<InviteListRep>()?;
    m.add_class::<InviteListRepUnknownStatus>()?;
    m.add_class::<InviteListRepOk>()?;

    m.add_class::<InviteInfoReq>()?;
    m.add_class::<InviteInfoRep>()?;
    m.add_class::<InviteInfoRepOk>()?;
    m.add_class::<InviteInfoRepUnknownStatus>()?;

    m.add_class::<Invite1ClaimerWaitPeerReq>()?;
    m.add_class::<Invite1ClaimerWaitPeerRep>()?;
    m.add_class::<Invite1ClaimerWaitPeerRepInvalidState>()?;
    m.add_class::<Invite1ClaimerWaitPeerRepNotFound>()?;
    m.add_class::<Invite1ClaimerWaitPeerRepUnknownStatus>()?;
    m.add_class::<Invite1ClaimerWaitPeerRepOk>()?;

    m.add_class::<Invite1GreeterWaitPeerReq>()?;
    m.add_class::<Invite1GreeterWaitPeerRep>()?;
    m.add_class::<Invite1GreeterWaitPeerRepNotFound>()?;
    m.add_class::<Invite1GreeterWaitPeerRepAlreadyDeleted>()?;
    m.add_class::<Invite1GreeterWaitPeerRepInvalidState>()?;
    m.add_class::<Invite1GreeterWaitPeerRepUnknownStatus>()?;
    m.add_class::<Invite1GreeterWaitPeerRepOk>()?;

    m.add_class::<Invite2aClaimerSendHashedNonceReq>()?;
    m.add_class::<Invite2aClaimerSendHashedNonceRep>()?;
    m.add_class::<Invite2aClaimerSendHashedNonceRepNotFound>()?;
    m.add_class::<Invite2aClaimerSendHashedNonceRepAlreadyDeleted>()?;
    m.add_class::<Invite2aClaimerSendHashedNonceRepInvalidState>()?;
    m.add_class::<Invite2aClaimerSendHashedNonceRepUnknownStatus>()?;
    m.add_class::<Invite2aClaimerSendHashedNonceRepOk>()?;

    m.add_class::<Invite2aGreeterGetHashedNonceReq>()?;
    m.add_class::<Invite2aGreeterGetHashedNonceRepUnknownStatus>()?;
    m.add_class::<Invite2aGreeterGetHashedNonceRep>()?;
    m.add_class::<Invite2aGreeterGetHashedNonceRepNotFound>()?;
    m.add_class::<Invite2aGreeterGetHashedNonceRepAlreadyDeleted>()?;
    m.add_class::<Invite2aGreeterGetHashedNonceRepInvalidState>()?;
    m.add_class::<Invite2aGreeterGetHashedNonceRepOk>()?;

    m.add_class::<Invite2bClaimerSendNonceReq>()?;
    m.add_class::<Invite2bClaimerSendNonceRepUnknownStatus>()?;
    m.add_class::<Invite2bClaimerSendNonceRep>()?;
    m.add_class::<Invite2bClaimerSendNonceRepNotFound>()?;
    m.add_class::<Invite2bClaimerSendNonceRepInvalidState>()?;
    m.add_class::<Invite2bClaimerSendNonceRepOk>()?;

    m.add_class::<Invite2bGreeterSendNonceReq>()?;
    m.add_class::<Invite2bGreeterSendNonceRep>()?;
    m.add_class::<Invite2bGreeterSendNonceRepNotFound>()?;
    m.add_class::<Invite2bGreeterSendNonceRepAlreadyDeleted>()?;
    m.add_class::<Invite2bGreeterSendNonceRepInvalidState>()?;
    m.add_class::<Invite2bGreeterSendNonceRepUnknownStatus>()?;
    m.add_class::<Invite2bGreeterSendNonceRepOk>()?;

    m.add_class::<Invite3aClaimerSignifyTrustReq>()?;
    m.add_class::<Invite3aClaimerSignifyTrustRep>()?;
    m.add_class::<Invite3aClaimerSignifyTrustRepNotFound>()?;
    m.add_class::<Invite3aClaimerSignifyTrustRepInvalidState>()?;
    m.add_class::<Invite3aClaimerSignifyTrustRepUnknownStatus>()?;
    m.add_class::<Invite3aClaimerSignifyTrustRepOk>()?;

    m.add_class::<Invite3aGreeterWaitPeerTrustReq>()?;
    m.add_class::<Invite3aGreeterWaitPeerTrustRep>()?;
    m.add_class::<Invite3aGreeterWaitPeerTrustRepNotFound>()?;
    m.add_class::<Invite3aGreeterWaitPeerTrustRepAlreadyDeleted>()?;
    m.add_class::<Invite3aGreeterWaitPeerTrustRepInvalidState>()?;
    m.add_class::<Invite3aGreeterWaitPeerTrustRepUnknownStatus>()?;
    m.add_class::<Invite3aGreeterWaitPeerTrustRepOk>()?;

    m.add_class::<Invite3bClaimerWaitPeerTrustReq>()?;
    m.add_class::<Invite3bClaimerWaitPeerTrustRepUnknownStatus>()?;
    m.add_class::<Invite3bClaimerWaitPeerTrustRepOk>()?;
    m.add_class::<Invite3bClaimerWaitPeerTrustRep>()?;
    m.add_class::<Invite3bClaimerWaitPeerTrustRepNotFound>()?;
    m.add_class::<Invite3bClaimerWaitPeerTrustRepInvalidState>()?;

    m.add_class::<Invite3bGreeterSignifyTrustReq>()?;
    m.add_class::<Invite3bGreeterSignifyTrustRep>()?;
    m.add_class::<Invite3bGreeterSignifyTrustRepNotFound>()?;
    m.add_class::<Invite3bGreeterSignifyTrustRepAlreadyDeleted>()?;
    m.add_class::<Invite3bGreeterSignifyTrustRepInvalidState>()?;
    m.add_class::<Invite3bGreeterSignifyTrustRepUnknownStatus>()?;
    m.add_class::<Invite3bGreeterSignifyTrustRepOk>()?;

    m.add_class::<Invite4ClaimerCommunicateReq>()?;
    m.add_class::<Invite4ClaimerCommunicateRep>()?;
    m.add_class::<Invite4ClaimerCommunicateRepNotFound>()?;
    m.add_class::<Invite4ClaimerCommunicateRepInvalidState>()?;
    m.add_class::<Invite4ClaimerCommunicateRepUnknownStatus>()?;
    m.add_class::<Invite4ClaimerCommunicateRepOk>()?;

    m.add_class::<Invite4GreeterCommunicateReq>()?;
    m.add_class::<Invite4GreeterCommunicateRep>()?;
    m.add_class::<Invite4GreeterCommunicateRepNotFound>()?;
    m.add_class::<Invite4GreeterCommunicateRepAlreadyDeleted>()?;
    m.add_class::<Invite4GreeterCommunicateRepInvalidState>()?;
    m.add_class::<Invite4GreeterCommunicateRepUnknownStatus>()?;
    m.add_class::<Invite4GreeterCommunicateRepOk>()?;
    m.add_class::<InviteListItem>()?;

    // User
    m.add_class::<UserGetReq>()?;
    m.add_class::<UserGetRep>()?;
    m.add_class::<UserGetRepOk>()?;
    m.add_class::<UserGetRepNotFound>()?;
    m.add_class::<UserGetRepUnknownStatus>()?;
    m.add_class::<UserCreateReq>()?;
    m.add_class::<UserCreateRep>()?;
    m.add_class::<UserCreateRepOk>()?;
    m.add_class::<UserCreateRepActiveUsersLimitReached>()?;
    m.add_class::<UserCreateRepAlreadyExists>()?;
    m.add_class::<UserCreateRepInvalidCertification>()?;
    m.add_class::<UserCreateRepInvalidData>()?;
    m.add_class::<UserCreateRepNotAllowed>()?;
    m.add_class::<UserCreateRepUnknownStatus>()?;
    m.add_class::<UserRevokeReq>()?;
    m.add_class::<UserRevokeRep>()?;
    m.add_class::<UserRevokeRepOk>()?;
    m.add_class::<UserRevokeRepAlreadyRevoked>()?;
    m.add_class::<UserRevokeRepNotFound>()?;
    m.add_class::<UserRevokeRepInvalidCertification>()?;
    m.add_class::<UserRevokeRepNotAllowed>()?;
    m.add_class::<UserRevokeRepUnknownStatus>()?;
    m.add_class::<DeviceCreateReq>()?;
    m.add_class::<DeviceCreateRep>()?;
    m.add_class::<DeviceCreateRepOk>()?;
    m.add_class::<DeviceCreateRepAlreadyExists>()?;
    m.add_class::<DeviceCreateRepBadUserId>()?;
    m.add_class::<DeviceCreateRepInvalidCertification>()?;
    m.add_class::<DeviceCreateRepInvalidData>()?;
    m.add_class::<DeviceCreateRepUnknownStatus>()?;
    m.add_class::<HumanFindReq>()?;
    m.add_class::<HumanFindRep>()?;
    m.add_class::<HumanFindRepOk>()?;
    m.add_class::<HumanFindRepNotAllowed>()?;
    m.add_class::<HumanFindRepUnknownStatus>()?;
    m.add_class::<Trustchain>()?;
    m.add_class::<HumanFindResultItem>()?;

    // Vlob
    m.add_class::<VlobCreateReq>()?;
    m.add_class::<VlobCreateRep>()?;
    m.add_class::<VlobCreateRepOk>()?;
    m.add_class::<VlobCreateRepAlreadyExists>()?;
    m.add_class::<VlobCreateRepNotAllowed>()?;
    m.add_class::<VlobCreateRepBadEncryptionRevision>()?;
    m.add_class::<VlobCreateRepInMaintenance>()?;
    m.add_class::<VlobCreateRepRequireGreaterTimestamp>()?;
    m.add_class::<VlobCreateRepBadTimestamp>()?;
    m.add_class::<VlobCreateRepNotASequesteredOrganization>()?;
    m.add_class::<VlobCreateRepSequesterInconsistency>()?;
    m.add_class::<VlobCreateRepRejectedBySequesterService>()?;
    m.add_class::<VlobCreateRepTimeout>()?;
    m.add_class::<VlobCreateRepUnknownStatus>()?;
    m.add_class::<VlobReadReq>()?;
    m.add_class::<VlobReadRep>()?;
    m.add_class::<VlobReadRepOk>()?;
    m.add_class::<VlobReadRepNotFound>()?;
    m.add_class::<VlobReadRepNotAllowed>()?;
    m.add_class::<VlobReadRepBadVersion>()?;
    m.add_class::<VlobReadRepBadEncryptionRevision>()?;
    m.add_class::<VlobReadRepInMaintenance>()?;
    m.add_class::<VlobReadRepUnknownStatus>()?;
    m.add_class::<VlobUpdateReq>()?;
    m.add_class::<VlobUpdateRep>()?;
    m.add_class::<VlobUpdateRepOk>()?;
    m.add_class::<VlobUpdateRepNotFound>()?;
    m.add_class::<VlobUpdateRepNotAllowed>()?;
    m.add_class::<VlobUpdateRepBadVersion>()?;
    m.add_class::<VlobUpdateRepBadEncryptionRevision>()?;
    m.add_class::<VlobUpdateRepInMaintenance>()?;
    m.add_class::<VlobUpdateRepRequireGreaterTimestamp>()?;
    m.add_class::<VlobUpdateRepRequireGreaterTimestamp>()?;
    m.add_class::<VlobUpdateRepBadTimestamp>()?;
    m.add_class::<VlobUpdateRepNotASequesteredOrganization>()?;
    m.add_class::<VlobUpdateRepSequesterInconsistency>()?;
    m.add_class::<VlobUpdateRepRejectedBySequesterService>()?;
    m.add_class::<VlobUpdateRepTimeout>()?;
    m.add_class::<VlobUpdateRepUnknownStatus>()?;
    m.add_class::<VlobPollChangesReq>()?;
    m.add_class::<VlobPollChangesRep>()?;
    m.add_class::<VlobPollChangesRepOk>()?;
    m.add_class::<VlobPollChangesRepNotFound>()?;
    m.add_class::<VlobPollChangesRepNotAllowed>()?;
    m.add_class::<VlobPollChangesRepInMaintenance>()?;
    m.add_class::<VlobPollChangesRepUnknownStatus>()?;
    m.add_class::<VlobListVersionsReq>()?;
    m.add_class::<VlobListVersionsRep>()?;
    m.add_class::<VlobListVersionsRepOk>()?;
    m.add_class::<VlobListVersionsRepNotFound>()?;
    m.add_class::<VlobListVersionsRepNotAllowed>()?;
    m.add_class::<VlobListVersionsRepInMaintenance>()?;
    m.add_class::<VlobListVersionsRepUnknownStatus>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchReq>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRep>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepOk>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepNotFound>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepNotAllowed>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepNotInMaintenance>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepMaintenanceError>()?;
    m.add_class::<VlobMaintenanceGetReencryptionBatchRepUnknownStatus>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchReq>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRep>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepOk>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepNotFound>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepNotAllowed>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepMaintenanceError>()?;
    m.add_class::<VlobMaintenanceSaveReencryptionBatchRepUnknownStatus>()?;
    m.add_class::<ReencryptionBatchEntry>()?;

    // Cmd
    m.add_class::<AuthenticatedAnyCmdReq>()?;
    m.add_class::<InvitedAnyCmdReq>()?;
    m.add_class::<AnonymousAnyCmdReq>()?;

    // Error type
    m.add_class::<error::ProtocolErrorFields>()?;
    m.add("ProtocolError", py.get_type::<error::ProtocolError>())?;

    Ok(())
}
