// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, punctuated::Pair, spanned::Spanned, FnArg, ItemFn, LitInt, LitStr, Pat,
    Type, TypeReference,
};

#[proc_macro_attribute]
pub fn parsec_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsec_test_item = parse_macro_input!(item as ItemFn);
    let mut attributes = Attributes::default();
    let parsec_test_parser = syn::meta::parser(|meta| attributes.parse(meta));
    parse_macro_input!(attr with parsec_test_parser);

    let mut sig = parsec_test_item.sig;
    let block = parsec_test_item.block;

    let quote_block = if let Some(testbed) = attributes.testbed {
        // If `testbed` is found, `env: &TestbedEnv` must be set as the last argument.
        if let Err(value) = validate_testbed_argument(&mut sig) {
            return value;
        }

        if attributes.with_server {
            quote! {
                ::libparsec_tests_fixtures::TestbedScope::run_with_server(#testbed, |env: std::sync::Arc<TestbedEnv>| async move {
                        let env = env.as_ref();
                        #block
                    }
                ).await;
            }
        } else {
            quote! {
                ::libparsec_tests_fixtures::TestbedScope::run(#testbed, |env: std::sync::Arc<TestbedEnv>| async move {
                        let env = env.as_ref();
                        #block
                    }
                ).await;
            }
        }
    } else {
        quote! { #block }
    };

    // By default Tokio uses a single threaded runtime, which is fine for most of our
    // tests (as on web our application will run in a single threaded environment anyway !).
    //
    // However multi-threaded might still be useful when testing the mountpoint (typically
    // to debug deadlocks in the tests due to unexpected blocking code in `tokio::fs` API).
    let tokio_flavor = match attributes.tokio_flavor {
        Some(tokio_flavor) => quote!(, flavor = #tokio_flavor),
        None => quote!(),
    };
    let tokio_worker_threads = match attributes.tokio_worker_threads {
        Some(tokio_worker_threads) => quote!(, worker_threads = #tokio_worker_threads),
        None => quote!(),
    };
    let tokio_decorator = sig
        .asyncness
        .map(|_| quote! {
            #[cfg_attr(not(target_arch = "wasm32"), ::libparsec_tests_lite::tokio::test(crate = "::libparsec_tests_lite::tokio" #tokio_flavor #tokio_worker_threads))]
        })
        .into_iter();
    let attrs = parsec_test_item.attrs;

    TokenStream::from(quote! {
        #[::libparsec_tests_lite::rstest::rstest]
        #(#attrs)*
        #(#tokio_decorator)*
        // FIXME: Workaround for rstest until https://github.com/la10736/rstest/issues/211 is resolved
        #[cfg_attr(target_arch = "wasm32", ::libparsec_tests_lite::wasm::test)]
        #sig {
            let _ = ::libparsec_tests_lite::env_logger::builder().is_test(true).try_init();
            #quote_block
        }
    })
}

fn validate_testbed_argument(sig: &mut syn::Signature) -> Result<(), TokenStream> {
    match sig.inputs.pop().map(Pair::into_value) {
        Some(FnArg::Typed(typed)) => {
            validate_testbed_argument_name(&typed)?;
            validate_testbed_argument_type(&typed)?;
        }
        _ => {
            return Err(generate_compile_error(
                sig.inputs.span(),
                "Missing argument: `env: &TestbedEnv`",
            ))
        }
    }
    Ok(())
}

fn validate_testbed_argument_type(typed: &syn::PatType) -> Result<(), TokenStream> {
    match typed.ty.as_ref() {
        Type::Reference(TypeReference { elem: e, .. }) => match e.as_ref() {
            Type::Path(p)
                if p.path.segments.last().expect("Incomplete path").ident == "TestbedEnv" => {}
            _ => {
                return Err(generate_compile_error(
                    e.span(),
                    "The last argument reference type must be `TestbedEnv`",
                ))
            }
        },
        _ => {
            return Err(generate_compile_error(
                typed.ty.span(),
                "The last argument type must be `&TestbedEnv`",
            ))
        }
    }
    Ok(())
}

fn validate_testbed_argument_name(typed: &syn::PatType) -> Result<(), TokenStream> {
    match typed.pat.as_ref() {
        Pat::Ident(pat) if pat.ident == "env" => (),
        _ => {
            return Err(generate_compile_error(
                typed.pat.span(),
                "The last argument must be called `env`",
            ))
        }
    }
    Ok(())
}

#[derive(Default)]
struct Attributes {
    testbed: Option<LitStr>,
    with_server: bool,
    tokio_flavor: Option<LitStr>,
    tokio_worker_threads: Option<LitInt>,
}

impl Attributes {
    fn parse(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("testbed") {
            self.testbed = Some(meta.value()?.parse::<LitStr>()?);
            Ok(())
        } else if meta.path.is_ident("with_server") {
            self.with_server = true;
            Ok(())
        } else if meta.path.is_ident("tokio_flavor") {
            self.tokio_flavor = Some(meta.value()?.parse::<LitStr>()?);
            Ok(())
        } else if meta.path.is_ident("tokio_worker_threads") {
            self.tokio_worker_threads = Some(meta.value()?.parse::<LitInt>()?);
            Ok(())
        } else {
            Err(meta.error("unsupported parsec_test property"))
        }
    }
}

fn generate_compile_error(span: Span, msg: &str) -> proc_macro::TokenStream {
    quote_spanned! { span => compile_error!(#msg); }.into()
}
