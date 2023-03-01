// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::{
    backend_connection::AuthenticatedCmds,
    data::SASCode,
    enumerate::UserProfile,
    ids::{DeviceLabel, HumanHandle, InvitationToken},
    invite::error::InviteExc,
    local_device::LocalDevice,
    runtime::FutureIntoCoroutine,
};

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer`
#[pyclass]
pub(crate) struct UserGreetInitialCtx(pub Option<libparsec::core::UserGreetInitialCtx>);

crate::binding_utils::gen_proto!(UserGreetInitialCtx, __repr__);

#[pymethods]
impl UserGreetInitialCtx {
    #[new]
    fn new(cmds: &AuthenticatedCmds, token: InvitationToken) -> Self {
        Self(Some(libparsec::core::UserGreetInitialCtx::new(
            cmds.0.as_ref().clone(),
            token.0,
        )))
    }

    fn do_wait_peer(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx
                .ok_or_else(|| PyRuntimeError::new_err("UserGreetInitialCtx has been consumed"))?;

            Ok(ctx
                .do_wait_peer()
                .await
                .map(|x| UserGreetInProgress1Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer`
#[pyclass]
pub(crate) struct DeviceGreetInitialCtx(pub Option<libparsec::core::DeviceGreetInitialCtx>);

crate::binding_utils::gen_proto!(DeviceGreetInitialCtx, __repr__);

#[pymethods]
impl DeviceGreetInitialCtx {
    #[new]
    fn new(cmds: &AuthenticatedCmds, token: InvitationToken) -> Self {
        Self(Some(libparsec::core::DeviceGreetInitialCtx::new(
            cmds.0.as_ref().clone(),
            token.0,
        )))
    }

    fn do_wait_peer(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceGreetInitialCtx has been consumed")
            })?;

            Ok(ctx
                .do_wait_peer()
                .await
                .map(|x| DeviceGreetInProgress1Ctx(Some(x)))
                .map_err(InviteExc::from)?)
        })
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer_trust`
#[pyclass]
pub(crate) struct UserGreetInProgress1Ctx(pub Option<libparsec::core::UserGreetInProgress1Ctx>);

crate::binding_utils::gen_proto!(UserGreetInProgress1Ctx, __repr__);

#[pymethods]
impl UserGreetInProgress1Ctx {
    fn do_wait_peer_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserGreetInProgress1Ctx has been consumed")
            })?;

            Ok(ctx
                .do_wait_peer_trust()
                .await
                .map(|x| UserGreetInProgress2Ctx(Some(x)))
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
            .ok_or_else(|| PyRuntimeError::new_err("UserGreetInProgress1Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_wait_peer_trust`
#[pyclass]
pub(crate) struct DeviceGreetInProgress1Ctx(pub Option<libparsec::core::DeviceGreetInProgress1Ctx>);

crate::binding_utils::gen_proto!(DeviceGreetInProgress1Ctx, __repr__);

#[pymethods]
impl DeviceGreetInProgress1Ctx {
    fn do_wait_peer_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceGreetInProgress1Ctx has been consumed")
            })?;

            Ok(ctx
                .do_wait_peer_trust()
                .await
                .map(|x| DeviceGreetInProgress2Ctx(Some(x)))
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
            .ok_or_else(|| PyRuntimeError::new_err("DeviceGreetInProgress1Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_signify_trust`
#[pyclass]
pub(crate) struct UserGreetInProgress2Ctx(pub Option<libparsec::core::UserGreetInProgress2Ctx>);

crate::binding_utils::gen_proto!(UserGreetInProgress2Ctx, __repr__);

#[pymethods]
impl UserGreetInProgress2Ctx {
    fn do_signify_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserGreetInProgress2Ctx has been consumed")
            })?;

            Ok(ctx
                .do_signify_trust()
                .await
                .map(|x| UserGreetInProgress3Ctx(Some(x)))
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
            .ok_or_else(|| PyRuntimeError::new_err("UserGreetInProgress2Ctx has been consumed"))
    }

    fn generate_claimer_sas_choices(&self, size: usize) -> PyResult<Vec<SASCode>> {
        self.0
            .as_ref()
            .map(|x| {
                x.generate_claimer_sas_choices(size)
                    .into_iter()
                    .map(SASCode)
                    .collect()
            })
            .ok_or_else(|| PyRuntimeError::new_err("UserGreetInProgress2Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_signify_trust`
#[pyclass]
pub(crate) struct DeviceGreetInProgress2Ctx(pub Option<libparsec::core::DeviceGreetInProgress2Ctx>);

crate::binding_utils::gen_proto!(DeviceGreetInProgress2Ctx, __repr__);

#[pymethods]
impl DeviceGreetInProgress2Ctx {
    fn do_signify_trust(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceGreetInProgress2Ctx has been consumed")
            })?;

            Ok(ctx
                .do_signify_trust()
                .await
                .map(|x| DeviceGreetInProgress3Ctx(Some(x)))
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
            .ok_or_else(|| PyRuntimeError::new_err("DeviceGreetInProgress2Ctx has been consumed"))
    }

    fn generate_claimer_sas_choices(&self, size: usize) -> PyResult<Vec<SASCode>> {
        self.0
            .as_ref()
            .map(|x| {
                x.generate_claimer_sas_choices(size)
                    .into_iter()
                    .map(SASCode)
                    .collect()
            })
            .ok_or_else(|| PyRuntimeError::new_err("DeviceGreetInProgress2Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_get_claim_requests`
#[pyclass]
pub(crate) struct UserGreetInProgress3Ctx(pub Option<libparsec::core::UserGreetInProgress3Ctx>);

crate::binding_utils::gen_proto!(UserGreetInProgress3Ctx, __repr__);

#[pymethods]
impl UserGreetInProgress3Ctx {
    fn do_get_claim_requests(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserGreetInProgress3Ctx has been consumed")
            })?;

            Ok(ctx
                .do_get_claim_requests()
                .await
                .map(Option::Some)
                .map(UserGreetInProgress4Ctx)
                .map_err(InviteExc::from)?)
        })
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_get_claim_requests`
#[pyclass]
pub(crate) struct DeviceGreetInProgress3Ctx(pub Option<libparsec::core::DeviceGreetInProgress3Ctx>);

crate::binding_utils::gen_proto!(DeviceGreetInProgress3Ctx, __repr__);

#[pymethods]
impl DeviceGreetInProgress3Ctx {
    fn do_get_claim_requests(&mut self) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceGreetInProgress3Ctx has been consumed")
            })?;

            Ok(ctx
                .do_get_claim_requests()
                .await
                .map(Option::Some)
                .map(DeviceGreetInProgress4Ctx)
                .map_err(InviteExc::from)?)
        })
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_create_new_user`
#[pyclass]
pub(crate) struct UserGreetInProgress4Ctx(pub Option<libparsec::core::UserGreetInProgress4Ctx>);

crate::binding_utils::gen_proto!(UserGreetInProgress4Ctx, __repr__);

#[pymethods]
impl UserGreetInProgress4Ctx {
    fn do_create_new_user(
        &mut self,
        author: LocalDevice,
        human_handle: Option<HumanHandle>,
        device_label: Option<DeviceLabel>,
        profile: UserProfile,
    ) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async move {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("UserGreetInProgress4Ctx has been consumed")
            })?;

            Ok(ctx
                .do_create_new_user(
                    &author.0,
                    device_label.map(|x| x.0),
                    human_handle.map(|x| x.0),
                    profile.0,
                )
                .await
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn requested_human_handle(&self) -> PyResult<Option<HumanHandle>> {
        self.0
            .as_ref()
            .map(|x| x.requested_human_handle.clone().map(HumanHandle))
            .ok_or_else(|| PyRuntimeError::new_err("UserGreetInProgress4Ctx has been consumed"))
    }

    #[getter]
    fn requested_device_label(&self) -> PyResult<Option<DeviceLabel>> {
        self.0
            .as_ref()
            .map(|x| x.requested_device_label.clone().map(DeviceLabel))
            .ok_or_else(|| PyRuntimeError::new_err("UserGreetInProgress4Ctx has been consumed"))
    }
}

/// Because we can't consume `self` with pyo3, we use an Option instead to consume the Rust object.
/// It will be consumed if we go to the next step: `do_create_new_device`
#[pyclass]
pub(crate) struct DeviceGreetInProgress4Ctx(pub Option<libparsec::core::DeviceGreetInProgress4Ctx>);

crate::binding_utils::gen_proto!(DeviceGreetInProgress4Ctx, __repr__);

#[pymethods]
impl DeviceGreetInProgress4Ctx {
    fn do_create_new_device(
        &mut self,
        author: LocalDevice,
        device_label: Option<DeviceLabel>,
    ) -> FutureIntoCoroutine {
        let ctx = self.0.take();

        FutureIntoCoroutine::from(async move {
            let ctx = ctx.ok_or_else(|| {
                PyRuntimeError::new_err("DeviceGreetInProgress4Ctx has been consumed")
            })?;

            Ok(ctx
                .do_create_new_device(&author.0, device_label.map(|x| x.0))
                .await
                .map_err(InviteExc::from)?)
        })
    }

    #[getter]
    fn requested_device_label(&self) -> PyResult<Option<DeviceLabel>> {
        self.0
            .as_ref()
            .map(|x| x.requested_device_label.clone().map(DeviceLabel))
            .ok_or_else(|| PyRuntimeError::new_err("DeviceGreetInProgress4Ctx has been consumed"))
    }
}
