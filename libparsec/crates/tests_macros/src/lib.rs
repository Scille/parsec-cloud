// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Pair, FnArg, ItemFn, LitStr, Pat, Type, TypeReference};

#[proc_macro_attribute]
pub fn parsec_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsec_test_item = parse_macro_input!(item as ItemFn);
    let mut sig = parsec_test_item.sig;
    let block = parsec_test_item.block;

    let mut attributes = Attributes::default();

    let parsec_test_parser = syn::meta::parser(|meta| attributes.parse(meta));

    let mut quote_block = quote! {
        #block
    };

    if !attr.is_empty() {
        parse_macro_input!(attr with parsec_test_parser);

        if let Some(testbed) = attributes.testbed {
            // If `testbed` is found, `env: &TestbedEnv` must be set in args
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

            if attributes.with_server {
                quote_block = quote! {
                    ::libparsec_tests_fixtures::TestbedScope::run_with_server(#testbed, |env: std::sync::Arc<TestbedEnv>| async move {
                            let env = env.as_ref();
                            #block
                        }
                    ).await;
                }
            } else {
                quote_block = quote! {
                    ::libparsec_tests_fixtures::TestbedScope::run(#testbed, |env: std::sync::Arc<TestbedEnv>| async move {
                            let env = env.as_ref();
                            #block
                        }
                    ).await;
                }
            }
        }
    }

    let mut quote_attrs = quote! {};

    for attr in parsec_test_item.attrs {
        quote_attrs = quote! {
            #quote_attrs
            #attr
        }
    }

    let quote_block = if sig.asyncness.is_some() {
        sig.asyncness = None;
        quote! {
            ::libparsec_tests_fixtures::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    #quote_block
                })
        }
    } else {
        quote_block
    };

    TokenStream::from(quote! {
        #[::libparsec_tests_fixtures::rstest::rstest]
        #quote_attrs
        #sig {
            let _ = ::libparsec_tests_fixtures::env_logger::builder().is_test(true).try_init();
            #quote_block
        }
    })
}

#[derive(Default)]
struct Attributes {
    testbed: Option<LitStr>,
    with_server: bool,
}

impl Attributes {
    fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("testbed") {
            self.testbed = Some(meta.value()?.parse::<LitStr>()?);
            Ok(())
        } else if meta.path.is_ident("with_server") {
            self.with_server = true;
            Ok(())
        } else {
            Err(meta.error("unsupported parsec_test property"))
        }
    }
}
