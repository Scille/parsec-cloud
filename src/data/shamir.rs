// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::{
    exceptions::PyValueError,
    pyclass, pymethods,
    types::{PyBytes, PyList, PyType},
    PyResult, Python,
};
use std::{collections::HashMap, num::NonZeroU64};

use crate::api_crypto::{PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey};
use crate::{
    binding_utils::BytesWrapper,
    data::DataResult,
    ids::{DeviceID, ShamirRevealToken, UserID},
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

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
    ) -> DataResult<Self> {
        Ok(
            libparsec::types::ShamirRecoveryBriefCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                &expected_author.0,
            )
            .map(Self)?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(libparsec::types::ShamirRecoveryBriefCertificate::unsecure_load(signed).map(Self)?)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoveryCommunicatedData(
    pub libparsec::types::ShamirRecoveryCommunicatedData,
);

crate::binding_utils::gen_proto!(ShamirRecoveryCommunicatedData, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoveryCommunicatedData, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoveryCommunicatedData, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoveryCommunicatedData, __richcmp__, eq);

#[pymethods]
impl ShamirRecoveryCommunicatedData {
    #[new]
    fn new(weighted_share: Vec<BytesWrapper>) -> Self {
        crate::binding_utils::unwrap_bytes!(weighted_share);

        Self(libparsec::types::ShamirRecoveryCommunicatedData { weighted_share })
    }

    #[getter]
    fn weighted_share<'p>(&self, py: Python<'p>) -> &'p PyList {
        PyList::new(
            py,
            self.0.weighted_share.iter().map(|x| PyBytes::new(py, x)),
        )
    }

    fn dump<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(PyValueError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, data: &[u8]) -> PyResult<Self> {
        let share_data = libparsec::types::ShamirRecoveryCommunicatedData::load(data)
            .map_err(PyValueError::new_err)?;
        Ok(Self(share_data))
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
    fn new(weighted_share: Vec<BytesWrapper>) -> Self {
        crate::binding_utils::unwrap_bytes!(weighted_share);

        Self(libparsec::types::ShamirRecoveryShareData { weighted_share })
    }

    #[getter]
    fn weighted_share<'p>(&self, py: Python<'p>) -> &'p PyList {
        PyList::new(
            py,
            self.0.weighted_share.iter().map(|x| PyBytes::new(py, x)),
        )
    }

    #[classmethod]
    fn decrypt_verify_and_load_for(
        _cls: &PyType,
        ciphered: &[u8],
        recipient_privkey: &PrivateKey,
        author_verify_key: &VerifyKey,
    ) -> PyResult<ShamirRecoveryShareData> {
        let share_data = libparsec::types::ShamirRecoveryShareData::decrypt_verify_and_load_for(
            ciphered,
            &recipient_privkey.0,
            &author_verify_key.0,
        )
        .map_err(PyValueError::new_err)?;
        Ok(ShamirRecoveryShareData(share_data))
    }

    fn dump_sign_and_encrypt_for<'py>(
        &self,
        author_signkey: &SigningKey,
        recipient_pubkey: &PublicKey,
        py: Python<'py>,
    ) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(
            py,
            &self
                .0
                .dump_sign_and_encrypt_for(&author_signkey.0, &recipient_pubkey.0),
        ))
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
        ciphered_share: &[u8],
    ) -> Self {
        let author = author.0;
        let timestamp = timestamp.0;
        let recipient = recipient.0;
        let ciphered_share = ciphered_share.into();

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
    fn ciphered_share(&self) -> &[u8] {
        &self.0.ciphered_share
    }

    #[classmethod]
    fn verify_and_load(
        _cls: &PyType,
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
    ) -> DataResult<Self> {
        Ok(
            libparsec::types::ShamirRecoveryShareCertificate::verify_and_load(
                signed,
                &author_verify_key.0,
                &expected_author.0,
            )
            .map(Self)?,
        )
    }

    fn dump_and_sign<'py>(&self, author_signkey: &SigningKey, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.0.dump_and_sign(&author_signkey.0))
    }

    #[classmethod]
    fn unsecure_load(_cls: &PyType, signed: &[u8]) -> DataResult<Self> {
        Ok(libparsec::types::ShamirRecoveryShareCertificate::unsecure_load(signed).map(Self)?)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct ShamirRecoverySecret(pub libparsec::types::ShamirRecoverySecret);

crate::binding_utils::gen_proto!(ShamirRecoverySecret, __repr__);
crate::binding_utils::gen_proto!(ShamirRecoverySecret, __copy__);
crate::binding_utils::gen_proto!(ShamirRecoverySecret, __deepcopy__);
crate::binding_utils::gen_proto!(ShamirRecoverySecret, __richcmp__, eq);

#[pymethods]
impl ShamirRecoverySecret {
    #[new]
    fn new(data_key: SecretKey, reveal_token: ShamirRevealToken) -> Self {
        Self(libparsec::types::ShamirRecoverySecret {
            data_key: data_key.0,
            reveal_token: reveal_token.0,
        })
    }

    #[getter]
    fn data_key(&self) -> SecretKey {
        SecretKey(self.0.data_key.clone())
    }

    #[getter]
    fn reveal_token(&self) -> ShamirRevealToken {
        ShamirRevealToken(self.0.reveal_token)
    }

    fn dump<'p>(&self, py: Python<'p>) -> PyResult<&'p PyBytes> {
        Ok(PyBytes::new(
            py,
            &self.0.dump().map_err(PyValueError::new_err)?,
        ))
    }

    #[classmethod]
    fn load(_cls: &PyType, data: &[u8]) -> PyResult<Self> {
        let share_data =
            libparsec::types::ShamirRecoverySecret::load(data).map_err(PyValueError::new_err)?;
        Ok(Self(share_data))
    }
}
