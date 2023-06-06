// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python},
    AsPyPointer, FromPyPointer, IntoPyPointer,
};

mod addrs;
mod api_crypto;
mod backend_events;
mod binding_utils;
mod data;
mod enumerate;
mod ids;
#[cfg(feature = "test-utils")]
mod local_db;
mod misc;
mod protocol;
mod regex;
mod runtime;
#[cfg(feature = "test-utils")]
mod testbed;
mod time;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(py: Python, m: &PyModule) -> PyResult<()> {
    crate::api_crypto::add_mod(py, m)?;
    crate::data::add_mod(py, m)?;
    #[cfg(feature = "test-utils")]
    crate::local_db::add_mod(py, m)?;
    crate::protocol::add_mod(py, m)?;

    patch_panic_exception_to_inherit_exception(py);
    // Useful to expose `PanicException` for testing
    m.add(
        "PanicException",
        <pyo3::panic::PanicException as pyo3::PyTypeInfo>::type_object(py),
    )?;

    m.add_class::<backend_events::BackendEvent>()?;
    m.add_class::<backend_events::BackendEventCertificatesUpdated>()?;
    m.add_class::<backend_events::BackendEventInviteConduitUpdated>()?;
    m.add_class::<backend_events::BackendEventUserUpdatedOrRevoked>()?;
    m.add_class::<backend_events::BackendEventOrganizationExpired>()?;
    m.add_class::<backend_events::BackendEventPinged>()?;
    m.add_class::<backend_events::BackendEventMessageReceived>()?;
    m.add_class::<backend_events::BackendEventInviteStatusChanged>()?;
    m.add_class::<backend_events::BackendEventRealmMaintenanceFinished>()?;
    m.add_class::<backend_events::BackendEventRealmMaintenanceStarted>()?;
    m.add_class::<backend_events::BackendEventRealmVlobsUpdated>()?;
    m.add_class::<backend_events::BackendEventRealmRolesUpdated>()?;
    m.add_class::<backend_events::BackendEventPkiEnrollmentUpdated>()?;

    m.add_class::<addrs::BackendAddr>()?;
    m.add_class::<addrs::BackendOrganizationAddr>()?;
    m.add_class::<addrs::BackendActionAddr>()?;
    m.add_class::<addrs::BackendOrganizationBootstrapAddr>()?;
    m.add_class::<addrs::BackendOrganizationFileLinkAddr>()?;
    m.add_class::<addrs::BackendInvitationAddr>()?;
    m.add_class::<addrs::BackendPkiEnrollmentAddr>()?;
    m.add_function(wrap_pyfunction!(addrs::export_root_verify_key, m)?)?;

    m.add_class::<enumerate::DeviceFileType>()?;
    m.add_class::<enumerate::InvitationStatus>()?;
    m.add_class::<enumerate::InvitationType>()?;
    m.add_class::<enumerate::RealmRole>()?;
    m.add_class::<enumerate::UserProfile>()?;

    m.add_class::<ids::OrganizationID>()?;
    m.add_class::<ids::EntryID>()?;
    m.add_class::<ids::BlockID>()?;
    m.add_class::<ids::RealmID>()?;
    m.add_class::<ids::VlobID>()?;
    m.add_class::<ids::ChunkID>()?;
    m.add_class::<ids::SequesterServiceID>()?;
    m.add_class::<ids::EnrollmentID>()?;
    m.add_class::<ids::HumanHandle>()?;
    m.add_class::<ids::DeviceID>()?;
    m.add_class::<ids::DeviceName>()?;
    m.add_class::<ids::DeviceLabel>()?;
    m.add_class::<ids::UserID>()?;
    m.add_class::<ids::InvitationToken>()?;

    // Time
    m.add_function(wrap_pyfunction!(time::mock_time, m)?)?;
    m.add_class::<time::TimeProvider>()?;
    m.add_class::<time::DateTime>()?;
    m.add_class::<time::LocalDateTime>()?;

    // Regex
    m.add_class::<regex::Regex>()?;

    // Registering ABC classes
    m.add_class::<runtime::FutureIntoCoroutine>()?;
    let future_into_coroutine_cls = m.getattr("FutureIntoCoroutine")?;
    py.import("typing")?
        .getattr("Coroutine")?
        .call_method1("register", (future_into_coroutine_cls,))?;

    // Misc
    m.add_class::<misc::ApiVersion>()?;

    // Testbed stuff
    #[cfg(feature = "test-utils")]
    {
        m.add_function(wrap_pyfunction!(testbed::test_new_testbed, m)?)?;
        m.add_function(wrap_pyfunction!(testbed::test_drop_testbed, m)?)?;
        m.add_function(wrap_pyfunction!(testbed::test_get_testbed_templates, m)?)?;
        m.add_class::<testbed::TestbedDeviceData>()?;
        m.add_class::<testbed::TestbedUserData>()?;
        m.add_class::<testbed::TestbedTemplate>()?;
    }

    Ok(())
}

/// WTF is this ???? O_o
/// `PanicException` inherits `BaseException` which is breaking Pythonic expectation
/// of having error inheriting `Exception`.
/// see https://github.com/PyO3/pyo3/issues/2783
/// TODO: remove me once (if ?) https://github.com/PyO3/pyo3/pull/3057 is merged
fn patch_panic_exception_to_inherit_exception(py: Python) {
    let mut panic_exception_cls = <pyo3::panic::PanicException as pyo3::PyTypeInfo>::type_object(py)
        .as_ptr() as *mut pyo3::ffi::PyTypeObject;
    let exception_cls = <pyo3::exceptions::PyException as pyo3::PyTypeInfo>::type_object(py);
    let new_bases = pyo3::types::PyTuple::new(py, [exception_cls]);
    // SAFETY: `tp_mro` is a pointer to a tuple once the exception structure has been
    // initialized (which is done lazily the first time pyo3 accesses `PanicException`)
    let mro = unsafe { pyo3::types::PyTuple::from_borrowed_ptr(py, (*panic_exception_cls).tp_mro) };
    let new_mro = pyo3::types::PyTuple::new(
        py,
        [
            // 1. Take `PanicException`
            mro.get_item(0).expect("PanicException has 3 items mro"),
            // 2. Add `Exception`
            exception_cls,
            // 3. Take `BaseException` (as `Exception` inherits from it)
            mro.get_item(1).expect("PanicException has 3 items mro"),
            // 4. Take `<class 'object'>`
            mro.get_item(2).expect("PanicException has 3 items mro"),
        ],
    );
    // SAFETY: `tp_base/tp_bases/tp_mro` are pointers that in theory should not be modified
    // once the exception has been initialized. But this is fine as long as `PanicException`
    // has not yet been used (which is the case here since we are initializing the pyo3 module)
    unsafe {
        (*panic_exception_cls).tp_base = exception_cls.into_ptr() as *mut pyo3::ffi::PyTypeObject;
        (*panic_exception_cls).tp_bases = new_bases.into_ptr();
        (*panic_exception_cls).tp_mro = new_mro.into_ptr();
    }
}
