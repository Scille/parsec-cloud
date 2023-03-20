// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use pyo3::{exceptions::PyRuntimeError, prelude::*};

use libparsec::protocol::invited_cmds::v2::invite_info::UserOrDevice;

use crate::{
    backend_connection::InvitedCmds,
    data::SASCode,
    ids::{DeviceLabel, HumanHandle, UserID},
    invite::error::InviteExc,
    local_device::LocalDevice,
    runtime::FutureIntoCoroutine,
};

#[pyfunction]
pub(crate) fn claimer_retrieve_info(cmds: &InvitedCmds) -> FutureIntoCoroutine {
    let cmds = cmds.0.as_ref().clone();

    FutureIntoCoroutine::from_raw(async move {
        Ok(
            match libparsec::core::claimer_retrieve_info(&cmds)
                .await
                .map_err(InviteExc::from)?
            {
                UserOrDevice::User {
                    claimer_email,
                    greeter_user_id,
                    greeter_human_handle,
                } => Python::with_gil(|py| {
                    UserClaimInitialCtx(Some(libparsec::core::UserClaimInitialCtx::new(
                        cmds,
                        claimer_email,
                        greeter_user_id,
                        greeter_human_handle,
                    )))
                    .into_py(py)
                }),
                UserOrDevice::Device {
                    greeter_user_id,
                    greeter_human_handle,
                } => Python::with_gil(|py| {
                    DeviceClaimInitialCtx(Some(libparsec::core::DeviceClaimInitialCtx::new(
                        cmds,
                        greeter_user_id,
                        greeter_human_handle,
                    )))
                    .into_py(py)
                }),
            },
        )
    })
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer`
#[pyclass]
pub(crate) struct UserClaimInitialCtx(pub Option<libparsec::core::UserClaimInitialCtx>);

crate::binding_utils::gen_proto!(UserClaimInitialCtx, __repr__);

#[pymethods]
impl UserClaimInitialCtx {
    #[new]
    fn new(
        cmds: &InvitedCmds,
        claimer_email: String,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    ) -> Self {
        Self(Some(libparsec::core::UserClaimInitialCtx::new(
            cmds.0.as_ref().clone(),
            claimer_email,
            greeter_user_id.0,
            greeter_human_handle.map(|x| x.0),
        )))
    }

    fn do_wait_peer(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx
                .ok_or_else(|| PyRuntimeError::new_err("UserClaimInitialCtx has been consumed"))?;

            Ok(ctx
                .do_wait_peer()
                .await
                .map(|x| UserClaimInProgress1Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn claimer_email(&self) -> PyResult<&str> {
        self.0
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("UserClaimInitialCtx has been consumed"))
            .map(|x| x.claimer_email.as_str())
    }

    #[getter]
    fn greeter_user_id(&self) -> PyResult<UserID> {
        self.0
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("UserClaimInitialCtx has been consumed"))
            .map(|x| UserID(x.greeter_user_id().clone()))
    }

    #[getter]
    fn greeter_human_handle(&self) -> PyResult<Option<HumanHandle>> {
        self.0
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("UserClaimInitialCtx has been consumed"))
            .map(|x| x.greeter_human_handle().clone().map(HumanHandle))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer`
#[pyclass]
pub(crate) struct DeviceClaimInitialCtx(pub Option<libparsec::core::DeviceClaimInitialCtx>);

crate::binding_utils::gen_proto!(DeviceClaimInitialCtx, __repr__);

#[pymethods]
impl DeviceClaimInitialCtx {
    #[new]
    fn new(
        cmds: &InvitedCmds,
        greeter_user_id: UserID,
        greeter_human_handle: Option<HumanHandle>,
    ) -> Self {
        Self(Some(libparsec::core::DeviceClaimInitialCtx::new(
            cmds.0.as_ref().clone(),
            greeter_user_id.0,
            greeter_human_handle.map(|x| x.0),
        )))
    }

    fn do_wait_peer(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceClaimInitialCtx has been consumed")
            })?;

            Ok(ctx
                .do_wait_peer()
                .await
                .map(|x| DeviceClaimInProgress1Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn greeter_user_id(&self) -> PyResult<UserID> {
        self.0
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("DeviceClaimInitialCtx has been consumed"))
            .map(|x| UserID(x.greeter_user_id().clone()))
    }

    #[getter]
    fn greeter_human_handle(&self) -> PyResult<Option<HumanHandle>> {
        self.0
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("DeviceClaimInitialCtx has been consumed"))
            .map(|x| x.greeter_human_handle().clone().map(HumanHandle))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_signify_trust`
#[pyclass]
pub(crate) struct UserClaimInProgress1Ctx(pub Option<libparsec::core::UserClaimInProgress1Ctx>);

crate::binding_utils::gen_proto!(UserClaimInProgress1Ctx, __repr__);

#[pymethods]
impl UserClaimInProgress1Ctx {
    fn do_signify_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserClaimInProgress1Ctx has been consumed")
            })?;

            Ok(ctx
                .do_signify_trust()
                .await
                .map(|x| UserClaimInProgress2Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn greeter_sas(&self) -> PyResult<SASCode> {
        self.0
            .as_ref()
            .map(|x| x.greeter_sas())
            .cloned()
            .map(SASCode)
            .ok_or_else(|| PyRuntimeError::new_err("UserClaimInProgress1Ctx has been consumed"))
    }

    fn generate_greeter_sas_choices(&self, size: usize) -> PyResult<Vec<SASCode>> {
        self.0
            .as_ref()
            .map(|x| {
                x.generate_greeter_sas_choices(size)
                    .into_iter()
                    .map(SASCode)
                    .collect()
            })
            .ok_or_else(|| PyRuntimeError::new_err("UserClaimInProgress1Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_signify_trust`
#[pyclass]
pub(crate) struct DeviceClaimInProgress1Ctx(pub Option<libparsec::core::DeviceClaimInProgress1Ctx>);

crate::binding_utils::gen_proto!(DeviceClaimInProgress1Ctx, __repr__);

#[pymethods]
impl DeviceClaimInProgress1Ctx {
    fn do_signify_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceClaimInProgress1Ctx has been consumed")
            })?;

            Ok(ctx
                .do_signify_trust()
                .await
                .map(|x| DeviceClaimInProgress2Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn greeter_sas(&self) -> PyResult<SASCode> {
        self.0
            .as_ref()
            .map(|x| x.greeter_sas())
            .cloned()
            .map(SASCode)
            .ok_or_else(|| PyRuntimeError::new_err("DeviceClaimInProgress1Ctx has been consumed"))
    }

    fn generate_greeter_sas_choices(&self, size: usize) -> PyResult<Vec<SASCode>> {
        self.0
            .as_ref()
            .map(|x| {
                x.generate_greeter_sas_choices(size)
                    .into_iter()
                    .map(SASCode)
                    .collect()
            })
            .ok_or_else(|| PyRuntimeError::new_err("DeviceClaimInProgress1Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer_trust`
#[pyclass]
pub(crate) struct UserClaimInProgress2Ctx(pub Option<libparsec::core::UserClaimInProgress2Ctx>);

crate::binding_utils::gen_proto!(UserClaimInProgress2Ctx, __repr__);

#[pymethods]
impl UserClaimInProgress2Ctx {
    fn do_wait_peer_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserClaimInProgress2Ctx has been consumed")
            })?;

            Ok(ctx
                .do_wait_peer_trust()
                .await
                .map(|x| UserClaimInProgress3Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn claimer_sas(&self) -> PyResult<SASCode> {
        self.0
            .as_ref()
            .map(|x| x.claimer_sas())
            .cloned()
            .map(SASCode)
            .ok_or_else(|| PyRuntimeError::new_err("UserClaimInProgress2Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer_trust`
#[pyclass]
pub(crate) struct DeviceClaimInProgress2Ctx(pub Option<libparsec::core::DeviceClaimInProgress2Ctx>);

crate::binding_utils::gen_proto!(DeviceClaimInProgress2Ctx, __repr__);

#[pymethods]
impl DeviceClaimInProgress2Ctx {
    fn do_wait_peer_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceClaimInProgress2Ctx has been consumed")
            })?;

            Ok(ctx
                .do_wait_peer_trust()
                .await
                .map(|x| DeviceClaimInProgress3Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn claimer_sas(&self) -> PyResult<SASCode> {
        self.0
            .as_ref()
            .map(|x| x.claimer_sas())
            .cloned()
            .map(SASCode)
            .ok_or_else(|| PyRuntimeError::new_err("DeviceClaimInProgress2Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_claim_user`
#[pyclass]
pub(crate) struct UserClaimInProgress3Ctx(pub Option<libparsec::core::UserClaimInProgress3Ctx>);

crate::binding_utils::gen_proto!(UserClaimInProgress3Ctx, __repr__);

#[pymethods]
impl UserClaimInProgress3Ctx {
    fn do_claim_user(
        &mut self,
        requested_device_label: Option<DeviceLabel>,
        requested_human_handle: Option<HumanHandle>,
    ) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserClaimInProgress3Ctx has been consumed")
            })?;

            Ok(ctx
                .do_claim_user(
                    requested_device_label.map(|x| x.0),
                    requested_human_handle.map(|x| x.0),
                )
                .await
                .map(|local_device| LocalDevice(Arc::new(local_device)))
                .map_err(InviteExc::from)?)
        })
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_claim_device`
#[pyclass]
pub(crate) struct DeviceClaimInProgress3Ctx(pub Option<libparsec::core::DeviceClaimInProgress3Ctx>);

crate::binding_utils::gen_proto!(DeviceClaimInProgress3Ctx, __repr__);

#[pymethods]
impl DeviceClaimInProgress3Ctx {
    fn do_claim_device(
        &mut self,
        requested_device_label: Option<DeviceLabel>,
    ) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceClaimInProgress3Ctx has been consumed")
            })?;

            Ok(ctx
                .do_claim_device(requested_device_label.map(|x| x.0))
                .await
                .map(|local_device| LocalDevice(Arc::new(local_device)))
                .map_err(InviteExc::from)?)
        })
    }
}
