// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::*;
use crate::utils::snake_to_pascal_case;

pub(crate) fn generate_protocol_cmds_family(cmds: Vec<JsonCmd>, family_name: &str) -> TokenStream {
    quote_cmds_family(&GenCmdsFamily::new(
        cmds,
        family_name,
        ReuseSchemaStrategy::Default,
    ))
}

fn quote_cmds_family(family: &GenCmdsFamily) -> TokenStream {
    let populate_mod_fn_name = format_ident!("{}_populate_mod", family.name);
    let family_mod_name = format_ident!("{}", &family.name);
    let family_mod_name_as_str = &family.name;

    let (versioned_cmds_populates, versioned_cmds_items): (Vec<TokenStream>, Vec<TokenStream>) =
        family
            .versions
            .iter()
            .sorted_by_key(|(v, _)| *v)
            .map(|(version, cmds)| quote_versioned_cmds(family, *version, cmds))
            .unzip();

    quote! {
        pub(crate) mod #family_mod_name {
            #![allow(clippy::too_many_arguments)]

            use ::pyo3::prelude::*;
            use ::pyo3::types::*;
            use libparsec_types::ProtocolRequest;
            use crate::*;
            use super::*;

            #(
            #versioned_cmds_items
            )*
            pub(super) fn #populate_mod_fn_name(py: Python<'_>, m: &PyModule) -> PyResult<()> {
                let family_mod = PyModule::new(py, #family_mod_name_as_str)?;
                m.add_submodule(family_mod)?;
                #(#versioned_cmds_populates)*

                // Version modules are returned ordered
                family_mod.add("latest", versioned_cmds_mod)?;

                Ok(())
            }
        }
        use #family_mod_name::#populate_mod_fn_name;
    }
}

fn quote_versioned_cmds(
    family: &GenCmdsFamily,
    version: u32,
    cmds: &[GenCmd],
) -> (TokenStream, TokenStream) {
    let family_mod_name = format_ident!("{}", &family.name);
    let versioned_cmds_mod_name_as_str = &format!("v{version}");
    let versioned_cmds_mod_name = format_ident!("{}", versioned_cmds_mod_name_as_str);
    let py_module_path_as_str = &format!("parsec._parsec.{}.v{}", family.name, version);
    let protocol_versioned_cmds_path =
        quote! { libparsec_protocol::#family_mod_name::#versioned_cmds_mod_name };

    let (cmds_populate_codes, cmds_impl_codes): (Vec<TokenStream>, Vec<TokenStream>) = cmds
        .iter()
        .map(|cmd| quote_cmd(family, version, cmd))
        .unzip();

    let populate_code = quote! {
        let versioned_cmds_mod = PyModule::new(py, #versioned_cmds_mod_name_as_str)?;
        family_mod.add_submodule(&versioned_cmds_mod)?;
        versioned_cmds_mod.add_class::<#versioned_cmds_mod_name::AnyCmdReq>()?;
        #(#cmds_populate_codes)*
    };

    let (any_cmd_values, any_cmd_reqs): (Vec<_>, Vec<_>) = cmds
        .iter()
        .map(|cmd| {
            let pascal_case_cmd_name = &snake_to_pascal_case(&cmd.cmd);
            let key = format_ident!("{}", &pascal_case_cmd_name);
            let cmd_name = format_ident!("{}", &cmd.cmd);
            let value = quote! { #cmd_name::Req };
            (key, value)
        })
        .unzip();

    let impl_code = quote! {
        pub(crate) mod #versioned_cmds_mod_name {
            use super::*;

            #(#cmds_impl_codes)*

            #[pyclass(module = #py_module_path_as_str)]
            #[derive(Clone)]
            pub struct AnyCmdReq(pub #protocol_versioned_cmds_path::AnyCmdReq);

            #[pymethods]
            impl AnyCmdReq {
                #[staticmethod]
                fn load(py: Python, raw: &[u8]) -> PyResult<PyObject> {
                    let cmd = #protocol_versioned_cmds_path::AnyCmdReq::load(raw).map_err(|e| PyValueError::new_err(e.to_string()))?;
                    Ok(match cmd {
                        #(
                            #protocol_versioned_cmds_path::AnyCmdReq::#any_cmd_values(x) => #any_cmd_reqs(x).into_py(py),
                        )*
                    })
                }
            }
        }
    };

    (populate_code, impl_code)
}

fn quote_cmd(family: &GenCmdsFamily, version: u32, cmd: &GenCmd) -> (TokenStream, TokenStream) {
    let family_name = format_ident!("{}", family.name);
    let version_name = format_ident!("v{}", version);
    let cmd_mod_name_as_str = &cmd.cmd;
    let cmd_mod_name = format_ident!("{}", cmd_mod_name_as_str);
    let protocol_path = quote! { libparsec_protocol::#family_name::#version_name::#cmd_mod_name };

    match &cmd.spec {
        GenCmdSpec::ReusedFromVersion {
            version: reused_version,
        } => {
            let reused_version_name = format!("v{}", reused_version);
            let reused_version_mod = format_ident!("{}", &reused_version_name);

            let populate_code = quote! {
                let reuse = family_mod
                    .getattr(#reused_version_name)?
                    .getattr(#cmd_mod_name_as_str)?;
                versioned_cmds_mod.setattr(#cmd_mod_name_as_str, reuse)?;
            };

            let impl_code = quote! {
                pub(crate) use super::#reused_version_mod::#cmd_mod_name;
            };

            (populate_code, impl_code)
        }

        GenCmdSpec::Original {
            req,
            reps,
            nested_types,
        } => {
            let py_module_path_as_str =
                &format!("parsec._parsec.{}.v{}.{}", family.name, version, cmd.cmd);
            let mut nested_types_names = vec![];
            let mut nested_types_impl_codes = vec![];
            for nested_type in nested_types {
                let (ntns, ntics) =
                    quote_nested_type(nested_type, &protocol_path, py_module_path_as_str);
                nested_types_names.extend(ntns);
                nested_types_impl_codes.push(ntics);
            }

            let req_impl_code = quote_req(req, &protocol_path, py_module_path_as_str);
            let (reps_variants_names, reps_impl_code) =
                quote_reps(reps, &protocol_path, py_module_path_as_str);

            let populate_code = quote! {
                let cmd_mod = PyModule::new(py, #cmd_mod_name_as_str)?;
                versioned_cmds_mod.add_submodule(&cmd_mod)?;
                cmd_mod.add_class::<#version_name::#cmd_mod_name::Req>()?;
                #(cmd_mod.add_class::<#version_name::#cmd_mod_name::#nested_types_names>()?;)*
                cmd_mod.add_class::<#version_name::#cmd_mod_name::Rep>()?;
                #(cmd_mod.add_class::<#version_name::#cmd_mod_name::#reps_variants_names>()?;)*
            };

            let impl_code = quote! {
                pub(crate) mod #cmd_mod_name{
                    use super::*;

                    #(#nested_types_impl_codes)*
                    #req_impl_code
                    #reps_impl_code
                }

            };

            (populate_code, impl_code)
        }
    }
}

fn quote_reps(
    reps: &[GenCmdRep],
    protocol_path: &TokenStream,
    py_module_path_as_str: &str,
) -> (Vec<Ident>, TokenStream) {
    let mut reps_variants_names = vec![format_ident!("RepUnknownStatus")];

    let reps_impl_codes = reps.iter().map(|rep| {
        let pascal_case_rep_status = snake_to_pascal_case(&rep.status);
        let variant_name = format_ident!("{}", &pascal_case_rep_status);
        let subclass_name = format_ident!("Rep{}", &pascal_case_rep_status);
        reps_variants_names.push(subclass_name.clone());

        match &rep.kind {
            GenCmdRepKind::Unit { nested_type_name } => {
                let nested_type_name = format_ident!("{}", nested_type_name);
                quote!{
                    #[pyclass(module = #py_module_path_as_str, extends = Rep)]
                    #[derive(Clone)]
                    pub(crate) struct #subclass_name;

                    #[pymethods]
                    impl #subclass_name {
                        #[new]
                        #[pyo3(signature = (unit))]
                        fn new(unit: #nested_type_name) -> PyResult<(Self, Rep)> {
                            Ok((
                                Self,
                                Rep(#protocol_path::Rep::#variant_name (unit.0))
                            ))
                        }

                        #[getter]
                        fn unit(_self: PyRef<Self>, py: Python) -> PyResult<PyObject> {
                            match &_self.into_super().0 {
                                #protocol_path::Rep::#variant_name ( unit ) => {
                                    #nested_type_name::from_raw(py, unit.clone())
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
            GenCmdRepKind::Composed { fields } => {
                let fn_new_impl = {
                    let params = fields.iter().map(|f| {
                        let name = format_ident!("{}", &f.name);
                        let ty = quote_type_as_fn_new_param(&f.ty);
                        quote! { #name: #ty }
                    });
                    let names = fields.iter().map(|field| format_ident!("{}", &field.name));
                    let signature = quote! { #(#names),* };
                    let conversions = fields.iter().map(quote_field_as_fn_new_conversion);

                    quote! {
                        #[new]
                        #[pyo3(signature = (#signature))]
                        fn new<'py>(py: Python<'py>, #(#params),*) -> PyResult<(Self, Rep)> {
                            #(#conversions)*
                            Ok((
                                Self,
                                Rep(#protocol_path::Rep::#variant_name {
                                    #signature
                                })
                            ))
                        }
                    }
                };

                let fn_getters_impls = fields.iter().map(|f| {
                    let field_name = format_ident!("{}", f.name);
                    let field_getter_ret_type = quote_type_as_fn_getter_ret_type(&f.ty);

                    let inner_impl = if f.added_in_minor_revision {
                        let access_field_path = quote! { x };
                        let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &f.ty);
                        quote! {
                            match x {
                                libparsec_types::Maybe::Present(x) => Ok(#conversion),
                                libparsec_types::Maybe::Absent => Err(PyAttributeError::new_err("")),
                            }
                        }
                    } else {
                        let access_field_path = quote! { x };
                        let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &f.ty);
                        quote! {
                            Ok(#conversion)
                        }
                    };

                    quote!{
                        #[getter]
                        fn #field_name<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<#field_getter_ret_type> {
                            match &_self.into_super().0 {
                                #protocol_path::Rep::#variant_name { #field_name: x, .. } => {
                                    #inner_impl
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                });

                quote!{
                    #[pyclass(module = #py_module_path_as_str, extends = Rep)]
                    #[derive(Clone)]
                    pub(crate) struct #subclass_name;

                    #[pymethods]
                    impl #subclass_name {
                        #(#fn_getters_impls)*
                        #fn_new_impl
                    }
                }
            }
        }
    });

    let fn_load_subclasses_matches_codes = reps.iter().map(|rep| {
        let pascal_case_rep_status = snake_to_pascal_case(&rep.status);
        let variant_name = format_ident!("{}", &pascal_case_rep_status);
        let subclass_name = format_ident!("Rep{}", &pascal_case_rep_status);
        quote! {
            raw_rep @ #protocol_path::Rep::#variant_name {..} => {
                let init = PyClassInitializer::from(Rep(raw_rep));
                Py::new(py, init.add_subclass(#subclass_name))?.into_py(py)
            }
        }
    });

    let impl_code = quote! {
        #[pyclass(module = #py_module_path_as_str, subclass)]
        #[derive(Clone)]
        pub(crate) struct Rep(pub #protocol_path::Rep);

        crate::binding_utils::gen_proto!(Rep, __repr__);
        crate::binding_utils::gen_proto!(Rep, __copy__);
        crate::binding_utils::gen_proto!(Rep, __deepcopy__);
        crate::binding_utils::gen_proto!(Rep, __richcmp__, eq);

        #[pymethods]
        impl Rep {
            #[staticmethod]
            fn load<'py>(py: Python<'py>, raw: &[u8]) -> PyResult<PyObject> {
                let rep = #protocol_path::Rep::load(raw).map_err(|e| PyValueError::new_err(e.to_string()))?;
                Ok(match rep {
                    #(#fn_load_subclasses_matches_codes)*
                    raw_rep @ #protocol_path::Rep::UnknownStatus { .. } => {
                        let init = PyClassInitializer::from(Rep(raw_rep));
                        Py::new(py, init.add_subclass(RepUnknownStatus))?.into_py(py)
                    }
                })
            }

            fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
                Ok(PyBytes::new(
                    py,
                    &self.0.dump().map_err(|e| PyValueError::new_err(e.to_string()))?,
                ))
            }
        }

        #[pyclass(extends = Rep)]
        #[derive(Clone)]
        pub(crate) struct RepUnknownStatus;

        #[pymethods]
        impl RepUnknownStatus {
            #[new]
            #[pyo3(signature = (status, reason))]
            fn new(status: String, reason: Option<String>) -> PyResult<(Self, Rep)> {
                Ok((
                    Self,
                    Rep(#protocol_path::Rep::UnknownStatus { unknown_status: status, reason })
                ))
            }

            #[getter]
            fn status(_self: PyRef<Self>) -> String {
                match &_self.into_super().0 {
                    #protocol_path::Rep::UnknownStatus { unknown_status, .. } => {
                        unknown_status.to_owned()
                    }
                    _ => unreachable!(),
                }
            }

            #[getter]
            fn reason(_self: PyRef<Self>) -> Option<String> {
                match &_self.into_super().0 {
                    #protocol_path::Rep::UnknownStatus { reason, .. } => {
                        reason.to_owned()
                    }
                    _ => unreachable!(),
                }
            }
        }

        #(#reps_impl_codes)*
    };

    (reps_variants_names, impl_code)
}

fn quote_req(
    req: &GenCmdReq,
    protocol_path: &TokenStream,
    py_module_path_as_str: &str,
) -> TokenStream {
    match &req.kind {
        GenCmdReqKind::Unit { nested_type_name } => {
            let nested_type_name = format_ident!("{}", nested_type_name);
            quote! {
                #[pyclass(module = #py_module_path_as_str)]
                #[derive(Clone)]
                pub(crate) struct Req(pub #protocol_path::Req);

                crate::binding_utils::gen_proto!(Req, __repr__);
                crate::binding_utils::gen_proto!(Req, __copy__);
                crate::binding_utils::gen_proto!(Req, __deepcopy__);
                crate::binding_utils::gen_proto!(Req, __richcmp__, eq);

                #[pymethods]
                impl Req {
                    #[new]
                    #[pyo3(signature = (unit))]
                    fn new(unit: #nested_type_name) -> PyResult<Self> {
                        Ok(Self(
                            #protocol_path::Req(unit.0)
                        ))
                    }

                    #[getter]
                    fn unit(&self, py: Python) -> PyResult<PyObject> {
                        #nested_type_name::from_raw(py, self.0.0.clone())
                    }

                    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
                        Ok(PyBytes::new(
                            py,
                            &self.0.dump().map_err(|e| PyValueError::new_err(e.to_string()))?,
                        ))
                    }
                }
            }
        }

        GenCmdReqKind::Composed { fields } => {
            let params = fields.iter().map(|field| {
                let name = format_ident!("{}", field.name);
                let ty = quote_type_as_fn_new_param(&field.ty);
                quote! { #name: #ty }
            });
            let conversions = fields.iter().map(quote_field_as_fn_new_conversion);
            let names = fields.iter().map(|field| format_ident!("{}", &field.name));
            let signature = quote! { #(#names),* };

            let fn_getters_impls = fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.name);
                let field_getter_ret_type = quote_type_as_fn_getter_ret_type(&field.ty);

                let fn_body = if field.added_in_minor_revision {
                    let access_field_path = quote! { x };
                    let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &field.ty);
                    quote! {
                        match &self.0.#field_name {
                            libparsec_types::Maybe::Present(x) => Ok(#conversion),
                            libparsec_types::Maybe::Absent => Err(PyAttributeError::new_err("")),
                        }
                    }
                } else {
                    let access_field_path = quote! { (&self.0.#field_name) };
                    let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &field.ty);
                    quote! {
                        Ok(#conversion)
                    }
                };

                quote! {
                    #[getter]
                    fn #field_name<'py>(&'py self, py: Python<'py>) -> PyResult<#field_getter_ret_type> {
                        #fn_body
                    }
                }
            });

            let fn_new_and_getters_iml = quote! {
                #[new]
                #[pyo3(signature = (#signature))]
                fn new(#(#params),*) -> PyResult<Self> {
                    #(#conversions)*
                    Ok(Self(
                        #protocol_path::Req {
                            #signature
                        }
                    ))
                }
                #(#fn_getters_impls)*
            };

            quote! {
                #[pyclass(module = #py_module_path_as_str)]
                #[derive(Clone)]
                pub(crate) struct Req(pub #protocol_path::Req);

                crate::binding_utils::gen_proto!(Req, __repr__);
                crate::binding_utils::gen_proto!(Req, __copy__);
                crate::binding_utils::gen_proto!(Req, __deepcopy__);
                crate::binding_utils::gen_proto!(Req, __richcmp__, eq);

                #[pymethods]
                impl Req {
                    #fn_new_and_getters_iml

                    fn dump<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
                        Ok(PyBytes::new(
                            py,
                            &self.0.dump().map_err(|e| PyValueError::new_err(e.to_string()))?,
                        ))
                    }
                }
            }
        }
    }
}

fn quote_nested_type(
    nested_type: &GenCmdNestedType,
    protocol_path: &TokenStream,
    py_module_path_as_str: &str,
) -> (Vec<Ident>, TokenStream) {
    match nested_type {
        GenCmdNestedType::LiteralsUnion { name, variants } => {
            let nested_type_name = format_ident!("{}", name);

            let enum_variants = variants.iter().map(|(name, value)| {
                let value_literal = &value;
                let value_as_fn_name = format_ident!("{}", value.to_ascii_lowercase());
                let name = format_ident!("{}", name);
                quote! {
                    [
                        #value_literal,
                        #value_as_fn_name,
                        #protocol_path::#nested_type_name::#name
                    ]
                }
            });

            let impl_code = quote! {
                crate::binding_utils::gen_py_wrapper_class_for_enum!(
                    #nested_type_name,
                    #protocol_path::#nested_type_name,
                    #(#enum_variants),*
                );

                impl #nested_type_name {
                    fn from_raw(py: Python, raw: #protocol_path::#nested_type_name) -> PyResult<PyObject> {
                        Ok(Self::convert(raw).as_ref(py).into())
                    }
                }
            };

            (vec![nested_type_name], impl_code)
        }

        GenCmdNestedType::StructsUnion { name, variants, .. } => {
            let nested_type_name = format_ident!("{}", name);
            let mut class_and_subclasses_names = vec![nested_type_name.clone()];
            let class_and_subclasses_names_ref = &mut class_and_subclasses_names;

            let subclass_impls = variants.iter().map(|v| {
                let variant_name = format_ident!("{}", v.name);
                let subclass_name = format_ident!("{}{}", name, v.name);
                class_and_subclasses_names_ref.push(subclass_name.clone());

                let fn_new_impl = {
                    let params = v.fields.iter().map(|f| {
                        let name = format_ident!("{}", &f.name);
                        let ty = quote_type_as_fn_new_param(&f.ty);
                        quote! { #name: #ty }
                    });
                    let names = v.fields.iter().map(|field| format_ident!("{}", &field.name));
                    let signature = quote! { #(#names),* };
                    let conversions = v.fields.iter().map(quote_field_as_fn_new_conversion);

                    quote! {
                        #[new]
                        #[pyo3(signature = (#signature))]
                        fn new<'py>(py: Python<'py>, #(#params),*) -> PyResult<(Self, #nested_type_name)> {
                            #(#conversions)*
                            Ok((
                                Self,
                                #nested_type_name(#protocol_path::#nested_type_name::#variant_name {
                                    #signature
                                })
                            ))
                        }
                    }
                };

                let fn_getters_impls = v.fields.iter().map(|f| {
                    let field_name = format_ident!("{}", f.name);
                    let field_getter_ret_type = quote_type_as_fn_getter_ret_type(&f.ty);

                    let inner_impl = if f.added_in_minor_revision {
                        let access_field_path = quote! { x };
                        let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &f.ty);
                        quote! {
                            match x {
                                libparsec_types::Maybe::Present(x) => Ok(#conversion),
                                libparsec_types::Maybe::Absent => Err(PyAttributeError::new_err("")),
                            }
                        }
                    } else {
                        let access_field_path = quote! { x };
                        let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &f.ty);
                        quote! {
                            {
                                Ok(#conversion)
                            }
                        }
                    };

                    quote!{
                        #[getter]
                        fn #field_name<'py>(_self: PyRef<'py, Self>, py: Python<'py>) -> PyResult<#field_getter_ret_type> {
                            match &_self.into_super().0 {
                                #protocol_path::#nested_type_name::#variant_name { #field_name: x, .. } => {
                                    #inner_impl
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                });

                quote!{
                    #[pyclass(module = #py_module_path_as_str, extends = #nested_type_name)]
                    #[derive(Clone)]
                    pub(crate) struct #subclass_name;

                    #[pymethods]
                    impl #subclass_name {
                        #fn_new_impl
                        #(#fn_getters_impls)*
                    }
                }
            });
            let from_subclasses_matches_codes = variants.iter().map(|v| {
                let variant_name = format_ident!("{}", v.name);
                let subclass_name = format_ident!("{}{}", name, v.name);
                quote! {
                    raw @ #protocol_path::#nested_type_name::#variant_name {..} => {
                        let init = PyClassInitializer::from(#nested_type_name(raw));
                        Ok(Py::new(py, init.add_subclass(#subclass_name))?.into_py(py))
                    }
                }
            });

            let impl_code = quote! {
                #[pyclass(module = #py_module_path_as_str, subclass)]
                #[derive(Clone)]
                pub(crate) struct #nested_type_name(pub #protocol_path::#nested_type_name);

                crate::binding_utils::gen_proto!(#nested_type_name, __repr__);
                crate::binding_utils::gen_proto!(#nested_type_name, __copy__);
                crate::binding_utils::gen_proto!(#nested_type_name, __deepcopy__);
                crate::binding_utils::gen_proto!(#nested_type_name, __richcmp__, eq);

                impl #nested_type_name {
                    fn from_raw(py: Python, raw: #protocol_path::#nested_type_name) -> PyResult<PyObject> {
                        match raw {
                            #(#from_subclasses_matches_codes)*
                        }
                    }
                }

                #(#subclass_impls)*
            };

            (class_and_subclasses_names, impl_code)
        }

        GenCmdNestedType::Struct { name, fields } => {
            let nested_type_name = format_ident!("{}", name);

            let fn_new_impl = {
                let params = fields.iter().map(|field| {
                    let name = format_ident!("{}", &field.name);
                    let ty = quote_type_as_fn_new_param(&field.ty);
                    quote! { #name: #ty }
                });
                let conversions = fields.iter().map(quote_field_as_fn_new_conversion);
                let names = fields.iter().map(|field| format_ident!("{}", field.name));
                let signature = quote! { #(#names),* };

                quote! {
                    #[new]
                    #[pyo3(signature = (#signature))]
                    fn new(#(#params),*) -> PyResult<Self> {
                        #(#conversions)*
                        Ok(Self(
                            #protocol_path::#nested_type_name {
                                #signature
                            }
                        ))
                    }
                }
            };

            let fn_getters_impls = fields.iter().map(|field| {
                let field_name = format_ident!("{}", field.name);
                let field_getter_ret_type = quote_type_as_fn_getter_ret_type(&field.ty);

                let fn_body = if field.added_in_minor_revision {
                    let access_field_path = quote! { x };
                    let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &field.ty);
                    quote! {
                        match &self.0.#field_name {
                            libparsec_types::Maybe::Present(x) => Ok(#conversion),
                            libparsec_types::Maybe::Absent => Err(PyAttributeError::new_err("")),
                        }
                    }
                } else {
                    let access_field_path = quote! { (&self.0.#field_name) };
                    let conversion = quote_type_as_fn_getter_conversion(&access_field_path, &field.ty);
                    quote! {
                        {
                            Ok(#conversion)
                        }
                    }
                };

                quote! {
                    #[getter]
                    fn #field_name<'py>(&'py self, py: Python<'py>) -> PyResult<#field_getter_ret_type> {
                        #fn_body
                    }
                }
            });

            let impl_code = quote! {
                #[pyclass(module = #py_module_path_as_str)]
                #[derive(Clone)]
                pub(crate) struct #nested_type_name(pub #protocol_path::#nested_type_name);

                crate::binding_utils::gen_proto!(#nested_type_name, __repr__);
                crate::binding_utils::gen_proto!(#nested_type_name, __copy__);
                crate::binding_utils::gen_proto!(#nested_type_name, __deepcopy__);
                crate::binding_utils::gen_proto!(#nested_type_name, __richcmp__, eq);

                impl #nested_type_name {
                    fn from_raw(py: Python, raw: #protocol_path::#nested_type_name) -> PyResult<PyObject> {
                        Ok(Py::new(py, #nested_type_name(raw))?.into_py(py))
                    }
                }

                #[pymethods]
                impl #nested_type_name {
                    #fn_new_impl
                    #(#fn_getters_impls)*
                }
            };

            (vec![nested_type_name], impl_code)
        }
    }
}

fn quote_type_as_fn_getter_ret_type(ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::Map(key, value) => {
            let key = quote_type_as_fn_getter_ret_type(key);
            let value = quote_type_as_fn_getter_ret_type(value);
            quote! { std::collections::HashMap<#key, #value> }
        }
        FieldType::List(nested) => {
            let nested = quote_type_as_fn_getter_ret_type(nested);
            quote! { Vec<#nested> }
        }
        FieldType::Set(nested) => {
            let nested = quote_type_as_fn_getter_ret_type(nested);
            quote! { std::collections::HashSet<#nested> }
        }
        FieldType::RequiredOption(nested) => {
            let nested = quote_type_as_fn_getter_ret_type(nested);
            quote! { Option<#nested> }
        }
        FieldType::NonRequiredOption(nested) => {
            let nested = quote_type_as_fn_getter_ret_type(nested);
            quote! { Option<#nested> }
        }
        FieldType::Tuple(_) => {
            quote! { &'py PyTuple }
        }
        FieldType::Custom(nested) => {
            let nested = format_ident!("{}", nested);
            quote! { #nested }
        }
        FieldType::Boolean => quote! { bool },
        FieldType::String => quote! { &'py PyString },
        FieldType::Bytes => quote! { &'py PyBytes },
        FieldType::Integer => quote! { i64 },
        FieldType::Float => quote! { f64 },
        FieldType::Version => quote! { u32 },
        FieldType::Size => quote! { u64},
        FieldType::Index => quote! { u64 },
        FieldType::NonZeroInteger => quote! { u64 },
        FieldType::IntegerBetween1And100 => quote! { u64 },
        FieldType::PublicKey => quote! { crate::crypto::PublicKey },
        FieldType::SigningKey => quote! { crate::crypto::SigningKey },
        FieldType::VerifyKey => quote! { crate::crypto::VerifyKey },
        FieldType::PrivateKey => quote! { crate::crypto::PrivateKey },
        FieldType::SecretKey => quote! { crate::crypto::SecretKey },
        FieldType::HashDigest => quote! { crate::crypto::HashDigest },
        FieldType::SequesterVerifyKeyDer => quote! { crate::crypto::SequesterVerifyKeyDer },
        FieldType::SequesterPublicKeyDer => quote! { crate::crypto::SequesterPublicKeyDer },
        FieldType::DateTime => quote! { crate::time::DateTime },
        FieldType::BlockID => quote! { crate::ids::BlockID },
        FieldType::DeviceID => quote! { crate::ids::DeviceID },
        FieldType::OrganizationID => quote! { crate::ids::OrganizationID },
        FieldType::UserID => quote! { crate::ids::UserID },
        FieldType::VlobID => quote! { crate::ids::VlobID },
        FieldType::EnrollmentID => quote! { crate::ids::EnrollmentID },
        FieldType::SequesterServiceID => quote! { crate::ids::SequesterServiceID },
        FieldType::DeviceLabel => quote! { crate::ids::DeviceLabel },
        FieldType::HumanHandle => quote! { crate::ids::HumanHandle },
        FieldType::UserProfile => quote! { &'static PyObject },
        FieldType::RealmRole => quote! { &'static PyObject },
        FieldType::BootstrapToken => quote! { crate::ids::BootstrapToken },
        FieldType::InvitationToken => quote! { crate::ids::InvitationToken },
        FieldType::InvitationStatus => quote! { &'static PyObject },
        FieldType::CertificateSignerOwned => quote! { crate::certif::CertificateSignerOwned },
        FieldType::BlockAccess => quote! { crate::data::BlockAccess },
        FieldType::EntryName => quote! { crate::data::EntryName },
        FieldType::WorkspaceEntry => quote! { crate::data::WorkspaceEntry },
        FieldType::FileManifest => quote! { crate::data::FileManifest },
        FieldType::FolderManifest => quote! { crate::data::FolderManifest },
        FieldType::WorkspaceManifest => quote! { crate::data::WorkspaceManifest },
        FieldType::UserManifest => quote! { crate::data::UserManifest },
        FieldType::ActiveUsersLimit => quote! { crate::protocol::ActiveUsersLimit },
        FieldType::Chunk => quote! { crate::data::Chunk },
        FieldType::BackendOrganizationAddr => quote! { crate::addrs::BackendOrganizationAddr },
        FieldType::UsersPerProfileDetailItem => quote! { crate::data::UsersPerProfileDetailItem },
        FieldType::BackendPkiEnrollmentAddr => quote! { crate::addrs::BackendPkiEnrollmentAddr },
        FieldType::PkiEnrollmentSubmitPayload => quote! { crate::data::PkiEnrollmentSubmitPayload },
        FieldType::X509Certificate => quote! { crate::data::X509Certificate },
    }
}

fn quote_type_as_fn_getter_conversion(field_path: &TokenStream, ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::Map(key, value) => {
            let key_name = quote! { k };
            let value_name = quote! { v };
            let conversion_key = quote_type_as_fn_getter_conversion(&key_name, key);
            let conversion_value = quote_type_as_fn_getter_conversion(&value_name, value);
            quote! {
                #field_path.iter().map(|(k, v)| {
                    let k = #conversion_key;
                    let v = #conversion_value;
                    (k, v)
                }).collect::<std::collections::HashMap<_, _>>()
            }
        }
        FieldType::List(nested) => {
            let nested_name = quote! { y };
            let conversion = quote_type_as_fn_getter_conversion(&nested_name, nested);
            quote! {
                #field_path.iter().map(|y| #conversion).collect::<Vec<_>>()
            }
        }
        FieldType::Set(nested) => {
            let nested_name = quote! { y };
            let conversion = quote_type_as_fn_getter_conversion(&nested_name, nested);
            quote! {
                #field_path.iter().map(|y| #conversion).collect::<std::collections::HashSet<_>>()
            }
        }
        FieldType::RequiredOption(nested) => {
            let nested_name = quote! { y };
            let nested_conversion = quote_type_as_fn_getter_conversion(&nested_name, nested);
            quote! { #field_path.as_ref().map(|y| #nested_conversion) }
        }
        FieldType::NonRequiredOption(nested) => {
            let nested_name = quote! { y };
            let nested_conversion = quote_type_as_fn_getter_conversion(&nested_name, nested);
            quote! { #field_path.as_ref().map(|y| #nested_conversion) }
        }
        FieldType::Tuple(items) => {
            let (items_names, items_conversions): (Vec<_>, Vec<_>) = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let item_name = format_ident!("x{}", i);
                    let item_name = quote! { #item_name };
                    let conversion = quote_type_as_fn_getter_conversion(&item_name, item);
                    (item_name, conversion)
                })
                .unzip();
            quote! {
                {
                    let (#(#items_names),*) = #field_path;
                    PyTuple::new(py, [
                        #(#items_conversions.into_py(py)),*
                    ])
                }
            }
        }
        FieldType::Custom(nested) => {
            let nested_name = format_ident!("{}", nested);
            quote! { #nested_name(#field_path.to_owned()) }
        }
        FieldType::String => quote! { PyString::new(py, #field_path) },
        FieldType::Bytes => quote! { PyBytes::new(py, #field_path) },
        FieldType::Boolean => quote! { bool::from(*#field_path) },
        FieldType::Integer => quote! { i64::from(*#field_path) },
        FieldType::Float => quote! { f64::from(*#field_path) },
        FieldType::Version => quote! { u32::from(*#field_path) },
        FieldType::Size => quote! { u64::from(*#field_path) },
        FieldType::Index => quote! { u64::from(*#field_path) },
        FieldType::NonZeroInteger => quote! { u64::from(*#field_path) },
        FieldType::IntegerBetween1And100 => quote! { u64::from(*#field_path) },
        FieldType::PublicKey => quote! { crate::crypto::PublicKey(#field_path.to_owned()) },
        FieldType::SigningKey => quote! { crate::crypto::SigningKey(#field_path.to_owned()) },
        FieldType::VerifyKey => quote! { crate::crypto::VerifyKey(#field_path.to_owned()) },
        FieldType::PrivateKey => quote! { crate::crypto::PrivateKey(#field_path.to_owned()) },
        FieldType::SecretKey => quote! { crate::crypto::SecretKey(#field_path.to_owned()) },
        FieldType::HashDigest => quote! { crate::crypto::HashDigest(#field_path.to_owned()) },
        FieldType::SequesterVerifyKeyDer => {
            quote! { crate::crypto::SequesterVerifyKeyDer(#field_path.to_owned()) }
        }
        FieldType::SequesterPublicKeyDer => {
            quote! { crate::crypto::SequesterPublicKeyDer(#field_path.to_owned()) }
        }
        FieldType::DateTime => quote! { crate::time::DateTime(#field_path.to_owned()) },
        FieldType::BlockID => quote! { crate::ids::BlockID(#field_path.to_owned()) },
        FieldType::DeviceID => quote! { crate::ids::DeviceID(#field_path.to_owned()) },
        FieldType::OrganizationID => quote! { crate::ids::OrganizationID(#field_path.to_owned()) },
        FieldType::UserID => quote! { crate::ids::UserID(#field_path.to_owned()) },
        FieldType::VlobID => quote! { crate::ids::VlobID(#field_path.to_owned()) },
        FieldType::EnrollmentID => quote! { crate::ids::EnrollmentID(#field_path.to_owned()) },
        FieldType::SequesterServiceID => {
            quote! { crate::ids::SequesterServiceID(#field_path.to_owned()) }
        }
        FieldType::DeviceLabel => quote! { crate::ids::DeviceLabel(#field_path.to_owned()) },
        FieldType::HumanHandle => quote! { crate::ids::HumanHandle(#field_path.to_owned()) },
        FieldType::UserProfile => {
            quote! { crate::enumerate::UserProfile::convert(#field_path.to_owned()) }
        }
        FieldType::RealmRole => {
            quote! { crate::enumerate::RealmRole::convert(#field_path.to_owned()) }
        }
        FieldType::BootstrapToken => {
            quote! { crate::ids::BootstrapToken(#field_path.to_owned()) }
        }
        FieldType::InvitationToken => {
            quote! { crate::ids::InvitationToken(#field_path.to_owned()) }
        }
        FieldType::InvitationStatus => {
            quote! { crate::enumerate::InvitationStatus::convert(#field_path.to_owned()) }
        }
        FieldType::CertificateSignerOwned => {
            quote! { crate::certif::CertificateSignerOwned(#field_path.to_owned()) }
        }
        FieldType::BlockAccess => quote! { crate::data::BlockAccess(#field_path.to_owned()) },
        FieldType::EntryName => quote! { crate::data::EntryName(#field_path.to_owned()) },
        FieldType::WorkspaceEntry => quote! { crate::data::WorkspaceEntry(#field_path.to_owned()) },
        FieldType::FileManifest => quote! { crate::data::FileManifest(#field_path.to_owned()) },
        FieldType::FolderManifest => quote! { crate::data::FolderManifest(#field_path.to_owned()) },
        FieldType::WorkspaceManifest => {
            quote! { crate::data::WorkspaceManifest(#field_path.to_owned()) }
        }
        FieldType::UserManifest => quote! { crate::data::UserManifest(#field_path.to_owned()) },
        FieldType::ActiveUsersLimit => {
            quote! { crate::protocol::ActiveUsersLimit(#field_path.to_owned()) }
        }
        FieldType::Chunk => quote! { crate::data::Chunk(#field_path.to_owned()) },
        FieldType::BackendOrganizationAddr => {
            quote! { crate::addrs::BackendOrganizationAddr(#field_path.to_owned()) }
        }
        FieldType::UsersPerProfileDetailItem => {
            quote! { crate::data::UsersPerProfileDetailItem(#field_path.to_owned()) }
        }
        FieldType::BackendPkiEnrollmentAddr => {
            quote! { crate::addrs::BackendPkiEnrollmentAddr(#field_path.to_owned()) }
        }
        FieldType::PkiEnrollmentSubmitPayload => {
            quote! { crate::data::PkiEnrollmentSubmitPayload(#field_path.to_owned()) }
        }
        FieldType::X509Certificate => {
            quote! { crate::data::X509Certificate(#field_path.to_owned()) }
        }
    }
}

fn quote_type_as_fn_new_param(ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::Map(key, value) => {
            let key = quote_type_as_fn_new_param(key);
            let value = quote_type_as_fn_new_param(value);
            quote! { std::collections::HashMap<#key, #value> }
        }
        FieldType::List(nested) => {
            let nested = quote_type_as_fn_new_param(nested);
            quote! { Vec<#nested> }
        }
        FieldType::Set(nested) => {
            let nested = quote_type_as_fn_new_param(nested);
            quote! { HashSet<#nested> }
        }
        FieldType::RequiredOption(nested) => {
            let nested = quote_type_as_fn_new_param(nested);
            quote! { Option<#nested> }
        }
        FieldType::NonRequiredOption(nested) => {
            let nested = quote_type_as_fn_new_param(nested);
            quote! { Option<#nested> }
        }
        FieldType::Tuple(items) => {
            let nested = items.iter().map(quote_type_as_fn_new_param);
            quote! { (#(#nested),*) }
        }
        FieldType::Custom(nested) => {
            let nested = format_ident!("{}", nested);
            quote! { #nested }
        }
        FieldType::Boolean => quote! { bool },
        FieldType::String => quote! { String },
        FieldType::Bytes => quote! { crate::binding_utils::BytesWrapper },
        FieldType::Integer => quote! { i64 },
        FieldType::Float => quote! { f64 },
        FieldType::Version => quote! { u32 },
        FieldType::Size => quote! { u64},
        FieldType::Index => quote! { u64 },
        FieldType::NonZeroInteger => quote! { u64 },
        FieldType::IntegerBetween1And100 => quote! { u64 },
        FieldType::PublicKey => quote! { crate::crypto::PublicKey },
        FieldType::SigningKey => quote! { crate::crypto::SigningKey },
        FieldType::VerifyKey => quote! { crate::crypto::VerifyKey },
        FieldType::PrivateKey => quote! { crate::crypto::PrivateKey },
        FieldType::SecretKey => quote! { crate::crypto::SecretKey },
        FieldType::HashDigest => quote! { crate::crypto::HashDigest },
        FieldType::SequesterVerifyKeyDer => quote! { crate::crypto::SequesterVerifyKeyDer },
        FieldType::SequesterPublicKeyDer => quote! { crate::crypto::SequesterPublicKeyDer },
        FieldType::DateTime => quote! { crate::time::DateTime },
        FieldType::BlockID => quote! { crate::ids::BlockID },
        FieldType::DeviceID => quote! { crate::ids::DeviceID },
        FieldType::OrganizationID => quote! { crate::ids::OrganizationID },
        FieldType::UserID => quote! { crate::ids::UserID },
        FieldType::VlobID => quote! { crate::ids::VlobID },
        FieldType::EnrollmentID => quote! { crate::ids::EnrollmentID },
        FieldType::SequesterServiceID => quote! { crate::ids::SequesterServiceID },
        FieldType::DeviceLabel => quote! { crate::ids::DeviceLabel },
        FieldType::HumanHandle => quote! { crate::ids::HumanHandle },
        FieldType::UserProfile => quote! { crate::enumerate::UserProfile },
        FieldType::RealmRole => quote! { crate::enumerate::RealmRole },
        FieldType::BootstrapToken => quote! { crate::ids::BootstrapToken },
        FieldType::InvitationToken => quote! { crate::ids::InvitationToken },
        FieldType::InvitationStatus => quote! { crate::enumerate::InvitationStatus },
        FieldType::CertificateSignerOwned => quote! { crate::certif::CertificateSignerOwned },
        FieldType::BlockAccess => quote! { crate::data::BlockAccess },
        FieldType::EntryName => quote! { crate::data::EntryName },
        FieldType::WorkspaceEntry => quote! { crate::data::WorkspaceEntry },
        FieldType::FileManifest => quote! { crate::data::FileManifest },
        FieldType::FolderManifest => quote! { crate::data::FolderManifest },
        FieldType::WorkspaceManifest => quote! { crate::data::WorkspaceManifest },
        FieldType::UserManifest => quote! { crate::data::UserManifest },
        FieldType::ActiveUsersLimit => quote! { crate::protocol::ActiveUsersLimit },
        FieldType::Chunk => quote! { crate::data::Chunk },
        FieldType::BackendOrganizationAddr => quote! { crate::addrs::BackendOrganizationAddr },
        FieldType::UsersPerProfileDetailItem => quote! { crate::data::UsersPerProfileDetailItem },
        FieldType::BackendPkiEnrollmentAddr => quote! { crate::addrs::BackendPkiEnrollmentAddr },
        FieldType::PkiEnrollmentSubmitPayload => quote! { crate::data::PkiEnrollmentSubmitPayload },
        FieldType::X509Certificate => quote! { crate::data::X509Certificate },
    }
}

fn quote_field_as_fn_new_conversion(field: &GenCmdField) -> TokenStream {
    let field_name = format_ident!("{}", field.name);
    let conversion = internal_quote_field_as_fn_new_conversion(&field_name, &field.ty);
    if field.added_in_minor_revision {
        quote! { let #field_name = libparsec_types::Maybe::Present(#conversion); }
    } else if conversion.to_string() == field.name {
        // No conversion
        quote! {}
    } else {
        quote! { let #field_name = #conversion; }
    }
}

fn internal_quote_field_as_fn_new_conversion(field_name: &Ident, ty: &FieldType) -> TokenStream {
    match ty {
        FieldType::Map(key, value) => {
            let key_name = format_ident!("k");
            let key_conversion = internal_quote_field_as_fn_new_conversion(&key_name, key);
            let value_name = format_ident!("v");
            let value_conversion = internal_quote_field_as_fn_new_conversion(&value_name, value);
            quote! {
                #field_name.into_iter().map(|(k, v)| {
                    let k = #key_conversion;
                    let v = #value_conversion;
                    (k, v)
                }).collect::<std::collections::HashMap<_, _>>()
            }
        }
        FieldType::List(nested) => {
            let item_name = format_ident!("x");
            let item_conversion = internal_quote_field_as_fn_new_conversion(&item_name, nested);
            quote! {
                #field_name.into_iter().map(|x| #item_conversion).collect::<Vec<_>>()
            }
        }
        FieldType::Set(nested) => {
            let item_name = format_ident!("x");
            let item_conversion = internal_quote_field_as_fn_new_conversion(&item_name, nested);
            quote! {
                #field_name.into_iter().map(|x| #item_conversion).collect::<std::collections::HashSet<_>>()
            }
        }
        FieldType::NonRequiredOption(nested) => {
            let nested_name = format_ident!("x");
            let nested_conversion = internal_quote_field_as_fn_new_conversion(&nested_name, nested);
            quote! { #field_name.map(|x| #nested_conversion) }
        }
        FieldType::RequiredOption(nested) => {
            let nested_name = format_ident!("x");
            let nested_conversion = internal_quote_field_as_fn_new_conversion(&nested_name, nested);
            quote! {
                #field_name.map(|x| #nested_conversion)
            }
        }
        FieldType::Tuple(items) => {
            let (xs, xs_conversions): (Vec<_>, Vec<_>) = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let x = format_ident!("x{}", i);
                    let x_conversion = internal_quote_field_as_fn_new_conversion(&x, item);
                    (x, x_conversion)
                })
                .unzip();
            quote! {
                {
                    let (#(#xs),*) = #field_name;
                    (#(#xs_conversions),*)
                }
            }
        }
        FieldType::IntegerBetween1And100 => {
            quote! { libparsec_types::IntegerBetween1And100::try_from(#field_name).map_err(PyValueError::new_err)? }
        }
        FieldType::NonZeroInteger => {
            quote! { ::std::num::NonZeroU64::try_from(#field_name).map_err(PyValueError::new_err)? }
        }
        FieldType::Bytes => quote! {
            {
                use crate::binding_utils::UnwrapBytesWrapper;
                #field_name.unwrap_bytes()
            }
        },
        FieldType::Custom(_)
        | FieldType::PublicKey
        | FieldType::SigningKey
        | FieldType::VerifyKey
        | FieldType::PrivateKey
        | FieldType::SecretKey
        | FieldType::HashDigest
        | FieldType::SequesterVerifyKeyDer
        | FieldType::SequesterPublicKeyDer
        | FieldType::DateTime
        | FieldType::BlockID
        | FieldType::DeviceID
        | FieldType::OrganizationID
        | FieldType::UserID
        | FieldType::VlobID
        | FieldType::EnrollmentID
        | FieldType::SequesterServiceID
        | FieldType::DeviceLabel
        | FieldType::HumanHandle
        | FieldType::UserProfile
        | FieldType::RealmRole
        | FieldType::BootstrapToken
        | FieldType::InvitationToken
        | FieldType::InvitationStatus
        | FieldType::CertificateSignerOwned
        | FieldType::BlockAccess
        | FieldType::EntryName
        | FieldType::WorkspaceEntry
        | FieldType::FileManifest
        | FieldType::FolderManifest
        | FieldType::WorkspaceManifest
        | FieldType::UserManifest
        | FieldType::ActiveUsersLimit
        | FieldType::Chunk
        | FieldType::BackendOrganizationAddr
        | FieldType::UsersPerProfileDetailItem
        | FieldType::BackendPkiEnrollmentAddr
        | FieldType::PkiEnrollmentSubmitPayload
        | FieldType::X509Certificate => quote! { #field_name.0 },
        // No conversion for the rest
        _ => quote! { #field_name },
    }
}
