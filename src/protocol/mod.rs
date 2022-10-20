// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod block;
mod cmds;
mod events;
mod invite;
mod message;
mod organization;
mod ping;
mod realm;
mod user;
mod vlob;

pub use block::*;
pub use cmds::*;
pub use events::*;
pub use invite::*;
pub use message::*;
pub use organization::*;
pub use ping::*;
pub use realm::*;
pub use user::*;
pub use vlob::*;

// We use this type because we can't match Option<String> in macro_rules
pub(crate) type Reason = Option<String>;
pub(crate) type Bytes = Vec<u8>;
pub(crate) type ListOfBytes = Vec<Vec<u8>>;
pub(crate) type OptionalFloat = Option<f64>;
pub(crate) type OptionalDateTime = Option<crate::time::DateTime>;

macro_rules! rs_to_py {
    ($v: ident, Reason, $py: ident) => {
        $v.as_ref().map(|x| ::pyo3::types::PyString::new($py, x))
    };
    ($v: ident, String, $py: ident) => {
        ::pyo3::types::PyString::new($py, $v)
    };

    ($v: ident, Bytes, $py: ident) => {
        ::pyo3::types::PyBytes::new($py, $v)
    };
    ($v: ident, ListOfBytes, $py: ident) => {
        ::pyo3::types::PyTuple::new($py, $v.iter().map(|x| ::pyo3::types::PyBytes::new($py, x)))
    };
    ($v: ident, DateTime, $py: ident) => {
        DateTime(*$v)
    };
    ($v: ident, SequesterServiceID, $py: ident) => {
        SequesterServiceID(*$v)
    };
    ($v: ident, OptionalDateTime, $py: ident) => {
        match *$v {
            libparsec::types::Maybe::Present(v) => Some(DateTime(v)),
            libparsec::types::Maybe::Absent => None,
        }
    };
    ($v: ident, OptionalFloat, $py: ident) => {
        match *$v {
            libparsec::types::Maybe::Present(v) => Some(v),
            libparsec::types::Maybe::Absent => None,
        }
    };
    ($v: ident, $ty: ty, $py: ident) => {
        *$v
    };
}

macro_rules! py_to_rs {
    ($v: ident, Reason) => {
        $v
    };
    ($v: ident, Bytes) => {
        $v
    };
    ($v: ident, ListOfBytes) => {
        $v
    };
    ($v: ident, f64) => {
        $v
    };
    ($v: ident, String) => {
        $v
    };
    ($v: ident, OptionalFloat) => {
        match $v {
            Some(v) => libparsec::types::Maybe::Present(v),
            None => libparsec::types::Maybe::Absent,
        }
    };
    ($v: ident, OptionalDateTime) => {
        match $v {
            Some(v) => libparsec::types::Maybe::Present(v.0),
            None => libparsec::types::Maybe::Absent,
        }
    };
    ($v: ident, $ty: ty) => {
        $v.0
    };
}

macro_rules! rs_to_py_ty {
    (Reason) => {
        Option<&'py ::pyo3::types::PyString>
    };
    (String) => {
        &'py ::pyo3::types::PyString
    };
    (Bytes) => {
        &'py ::pyo3::types::PyBytes
    };
    (ListOfBytes) => {
        &'py ::pyo3::types::PyTuple
    };
    ($ty: ty) => {
        $ty
    };
}

macro_rules! gen_rep {
    (
        $mod: path,
        $base_class: ident
        $(, { $($tt: tt)+ })?
        $(, [$variant: ident $($(, $field: ident : $ty: ty)+)? $(,)?])*
        $(,)?
    ) => {
        paste::paste! {
            #[::pyo3::pyclass(subclass)]
            #[derive(Clone)]
            pub(crate) struct $base_class(pub $mod::Rep);

            crate::binding_utils::gen_proto!($base_class, __repr__);
            crate::binding_utils::gen_proto!($base_class, __richcmp__, eq);

            #[::pyo3::pymethods]
            impl $base_class {
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

                    let ret = match rep {
                        rep @ $mod::Rep::Ok $({ $($tt)+ })? => {
                            let initializer =
                                ::pyo3::PyClassInitializer::from(([<$base_class Ok>], Self(rep)));
                            // SAFETY: `PyObjectInit::into_new_object` requires `subtype` used to generate a new object to be the same type
                            // or a sub-type of `T` (the type of `initializer` here).
                            // Here `initializer` is created using the type `<$base_class Ok>` and the same type of `<$base_class Ok>`
                            // will be used as the type of `subtype` in the call of `into_new_object`.
                            unsafe {
                                let ptr = initializer.into_new_object(py, [<$base_class Ok>]::type_object_raw(py))?;
                                PyObject::from_owned_ptr(py, ptr)
                            }
                        }
                        $(
                            rep @ $mod::Rep::$variant $({ $($field: _,)+ })? => {
                                let initializer =
                                    ::pyo3::PyClassInitializer::from(([<$base_class $variant>], Self(rep)));
                                // SAFETY: `PyObjectInit::into_new_object` requires `subtype` used to generate a new object to be the same type
                                // or a sub-type of `T` (the type of `initializer` here).
                                // Here `initializer` is created using the type `<$base_class $variant>` and the same type of `<$base_class $variant>`
                                // will be used as the type of `subtype` in the call of `into_new_object`.
                                unsafe {
                                    let ptr = initializer.into_new_object(py, [<$base_class $variant>]::type_object_raw(py))?;
                                    PyObject::from_owned_ptr(py, ptr)
                                }
                            }
                        )*
                        rep @ $mod::Rep::UnknownStatus { .. } => {
                            let initializer =
                                ::pyo3::PyClassInitializer::from(([<$base_class UnknownStatus>], Self(rep)));
                            // SAFETY: `PyObjectInit::into_new_object` requires `subtype` used to generate a new object to be the same type
                            // or a sub-type of `T` (the type of `initializer` here).
                            // Here `initializer` is created using the type `<$base_class UnknownStatus>` and the same type of `<$base_class UnknownStatus>`
                            // will be used as the type of `subtype` in the call of `into_new_object`.
                            unsafe {
                                let ptr = initializer.into_new_object(py, [<$base_class UnknownStatus>]::type_object_raw(py))?;
                                PyObject::from_owned_ptr(py, ptr)
                            }
                        },
                    };
                    Ok(match _cls.getattr("_post_load") {
                        Ok(post_load) => post_load.call1((ret.as_ref(py), ))?.into_py(py),
                        Err(_) => ret,
                    })
                }
            }

            $(
                #[::pyo3::pyclass(extends=$base_class)]
                pub(crate) struct [<$base_class $variant>];

                #[::pyo3::pymethods]
                impl [<$base_class $variant>] {
                    #[new]
                    fn new($($($field: $ty,)+)?) -> ::pyo3::PyResult<(Self, $base_class)> {
                        $($( let $field = crate::protocol::py_to_rs!($field, $ty); )+)?
                        Ok((Self, $base_class($mod::Rep::$variant $({ $($field, )+ })?)))
                    }

                    $($(
                        #[getter]
                        fn $field<'py>(
                            _self: ::pyo3::PyRef<'py, Self>,
                            _py: ::pyo3::Python<'py>
                        ) -> ::pyo3::PyResult<crate::protocol::rs_to_py_ty!($ty)> {
                            Ok(match &_self.as_ref().0 {
                                $mod::Rep::$variant { $field, .. } => crate::protocol::rs_to_py!($field, $ty, _py),
                                _ => return Err(::pyo3::exceptions::PyNotImplementedError::new_err("")),
                            })
                        }
                    )+)?
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
use py_to_rs;
use rs_to_py;
use rs_to_py_ty;
