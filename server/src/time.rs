// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use pyo3::{
    exceptions::{PyAttributeError, PyValueError},
    prelude::*,
    types::PyType,
    PyAny, PyResult,
};

use crate::FutureIntoCoroutine;

#[pyfunction]
/// mock_time takes as argument a DateTime (for FrozenTime), an int (for ShiftedTime) or None (for RealTime)
pub(crate) fn mock_time(time: &PyAny) -> PyResult<()> {
    #[cfg(not(feature = "test-utils"))]
    {
        let _time = time;
        Err(pyo3::exceptions::PyRuntimeError::new_err(
            "Test features are disabled !",
        ))
    }

    #[cfg(feature = "test-utils")]
    {
        use libparsec::low_level::types::MockedTime;
        libparsec::low_level::types::DateTime::mock_time(
            if let Ok(dt) = time.extract::<DateTime>() {
                MockedTime::FrozenTime(dt.0)
            } else if let Ok(us) = time.extract::<i64>() {
                MockedTime::ShiftedTime { microseconds: us }
            } else if time.is_none() {
                MockedTime::RealTime
            } else {
                return Err(pyo3::exceptions::PyTypeError::new_err("Invalid field time"));
            },
        );
        Ok(())
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    TimeProvider,
    libparsec::low_level::types::TimeProvider,
    __repr__,
    __copy__,
);

#[pymethods]
impl TimeProvider {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self(libparsec::low_level::types::TimeProvider::default()))
    }

    pub fn now(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.now()))
    }

    pub fn sleeping_stats(&self) -> PyResult<u64> {
        #[cfg(not(feature = "test-utils"))]
        {
            Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Test features are disabled !",
            ))
        }

        #[cfg(feature = "test-utils")]
        {
            Ok(self.0.sleeping_stats())
        }
    }

    // Booyakasha !
    pub fn sleep(&self, py: Python<'_>, time: f64) -> PyResult<PyObject> {
        // Unlike Trio, Tokio doesn't support sleeping forever.
        // Instead it sleeps for a long, long time (~2.2 years according to doc). So in
        // theory it should be fine for our needs, but better be extra-careful and do
        // *real* forever sleep (we already got our share of weird stuff with time mocking).
        if time == f64::INFINITY {
            return Ok(py
                .import("trio")?
                .getattr("sleep_forever")?
                .call0()?
                .into_py(py));
        }

        let time_provider = self.0.clone();
        let coroutine = FutureIntoCoroutine::from(async move {
            time_provider
                .sleep(libparsec::low_level::types::Duration::microseconds(
                    (time * 1e6) as i64,
                ))
                .await;
            Ok(())
        });
        Ok(coroutine.into_py(py))
    }

    pub fn new_child(&self) -> PyResult<TimeProvider> {
        #[cfg(not(feature = "test-utils"))]
        {
            Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Test features are disabled !",
            ))
        }

        #[cfg(feature = "test-utils")]
        {
            Ok(TimeProvider(self.0.new_child()))
        }
    }

    #[pyo3(signature = (freeze = None, shift = None, speed = None, realtime = false))]
    pub fn mock_time(
        &mut self,
        freeze: Option<DateTime>,
        shift: Option<f64>,
        speed: Option<f64>,
        realtime: bool,
    ) -> PyResult<()> {
        #[cfg(not(feature = "test-utils"))]
        {
            let _freeze = freeze;
            let _shift = shift;
            let _speed = speed;
            let _realtime = realtime;
            Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Test features are disabled !",
            ))
        }

        #[cfg(feature = "test-utils")]
        {
            use libparsec::low_level::types::MockedTime;
            let mock_config = match (freeze, shift, speed, realtime) {
                (None, None, None, true) => MockedTime::RealTime,
                (Some(dt), None, None, false) => MockedTime::FrozenTime(dt.0),
                (None, Some(shift_in_seconds), None, false) => MockedTime::ShiftedTime {
                    microseconds: (shift_in_seconds * 1e6) as i64,
                },
                (None, None, Some(speed_factor), false) => {
                    let reference = self.0.parent_now_or_realtime();
                    MockedTime::FasterTime {
                        reference,
                        microseconds: (self.0.now() - reference)
                            .num_microseconds()
                            .expect("No reason to overflow"),
                        speed_factor,
                    }
                }
                _ => {
                    return Err(PyValueError::new_err(
                        "Must only provide one of `freeze`, `shift`, `speed` and `realtime`",
                    ))
                }
            };
            self.0.mock_time(mock_config);
            Ok(())
        }
    }
}

crate::binding_utils::gen_py_wrapper_class!(
    DateTime,
    libparsec::low_level::types::DateTime,
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
        libparsec::low_level::types::DateTime::from_ymd_hms_us(
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
        )
        .single()
        .ok_or_else(|| PyAttributeError::new_err("Invalid attributes"))
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
    fn now(_cls: &PyType) -> PyResult<Self> {
        Ok(Self(libparsec::low_level::types::DateTime::now_legacy()))
    }

    fn timestamp(&self) -> PyResult<f64> {
        Ok(self.0.get_f64_with_us_precision())
    }

    #[classmethod]
    fn from_timestamp(_cls: &PyType, ts: f64) -> PyResult<Self> {
        Ok(Self(
            libparsec::low_level::types::DateTime::from_f64_with_us_precision(ts),
        ))
    }

    #[classmethod]
    fn from_rfc3339(_cls: &PyType, value: &str) -> PyResult<Self> {
        libparsec::low_level::types::DateTime::from_rfc3339(value)
            .map(Self)
            .map_err(|e| PyValueError::new_err(format!("Invalid rfc3339 date `{}`: {}", value, e)))
    }

    fn to_local(&self) -> PyResult<LocalDateTime> {
        Ok(LocalDateTime(self.0.to_local()))
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
                )))
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

crate::binding_utils::gen_py_wrapper_class!(
    LocalDateTime,
    libparsec::low_level::types::LocalDateTime,
    __repr__,
    __copy__,
    __deepcopy__,
    __str__,
    __richcmp__ ord,
    __hash__,
);

#[pymethods]
impl LocalDateTime {
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
        libparsec::low_level::types::LocalDateTime::from_ymd_hms_us(
            year,
            month,
            day,
            hour,
            minute,
            second,
            microsecond,
        )
        .single()
        .ok_or_else(|| PyAttributeError::new_err("Invalid attributes"))
        .map(Self)
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
            libparsec::low_level::types::LocalDateTime::from_f64_with_us_precision(ts),
        ))
    }

    fn to_utc(&self) -> PyResult<DateTime> {
        Ok(DateTime(self.0.to_utc()))
    }

    fn format(&self, fmt: &str) -> PyResult<String> {
        Ok(self.0.format(fmt))
    }
}
