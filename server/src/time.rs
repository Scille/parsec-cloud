// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{PyResult, exceptions::PyValueError, prelude::*, types::PyType};

crate::binding_utils::gen_py_wrapper_class!(
    DateTime,
    libparsec_types::DateTime,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl DateTime {
    #[new]
    #[pyo3(signature = (year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0))]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
    ) -> PyResult<Self> {
        libparsec_types::DateTime::from_ymd_hms_us(
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))
        .map(Self)
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
    fn now(_cls: Bound<'_, PyType>) -> PyResult<Self> {
        Ok(Self(chrono::Utc::now().into()))
    }

    fn as_timestamp_micros(&self) -> PyResult<i64> {
        Ok(self.0.as_timestamp_micros())
    }

    #[classmethod]
    fn from_timestamp_micros(_cls: Bound<'_, PyType>, ts: i64) -> PyResult<Self> {
        libparsec_types::DateTime::from_timestamp_micros(ts)
            .map(Self)
            .map_err(|e| PyValueError::new_err(format!("Invalid datetime `{}`: {}", ts, e)))
    }

    fn as_timestamp_seconds(&self) -> PyResult<i64> {
        Ok(self.0.as_timestamp_seconds())
    }

    #[classmethod]
    fn from_timestamp_seconds(_cls: Bound<'_, PyType>, ts: i64) -> PyResult<Self> {
        libparsec_types::DateTime::from_timestamp_seconds(ts)
            .map(Self)
            .map_err(|e| PyValueError::new_err(format!("Invalid datetime `{}`: {}", ts, e)))
    }

    #[classmethod]
    fn from_rfc3339(_cls: Bound<'_, PyType>, value: &str) -> PyResult<Self> {
        libparsec_types::DateTime::from_rfc3339(value)
            .map(Self)
            .map_err(|e| PyValueError::new_err(format!("Invalid rfc3339 date `{}`: {}", value, e)))
    }

    fn to_rfc3339(&self) -> PyResult<String> {
        Ok(self.0.to_rfc3339())
    }

    fn __sub__(&self, other: Self) -> PyResult<f64> {
        let us = match (self.0 - other.0).num_microseconds() {
            Some(us) => us,
            None => {
                return Err(PyValueError::new_err(format!(
                    "Could not subtract `{}` and `{}`",
                    self.0, other.0
                )));
            }
        };
        Ok(us as f64 / 1e6)
    }

    #[pyo3(signature = (*, days = 0, hours = 0, minutes = 0, seconds = 0, microseconds = 0))]
    fn subtract(
        &self,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
        microseconds: i32,
    ) -> PyResult<Self> {
        let us = -((((days * 24 + hours) * 60 + minutes) * 60 + seconds) as i64 * 1_000_000
            + microseconds as i64);
        Ok(Self(self.0.add_us(us)))
    }

    #[pyo3(signature = (*, days = 0, hours = 0, minutes = 0, seconds = 0, microseconds = 0))]
    fn add(
        &self,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
        microseconds: i32,
    ) -> PyResult<Self> {
        let us = (((days * 24 + hours) * 60 + minutes) * 60 + seconds) as i64 * 1_000_000
            + microseconds as i64;
        Ok(Self(self.0.add_us(us)))
    }
}
