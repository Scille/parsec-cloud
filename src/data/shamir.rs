// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyType},
    PyResult, Python,
};
use std::{collections::HashMap, num::NonZeroU64};

use crate::{
    binding_utils::BytesWrapper,
    ids::{DeviceID, UserID},
    time::DateTime,
};

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoveryBriefCertificate(
    pub libparsec::types::ShamirRecoveryBriefCertificate,
);

crate::binding_utils::gen_proto!(ShamirRecoveryBriefCertificate, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoveryBriefCertificate, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoveryBriefCertificate, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoveryBriefCertificate, __richcmp__, eq);

#[pymethods]
impl ShamirRecoveryBriefCertificate {
    #[new]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        threshold: u64,
        per_recipient_shares: HashMap<UserID, u64>,
    ) -> PyResult<Self> {
        let author = author.0;
        let timestamp = timestamp.0;
        let threshold = NonZeroU64::try_from(threshold)
            .map_err(|_| PyValueError::new_err("threshold must be greater than 0"))?;
        let per_recipient_shares = per_recipient_shares
            .into_iter()
            .map(|(k, v)| {
                NonZeroU64::try_from(v).map(|v| (k.0, v)).map_err(|_| {
                    PyValueError::new_err("per_recipient_shares values must be greater than 0")
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(libparsec::types::ShamirRecoveryBriefCertificate {
            author,
            timestamp,
            threshold,
            per_recipient_shares,
        }))
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author.clone())
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn threshold(&self) -> u64 {
        self.0.threshold.into()
    }

    #[getter]
    fn per_recipient_shares(&self) -> HashMap<UserID, u64> {
        self.0
            .per_recipient_shares
            .clone()
            .into_iter()
            .map(|(k, v)| (UserID(k), v.into()))
            .collect()
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    #[classmethod]
    fn load(_cls: &PyType, raw: &[u8]) -> PyResult<Self> {
        libparsec::types::ShamirRecoveryBriefCertificate::load(raw)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoveryShareData(pub libparsec::types::ShamirRecoveryShareData);

crate::binding_utils::gen_proto!(ShamirRecoveryShareData, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoveryShareData, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoveryShareData, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoveryShareData, __richcmp__, eq);

#[pymethods]
impl ShamirRecoveryShareData {
    #[new]
    fn new(reveal_token_share: BytesWrapper, data_key_share: BytesWrapper) -> Self {
        crate::binding_utils::unwrap_bytes!(reveal_token_share, data_key_share);

        Self(libparsec::types::ShamirRecoveryShareData {
            reveal_token_share,
            data_key_share,
        })
    }

    #[getter]
    fn reveal_token_share(&self) -> &[u8] {
        &self.0.reveal_token_share
    }

    #[getter]
    fn data_key_share(&self) -> &[u8] {
        &self.0.data_key_share
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoveryShareCertificate(
    pub libparsec::types::ShamirRecoveryShareCertificate,
);

crate::binding_utils::gen_proto!(ShamirRecoveryShareCertificate, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoveryShareCertificate, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoveryShareCertificate, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoveryShareCertificate, __richcmp__, eq);

#[pymethods]
impl ShamirRecoveryShareCertificate {
    #[new]
    fn new(
        author: DeviceID,
        timestamp: DateTime,
        recipient: UserID,
        ciphered_share: ShamirRecoveryShareData,
    ) -> Self {
        let author = author.0;
        let timestamp = timestamp.0;
        let recipient = recipient.0;
        let ciphered_share = ciphered_share.0;

        Self(libparsec::types::ShamirRecoveryShareCertificate {
            author,
            timestamp,
            recipient,
            ciphered_share,
        })
    }

    #[getter]
    fn author(&self) -> DeviceID {
        DeviceID(self.0.author.clone())
    }

    #[getter]
    fn timestamp(&self) -> DateTime {
        DateTime(self.0.timestamp)
    }

    #[getter]
    fn recipient(&self) -> UserID {
        UserID(self.0.recipient.clone())
    }

    #[getter]
    fn ciphered_share(&self) -> ShamirRecoveryShareData {
        ShamirRecoveryShareData(self.0.ciphered_share.clone())
    }

    fn dump<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump())
    }

    #[classmethod]
    fn load(_cls: &PyType, raw: &[u8]) -> PyResult<Self> {
        libparsec::types::ShamirRecoveryShareCertificate::load(raw)
            .map(Self)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
