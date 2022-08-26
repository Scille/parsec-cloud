// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod block;
mod cmds;

pub use block::*;
pub use cmds::*;

macro_rules! gen_rep {
    ($mod: ident, $base_class: ident $(, { $($tt: tt)+ })? $(, [$variant: ident])* $(,)?) => {
        paste::paste! {
            #[::pyo3::pyclass(subclass)]
            #[derive(Clone)]
            pub(crate) struct $base_class(pub $mod::Rep);

            #[::pyo3::pymethods]
            impl $base_class {
                fn __repr__(&self) -> ::pyo3::PyResult<String> {
                    Ok(format!("{:?}", self.0))
                }

                fn __richcmp__(&self, other: Self, op: ::pyo3::pyclass::CompareOp) -> ::pyo3::PyResult<bool> {
                    Ok(match op {
                        ::pyo3::pyclass::CompareOp::Eq => self.0 == other.0,
                        ::pyo3::pyclass::CompareOp::Ne => self.0 != other.0,
                        _ => return  Err(::pyo3::exceptions::PyNotImplementedError::new_err("Not implemented")),
                    })
                }

                fn dump<'py>(&self, py: ::pyo3::Python<'py>) -> ::pyo3::PyResult<&'py ::pyo3::types::PyBytes> {
                    Ok(::pyo3::types::PyBytes::new(
                        py,
                        &self.0.clone().dump().map_err(ProtocolError::new_err)?,
                    ))
                }

                #[classmethod]
                fn load<'py>(_cls: &::pyo3::types::PyType, buf: Vec<u8>, py: Python<'py>) -> ::pyo3::PyResult<PyObject> {
                    use pyo3::{pyclass_init::PyObjectInit, PyTypeInfo};

                    let rep = $mod::Rep::load(&buf).map_err(|e| ProtocolError::new_err(e.to_string()))?;
                    Ok(unsafe {
                        match rep {
                            rep @ $mod::Rep::Ok $({ $($tt)+ })? => {
                                let initializer =
                                    ::pyo3::PyClassInitializer::from(([<$base_class Ok>], Self(rep)));
                                let ptr = initializer.into_new_object(py, [<$base_class Ok>]::type_object_raw(py))?;
                                PyObject::from_owned_ptr(py, ptr)
                            }
                            $(
                                rep @ $mod::Rep::$variant => {
                                    let initializer =
                                        ::pyo3::PyClassInitializer::from(([<$base_class $variant>], Self(rep)));
                                    let ptr = initializer.into_new_object(py, [<$base_class $variant>]::type_object_raw(py))?;
                                    PyObject::from_owned_ptr(py, ptr)
                                }
                            )*
                            rep @ $mod::Rep::UnknownStatus { .. } => {
                                let initializer =
                                    ::pyo3::PyClassInitializer::from(([<$base_class UnknownStatus>], Self(rep)));
                                let ptr = initializer.into_new_object(py, [<$base_class UnknownStatus>]::type_object_raw(py))?;
                                PyObject::from_owned_ptr(py, ptr)
                            },
                        }
                    })
                }
            }

            $(
                #[::pyo3::pyclass(extends=$base_class)]
                pub(crate) struct [<$base_class $variant>];

                #[::pyo3::pymethods]
                impl [<$base_class $variant>] {
                    #[new]
                    fn new() -> ::pyo3::PyResult<(Self, $base_class)> {
                        Ok((Self, $base_class($mod::Rep::$variant)))
                    }
                }
            )*

            #[::pyo3::pyclass(extends=$base_class)]
            pub(crate) struct [<$base_class UnknownStatus>];

            #[::pyo3::pymethods]
            impl [<$base_class UnknownStatus>] {
                #[getter]
                fn status<'py>(
                    _self: ::pyo3::PyRef<'py, Self>,
                    py: ::pyo3::Python<'py>
                ) -> ::pyo3::PyResult<&'py ::pyo3::types::PyString> {
                    Ok(match &_self.as_ref().0 {
                        $mod::Rep::UnknownStatus { _status, .. } => ::pyo3::types::PyString::new(py, _status),
                        _ => return Err(::pyo3::exceptions::PyNotImplementedError::new_err("")),
                    })
                }

                #[getter]
                fn reason<'py>(
                    _self: ::pyo3::PyRef<'py, Self>,
                    py: ::pyo3::Python<'py>
                ) -> ::pyo3::PyResult<Option<&'py ::pyo3::types::PyString>> {
                    Ok(match &_self.as_ref().0 {
                        $mod::Rep::UnknownStatus { reason, .. } => reason.as_ref().map(|x| ::pyo3::types::PyString::new(py, x)),
                        _ => return Err(::pyo3::exceptions::PyNotImplementedError::new_err("")),
                    })
                }
            }
        }
    };
}

pub(crate) use gen_rep;
