// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Pair, AttributeArgs, FnArg, ItemFn, Meta, NestedMeta, Pat, Type,
    TypeReference,
};

#[proc_macro_attribute]
pub fn parsec_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let parsec_test_item = parse_macro_input!(item as ItemFn);
    let mut sig = parsec_test_item.sig;
    let block = parsec_test_item.block;

    let mut testbed = None;
    let mut with_server = false;

    for arg in args {
        match arg {
            NestedMeta::Meta(meta) => match meta {
                Meta::NameValue(v) => {
                    match v
                        .path
                        .get_ident()
                        .expect("Ident expected")
                        .to_string()
                        .as_str()
                    {
                        // Add your new NameValue there
                        "testbed" => testbed = Some(v.lit),
                        _ => panic!("unimplemented for {v:?}"),
                    }
                }
                Meta::Path(p) => {
                    match p
                        .segments
                        .last()
                        .expect("Incomplete path")
                        .ident
                        .to_string()
                        .as_str()
                    {
                        // Add your new Path there
                        "with_server" => with_server = true,
                        _ => panic!("unimplemented for {p:?}"),
                    }
                }
                _ => panic!("unimplemented for {meta:?}"),
            },
            _ => panic!("unimplemented for {arg:?}"),
        }
    }

    let mut quote_attrs = quote! {};

    for attr in parsec_test_item.attrs {
        quote_attrs = quote! {
            #quote_attrs
            #attr
        }
    }

    let quote_block = if let Some(testbed) = testbed {
        match sig.inputs.pop().map(Pair::into_value) {
            Some(FnArg::Typed(typed)) => {
                match typed.pat.as_ref() {
                    Pat::Ident(pat) if pat.ident == "env" => (),
                    _ => panic!("The last argument must be: `env: &TestbedEnv`"),
                }

                match typed.ty.as_ref() {
                    Type::Reference(TypeReference { elem: e, .. }) => match e.as_ref() {
                        Type::Path(p)
                            if p.path.segments.last().expect("Incomplete path").ident
                                == "TestbedEnv" => {}
                        _ => panic!("The last argument must be: `env: &TestbedEnv`"),
                    },
                    _ => panic!("The last argument must be: `env: &TestbedEnv`"),
                }
            }
            _ => panic!("Missing argument: `env: &TestbedEnv`"),
        }
        if with_server {
            quote! {
                libparsec_tests_fixtures::TestbedScope::run_with_server(#testbed, |env: std::sync::Arc<TestbedEnv>| async move {
                        let env = env.as_ref();
                        #block
                    }
                ).await;
            }
        } else {
            quote! {
                libparsec_tests_fixtures::TestbedScope::run(#testbed, |env: std::sync::Arc<TestbedEnv>| async move {
                        let env = env.as_ref();
                        #block
                    }
                ).await;
            }
        }
    } else {
        quote! {
            #block
        }
    };

    let quote_async_test = if sig.asyncness.is_some() {
        quote! {
            #[tokio::test]
        }
    } else {
        quote! {}
    };

    TokenStream::from(quote! {
        #[libparsec_tests_fixtures::rstest::rstest]
        #quote_attrs
        #quote_async_test
        #sig {
            let _ = libparsec_tests_fixtures::env_logger::builder().is_test(true).try_init();
            #quote_block
        }
    })
}
