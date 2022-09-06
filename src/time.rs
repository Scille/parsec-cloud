// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
use pyo3::{PyAny, PyResult};

use crate::binding_utils::hash_generic;

#[pyfunction]
/// mock_time takes as argument a DateTime (for FrozenTime), an int (for ShiftedTime) or None (for RealTime)
pub(crate) fn mock_time(time: &PyAny) -> PyResult<()> {
    use libparsec::types::MockedTime;
    libparsec::types::DateTime::mock_time(if let Ok(dt) = time.extract::<DateTime>() {
        MockedTime::FrozenTime(dt.0)
    } else if let Ok(dt) = time.extract::<i64>() {
        MockedTime::ShiftedTime(dt)
    } else if time.is_none() {
        MockedTime::RealTime
    } else {
        return Err(PyTypeError::new_err("Invalid field time"));
    });
    Ok(())
}

#[pyclass(module = "parsec._parsec")]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct TimeProvider(pub libparsec::types::TimeProvider);

#[pymethods]
impl TimeProvider {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(libparsec::types::TimeProvider::default()))
    }

    pub fn now(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.now()))
    }

    pub fn mock_time(&mut self, time: &PyAny) -> PyResult<()> {
        use libparsec::types::MockedTime;
        self.0
            .mock_time(if let Ok(dt) = time.extract::<DateTime>() {
                MockedTime::FrozenTime(dt.0)
            } else if let Ok(dt) = time.extract::<i64>() {
                MockedTime::ShiftedTime(dt)
            } else if time.is_none() {
                MockedTime::RealTime
            } else {
                return Err(PyTypeError::new_err("Invalid field time"));
            });
        Ok(())
    }
}

#[pyclass(module = "parsec._parsec")]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct DateTime(pub libparsec::types::DateTime);

#[pymethods]
impl DateTime {
    #[new]
    #[args(hour = 0, minute = 0, second = 0, microsecond = 0)]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> PyResult<Self> {
        Ok(Self(libparsec::types::DateTime::from_ymd_and_hms_micro(
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
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

    // Pickle support needed by PyQt when passing DateTime in pyQtSignal
    fn __getnewargs__(&self) -> PyResult<(i32, u32, u32, u32, u32, u32, u32)> {
        Ok((
            self.0.year(),
            self.0.month(),
            self.0.day(),
            self.0.hour(),
            self.0.minute(),
            self.0.second(),
            self.0.microsecond(),
        ))
    }

    #[getter]
    fn year(&self) -> PyResult<i32> {
        Ok(self.0.year())
    }

    #[getter]
    fn month(&self) -> PyResult<u32> {
        Ok(self.0.month())
    }

    #[getter]
    fn day(&self) -> PyResult<u32> {
        Ok(self.0.day())
    }

    #[getter]
    fn hour(&self) -> PyResult<u32> {
        Ok(self.0.hour())
    }

    #[getter]
    fn minute(&self) -> PyResult<u32> {
        Ok(self.0.minute())
    }

    #[getter]
    fn second(&self) -> PyResult<u32> {
        Ok(self.0.second())
    }

    #[getter]
    fn microsecond(&self) -> PyResult<u32> {
        Ok(self.0.microsecond())
    }

    #[classmethod]
    fn now(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::types::DateTime::now_legacy()))
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

    fn to_local(&self) -> PyResult<LocalDateTime> {
        Ok(LocalDateTime(self.0.to_local()))
    }

    // Equivalent to ISO 8601
    fn to_rfc3339(&self) -> PyResult<String> {
        Ok(self.0.to_rfc3339())
    }

    fn __sub__(&self, other: Self) -> PyResult<f64> {
        let us = match (self.0 - other.0).num_microseconds() {
            Some(us) => us,
            None => {
                return Err(PyValueError::new_err(format!(
                    "Could not substract {} {}",
                    self.0, other.0
                )))
            }
        };
        Ok(us as f64 / 1e6)
    }

    #[args(
        days = "0.",
        hours = "0.",
        minutes = "0.",
        seconds = "0.",
        microseconds = "0."
    )]
    fn subtract(
        &self,
        days: f64,
        hours: f64,
        minutes: f64,
        seconds: f64,
        microseconds: f64,
    ) -> PyResult<Self> {
        Ok(Self(
            self.0
                - ((((days * 24. + hours) * 60. + minutes) * 60. + seconds) * 1e6 + microseconds)
                    as i64,
        ))
    }

    #[args(
        days = "0.",
        hours = "0.",
        minutes = "0.",
        seconds = "0.",
        microseconds = "0."
    )]
    fn add(
        &self,
        days: f64,
        hours: f64,
        minutes: f64,
        seconds: f64,
        microseconds: f64,
    ) -> PyResult<Self> {
        Ok(Self(
            self.0
                + ((((days * 24. + hours) * 60. + minutes) * 60. + seconds) * 1e6 + microseconds)
                    as i64,
        ))
    }
}

#[pyclass(module = "parsec._parsec")]
#[derive(PartialEq, Eq, Clone)]
pub(crate) struct LocalDateTime(pub libparsec::types::LocalDateTime);

#[pymethods]
impl LocalDateTime {
    #[new]
    #[args(hour = 0, minute = 0, second = 0, microsecond = 0)]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> PyResult<Self> {
        Ok(Self(
            libparsec::types::LocalDateTime::from_ymd_and_hms_micro(
                year,
                month,
                day,
                hour,
                minute,
                second,
                microsecond,
            ),
        ))
    }

    #[getter]
    fn year(&self) -> PyResult<i32> {
        Ok(self.0.year())
    }

    #[getter]
    fn month(&self) -> PyResult<u32> {
        Ok(self.0.month())
    }

    #[getter]
    fn day(&self) -> PyResult<u32> {
        Ok(self.0.day())
    }

    #[getter]
    fn hour(&self) -> PyResult<u32> {
        Ok(self.0.hour())
    }

    #[getter]
    fn minute(&self) -> PyResult<u32> {
        Ok(self.0.minute())
    }

    #[getter]
    fn second(&self) -> PyResult<u32> {
        Ok(self.0.second())
    }

    #[getter]
    fn microsecond(&self) -> PyResult<u32> {
        Ok(self.0.microsecond())
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

    fn to_utc(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.to_utc()))
    }

    fn format(&self, fmt: &str) -> PyResult<String> {
        Ok(self.0.format(fmt))
    }
}
