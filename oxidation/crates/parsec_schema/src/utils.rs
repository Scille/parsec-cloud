// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Iter;
use syn::{Attribute, Field, Fields, GenericArgument, PathArguments, Type, Variant};

fn extract_ty(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            let mut ident = p.path.segments[0].ident.to_string();
            match &p.path.segments[0].arguments {
                PathArguments::None => ident,
                PathArguments::AngleBracketed(x) => {
                    ident.push('<');
                    ident += &x
                        .args
                        .iter()
                        .map(|arg| match arg {
                            GenericArgument::Type(ty) => extract_ty(ty),
                            _ => unimplemented!(),
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    ident.push('>');
                    ident
                }
                PathArguments::Parenthesized(x) => {
                    ident.push('(');
                    ident += &x
                        .inputs
                        .iter()
                        .map(extract_ty)
                        .collect::<Vec<_>>()
                        .join(",");
                    ident.push(')');
                    ident
                }
            }
        }
        ty => panic!("{ty:?} encountered"),
    }
}

pub fn quote_fields(fields: Iter<Field>) -> TokenStream {
    let mut _fields = quote! {};

    for field in fields {
        let ty = extract_ty(&field.ty);
        if ty.starts_with("Vec<u8>") {
            _fields = quote! {
                #_fields
                #[serde_as(as = "::serde_with::Bytes")]
                #field,
            }
        } else if ty.starts_with("Option") {
            _fields = quote! {
                #_fields
                #[serde(default, deserialize_with = "::parsec_api_types::maybe_field::deserialize_some")]
                #field,
            }
        } else {
            _fields = quote! {
                #_fields
                #field,
            }
        }
    }

    _fields
}

pub fn quote_variants(variants: Iter<Variant>) -> TokenStream {
    let mut _variants = quote! {};

    for variant in variants {
        let ident = &variant.ident;
        let fields = &variant.fields;
        let attrs = quote_attrs(&variant.attrs);

        match fields {
            Fields::Unit => {
                _variants = quote! {
                    #_variants
                    #attrs
                    #ident,
                }
            }
            Fields::Unnamed(_) => {
                let fields = quote_fields(variant.fields.iter());
                _variants = quote! {
                    #_variants
                    #attrs
                    #ident (#fields),
                }
            }
            Fields::Named(_) => {
                let fields = quote_fields(variant.fields.iter());
                _variants = quote! {
                    #_variants
                    #attrs
                    #ident {#fields},
                }
            }
        }
    }

    _variants
}

pub fn quote_attrs(attrs: &[Attribute]) -> TokenStream {
    let mut _attrs = quote! {};

    for attr in attrs {
        _attrs = quote! {
            #_attrs
            #attr
        };
    }

    _attrs
}
