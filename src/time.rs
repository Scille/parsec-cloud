// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyType;
use pyo3::{PyAny, PyResult};

use crate::binding_utils::hash_generic;
use crate::runtime::spawn_future_into_trio_coroutine;

#[pyfunction]
/// mock_time takes as argument a DateTime (for FrozenTime), an int (for ShiftedTime) or None (for RealTime)
pub(crate) fn mock_time(time: &PyAny) -> PyResult<()> {
    use libparsec::types::MockedTime;
    libparsec::types::DateTime::mock_time(if let Ok(dt) = time.extract::<DateTime>() {
        MockedTime::FrozenTime(dt.0)
    } else if let Ok(us) = time.extract::<i64>() {
        MockedTime::ShiftedTime { microseconds: us }
    } else if time.is_none() {
        MockedTime::RealTime
    } else {
        return Err(PyTypeError::new_err("Invalid field time"));
    });
    Ok(())
}

#[pyclass]
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

    pub fn sleeping_stats(&self) -> PyResult<u64> {
        Ok(self.0.sleeping_stats())
    }

    // Booyakasha !
    pub fn sleep<'py>(&self, py: Python<'py>, time: f64) -> PyResult<&'py PyAny> {
        let time_provider = self.0.clone();
        spawn_future_into_trio_coroutine(py, async move {
            time_provider
                .sleep(libparsec::types::Duration::microseconds(
                    (time * 1e6) as i64,
                ))
                .await;
            Ok(())
        })
    }

    pub fn new_child(&self) -> PyResult<TimeProvider> {
        Ok(TimeProvider(self.0.new_child()))
    }

    #[args(freeze = "None", shift = "None", realtime = "false")]
    pub fn mock_time(
        &mut self,
        freeze: Option<DateTime>,
        shift: Option<f64>,
        realtime: bool,
    ) -> PyResult<()> {
        use libparsec::types::MockedTime;
        let mock_config = match (freeze, shift, realtime) {
            (None, None, true) => MockedTime::RealTime,
            (Some(dt), None, false) => MockedTime::FrozenTime(dt.0),
            (None, Some(shift_in_seconds), false) => MockedTime::ShiftedTime {
                microseconds: (shift_in_seconds * 1e6) as i64,
            },
            _ => {
                return Err(PyValueError::new_err(
                    "Must only provide one of `freeze`, `shift` and `realtime`",
                ))
            }
        };
        self.0.mock_time(mock_config);
        Ok(())
    }
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
        Ok(format!("DateTime({})", self.0))
    }

    fn __str__(&self) -> PyResult<String> {
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
    fn year(&self) -> PyResult<u64> {
        Ok(self.0.year())
    }

    #[getter]
    fn month(&self) -> PyResult<u64> {
        Ok(self.0.month())
    }

    #[getter]
    fn day(&self) -> PyResult<u64> {
        Ok(self.0.day())
    }

    #[getter]
    fn hour(&self) -> PyResult<u64> {
        Ok(self.0.hour())
    }

    #[getter]
    fn minute(&self) -> PyResult<u64> {
        Ok(self.0.minute())
    }

    #[getter]
    fn second(&self) -> PyResult<u64> {
        Ok(self.0.second())
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
        let us =
            -((((days * 24. + hours) * 60. + minutes) * 60. + seconds) * 1e6 + microseconds) as i64;
        Ok(Self(self.0.add_us(us)))
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
        let us =
            ((((days * 24. + hours) * 60. + minutes) * 60. + seconds) * 1e6 + microseconds) as i64;
        Ok(Self(self.0.add_us(us)))
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

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    #[getter]
    fn year(&self) -> PyResult<u64> {
        Ok(self.0.year())
    }

    #[getter]
    fn month(&self) -> PyResult<u64> {
        Ok(self.0.month())
    }

    #[getter]
    fn day(&self) -> PyResult<u64> {
        Ok(self.0.day())
    }

    #[getter]
    fn hour(&self) -> PyResult<u64> {
        Ok(self.0.hour())
    }

    #[getter]
    fn minute(&self) -> PyResult<u64> {
        Ok(self.0.minute())
    }

    #[getter]
    fn second(&self) -> PyResult<u64> {
        Ok(self.0.second())
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
