// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    prelude::{pymodule, wrap_pyfunction, PyModule, PyResult, Python},
    AsPyPointer, FromPyPointer, IntoPyPointer,
};

mod addrs;
mod binding_utils;
mod crypto;
mod data;
mod enumerate;
mod ids;
mod misc;
mod protocol;
#[cfg(feature = "test-utils")]
mod testbed;
mod time;

pub(crate) use addrs::*;
pub(crate) use binding_utils::*;
pub(crate) use crypto::*;
pub(crate) use enumerate::*;
pub(crate) use ids::*;
pub(crate) use misc::*;
#[cfg(feature = "test-utils")]
pub(crate) use testbed::*;
pub(crate) use time::*;

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_parsec")]
fn entrypoint(py: Python, m: &PyModule) -> PyResult<()> {
    crate::crypto::add_mod(py, m)?;
    crate::data::add_mod(m)?;
    crate::protocol::add_mod(py, m)?;

    patch_panic_exception_to_inherit_exception(py);
    // Useful to expose `PanicException` for testing
    m.add(
        "PanicException",
        <pyo3::panic::PanicException as pyo3::PyTypeInfo>::type_object(py),
    )?;

    m.add_class::<ParsecAddr>()?;
    m.add_class::<ParsecOrganizationAddr>()?;
    m.add_class::<ParsecActionAddr>()?;
    m.add_class::<ParsecOrganizationBootstrapAddr>()?;
    m.add_class::<ParsecOrganizationFileLinkAddr>()?;
    m.add_class::<ParsecInvitationAddr>()?;
    m.add_class::<ParsecPkiEnrollmentAddr>()?;
    m.add_function(wrap_pyfunction!(export_root_verify_key, m)?)?;

    m.add_class::<DeviceFileType>()?;
    m.add_class::<InvitationStatus>()?;
    m.add_class::<InvitationType>()?;
    m.add_class::<RealmRole>()?;
    m.add_class::<UserProfile>()?;

    m.add_class::<OrganizationID>()?;
    m.add_class::<VlobID>()?;
    m.add_class::<BlockID>()?;
    m.add_class::<VlobID>()?;
    m.add_class::<VlobID>()?;
    m.add_class::<ChunkID>()?;
    m.add_class::<SequesterServiceID>()?;
    m.add_class::<EnrollmentID>()?;
    m.add_class::<HumanHandle>()?;
    m.add_class::<DeviceID>()?;
    m.add_class::<DeviceName>()?;
    m.add_class::<DeviceLabel>()?;
    m.add_class::<UserID>()?;
    m.add_class::<BootstrapToken>()?;
    m.add_class::<InvitationToken>()?;

    // Time
    m.add_class::<DateTime>()?;

    // Misc
    m.add_class::<ApiVersion>()?;

    // Testbed stuff
    #[cfg(feature = "test-utils")]
    {
        let tm = PyModule::new(py, "testbed")?;
        m.add_submodule(tm)?;
        // tm.add_function(wrap_pyfunction!(test_new_testbed, tm)?)?;
        // tm.add_function(wrap_pyfunction!(test_drop_testbed, tm)?)?;
        tm.add_function(wrap_pyfunction!(test_get_testbed_template, tm)?)?;
        tm.add_class::<TestbedTemplateContent>()?;
        tm.add_class::<TestbedEventBootstrapOrganization>()?;
        tm.add_class::<TestbedEventNewSequesterService>()?;
        tm.add_class::<TestbedEventRevokeSequesterService>()?;
        tm.add_class::<TestbedEventNewUser>()?;
        tm.add_class::<TestbedEventNewDevice>()?;
        tm.add_class::<TestbedEventUpdateUserProfile>()?;
        tm.add_class::<TestbedEventRevokeUser>()?;
        tm.add_class::<TestbedEventNewUserInvitation>()?;
        tm.add_class::<TestbedEventNewDeviceInvitation>()?;
        tm.add_class::<TestbedEventNewRealm>()?;
        tm.add_class::<TestbedEventShareRealm>()?;
        tm.add_class::<TestbedEventRenameRealm>()?;
        tm.add_class::<TestbedEventRotateKeyRealm>()?;
        tm.add_class::<TestbedEventArchiveRealm>()?;
        tm.add_class::<TestbedEventNewShamirRecovery>()?;
        tm.add_class::<TestbedEventCreateOrUpdateOpaqueVlob>()?;
        tm.add_class::<TestbedEventCreateBlock>()?;
        tm.add_class::<TestbedEventCreateOpaqueBlock>()?;
    }

    Ok(())
}

/// WTF is this ???? O_o
/// `PanicException` inherits `BaseException` which is breaking Pythonic expectation
/// of having error inheriting `Exception`.
/// see https://github.com/PyO3/pyo3/issues/2783
/// TODO: remove me once (if ?) https://github.com/PyO3/pyo3/pull/3057 is merged
fn patch_panic_exception_to_inherit_exception(py: Python) {
    let panic_exception_cls = <pyo3::panic::PanicException as pyo3::PyTypeInfo>::type_object(py)
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
