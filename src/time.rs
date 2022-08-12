// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
use pyo3::PyResult;

use crate::binding_utils::hash_generic;

#[pyfunction]
pub(crate) fn mock_time(time: &PyAny) -> PyResult<()> {
    use libparsec::types::MockedTime::*;
    libparsec::types::DateTime::mock_time(if let Ok(dt) = time.extract::<DateTime>() {
        FrozenTime(dt.0)
    } else if let Ok(dt) = time.extract::<i64>() {
        ShiftedTime(dt)
    } else if time.is_none() {
        RealTime
    } else {
        return Err(PyValueError::new_err("Invalid field time"));
    });
    Ok(())
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct DateTime(pub libparsec::types::DateTime);

#[pymethods]
impl DateTime {
    #[new]
    #[args(hour = 0, minute = 0, second = 0)]
    fn new(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> PyResult<Self> {
        Ok(Self(libparsec::types::DateTime::from_ymd_and_hms(
            year, month, day, hour, minute, second,
        )))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Gt => self.0 > other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Ge => self.0 >= other.0,
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        hash_generic(&self.0.to_string(), py)
    }

    #[getter]
    fn year(&self) -> u64 {
        self.0.year()
    }

    #[getter]
    fn month(&self) -> u64 {
        self.0.month()
    }

    #[getter]
    fn day(&self) -> u64 {
        self.0.day()
    }

    #[getter]
    fn hour(&self) -> u64 {
        self.0.hour()
    }

    #[getter]
    fn minute(&self) -> u64 {
        self.0.minute()
    }

    #[getter]
    fn second(&self) -> u64 {
        self.0.second()
    }

    #[classmethod]
    fn now(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::DateTime::now()))
    }

    fn timestamp(&self) -> PyResult<f64> {
        Ok(self.0.get_f64_with_us_precision())
    }

    #[classmethod]
    fn from_timestamp(_cls: &PyType, ts: f64) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::DateTime::from_f64_with_us_precision(ts),
        ))
    }

    fn to_local(&self) -> LocalDateTime {
        LocalDateTime(self.0.to_local())
    }

    fn __sub__(&self, other: Self) -> i64 {
        (self.0 - other.0).num_seconds()
    }

    #[args(days = 0, hours = 0, minutes = 0, seconds = 0, microseconds = 0)]
    fn subtract(
        &self,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
        microseconds: i64,
    ) -> Self {
        Self(
            self.0
                - (((days * 24 + hours) * 60 + minutes) * 60 + seconds) * 1_000_000
                - microseconds,
        )
    }

    #[args(days = 0, hours = 0, minutes = 0, seconds = 0, microseconds = 0)]
    fn add(&self, days: i64, hours: i64, minutes: i64, seconds: i64, microseconds: i64) -> Self {
        Self(
            self.0
                + (((days * 24 + hours) * 60 + minutes) * 60 + seconds) * 1_000_000
                + microseconds,
        )
    }
}

#[pyclass]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct LocalDateTime(pub libparsec::types::LocalDateTime);

#[pymethods]
impl LocalDateTime {
    #[new]
    #[args(hour = 0, minute = 0, second = 0)]
    fn new(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> PyResult<Self> {
        Ok(Self(libparsec::types::LocalDateTime::from_ymd_and_hms(
            year, month, day, hour, minute, second,
        )))
    }

    #[getter]
    fn year(&self) -> u64 {
        self.0.year()
    }

    #[getter]
    fn month(&self) -> u64 {
        self.0.month()
    }

    #[getter]
    fn day(&self) -> u64 {
        self.0.day()
    }

    #[getter]
    fn hour(&self) -> u64 {
        self.0.hour()
    }

    #[getter]
    fn minute(&self) -> u64 {
        self.0.minute()
    }

    #[getter]
    fn second(&self) -> u64 {
        self.0.second()
    }

    fn timestamp(&self) -> PyResult<f64> {
        Ok(self.0.get_f64_with_us_precision())
    }

    #[classmethod]
    fn from_timestamp(_cls: &PyType, ts: f64) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::LocalDateTime::from_f64_with_us_precision(ts),
        ))
    }

    fn format(&self, fmt: &str) -> PyResult<String> {
        Ok(self.0.format(fmt))
    }
}
