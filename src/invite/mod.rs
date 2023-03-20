// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod claimer;
mod error;
mod greeter;

use claimer::*;
use error::*;
use greeter::*;

use pyo3::prelude::*;

pub(crate) fn add_mod(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Error
    m.add("InviteError", py.get_type::<InviteError>())?;
    m.add(
        "InvitePeerResetError",
        py.get_type::<InvitePeerResetError>(),
    )?;
    m.add("InviteNotFoundError", py.get_type::<InviteNotFoundError>())?;
    m.add(
        "InviteAlreadyUsedError",
        py.get_type::<InviteAlreadyUsedError>(),
    )?;
    m.add(
        "InviteActiveUsersLimitReachedError",
        py.get_type::<InviteActiveUsersLimitReachedError>(),
    )?;

    // Ctx
    m.add_class::<UserGreetInitialCtx>()?;
    m.add_class::<DeviceGreetInitialCtx>()?;
    m.add_class::<UserGreetInProgress1Ctx>()?;
    m.add_class::<DeviceGreetInProgress1Ctx>()?;
    m.add_class::<UserGreetInProgress2Ctx>()?;
    m.add_class::<DeviceGreetInProgress2Ctx>()?;
    m.add_class::<UserGreetInProgress3Ctx>()?;
    m.add_class::<DeviceGreetInProgress3Ctx>()?;
    m.add_class::<UserGreetInProgress4Ctx>()?;
    m.add_class::<DeviceGreetInProgress4Ctx>()?;
    m.add_class::<UserClaimInitialCtx>()?;
    m.add_class::<DeviceClaimInitialCtx>()?;
    m.add_class::<UserClaimInProgress1Ctx>()?;
    m.add_class::<DeviceClaimInProgress1Ctx>()?;
    m.add_class::<UserClaimInProgress2Ctx>()?;
    m.add_class::<DeviceClaimInProgress2Ctx>()?;
    m.add_class::<UserClaimInProgress3Ctx>()?;
    m.add_class::<DeviceClaimInProgress3Ctx>()?;
    m.add_function(wrap_pyfunction!(claimer_retrieve_info, m)?)?;

    Ok(())
}
