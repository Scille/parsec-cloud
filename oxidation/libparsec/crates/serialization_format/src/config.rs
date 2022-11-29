// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct MacroConfig {
    pub raw_path: String,
    /// Path or crate name pointing to `libparsec_types` types.
    pub crates_override: CratesPaths,
}

impl syn::parse::Parse for MacroConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            let raw_path = input.parse::<syn::LitStr>()?.value();
            Ok(Self {
                raw_path,
                crates_override: CratesPaths::default(),
            })
        } else {
            let raw_stmts = input.parse::<syn::Block>()?.stmts;
            let raw_exprs = parse_assign_exprs(raw_stmts);
            let mut raw_path: Option<String> = None;
            let mut crates_paths: Option<CratesPaths> = None;

            for (var, right_expr) in raw_exprs {
                match var.as_str() {
                    "path" => {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = *right_expr
                        {
                            raw_path = Some(lit_str.value());
                        } else {
                            panic!("Unexpected right operand path type `{right_expr:?}`")
                        }
                    }
                    "crates" => {
                        if let syn::Expr::Block(syn::ExprBlock { block, .. }) = *right_expr {
                            crates_paths = Some(CratesPaths::parse_from_block(block)?);
                        } else {
                            panic!("Unexpected right operand crates type `{right_expr:?}`")
                        }
                    }
                    _ => panic!("Unexpected variable name `{var}`"),
                }
            }
            Ok(Self {
                raw_path: raw_path.expect("Missing path"),
                crates_override: crates_paths.unwrap_or_default(),
            })
        }
    }
}

fn parse_assign_exprs(raw_stmts: Vec<syn::Stmt>) -> impl Iterator<Item = (String, Box<syn::Expr>)> {
    raw_stmts
        .into_iter()
        .map(|stmts| match stmts {
            syn::Stmt::Semi(syn::Expr::Assign(syn::ExprAssign { left, right, .. }), _)
            | syn::Stmt::Expr(syn::Expr::Assign(syn::ExprAssign { left, right, .. })) => {
                (left, right)
            }
            _ => panic!("Wanted assign statement but got `{:?}`", stmts),
        })
        .map(|(left, right)| {
            if let syn::Expr::Path(syn::ExprPath {
                path: syn::Path { segments, .. },
                ..
            }) = *left
            {
                let var_name = segments
                    .first()
                    .expect("Unexpected empty path")
                    .ident
                    .to_string();
                (var_name, right)
            } else {
                panic!("Unexpected left operand type `{left:?}`")
            }
        })
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct CratesPaths(HashMap<String, String>);

#[cfg(test)]
impl syn::parse::Parse for CratesPaths {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::parse_from_block(input.parse()?)
    }
}

impl CratesPaths {
    fn parse_from_block(input: syn::Block) -> syn::Result<Self> {
        let mut paths = Self::default();

        let crates_exprs = parse_assign_exprs(input.stmts);
        for (crate_name, raw_override) in crates_exprs {
            let crate_override = if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            }) = *raw_override
            {
                lit_str.value()
            } else {
                panic!("Unexpected right operand override type `{raw_override:?}`");
            };
            paths.0.insert(crate_name, crate_override);
        }

        Ok(paths)
    }
}

impl Default for CratesPaths {
    fn default() -> Self {
        macro_rules! crate_path {
            ($name:literal) => {
                ($name.to_string(), $name.to_string())
            };
        }

        Self(HashMap::from_iter([
            crate_path!("libparsec_crypto"),
            crate_path!("libparsec_types"),
            crate_path!("libparsec_client_types"),
            crate_path!("libparsec_protocol"),
        ]))
    }
}

impl std::ops::Index<&str> for CratesPaths {
    type Output = String;

    fn index<'a>(&'a self, index: &str) -> &'a Self::Output {
        self.0.index(index)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use proc_macro2::TokenStream;
    use quote::quote;
    use rstest::rstest;
    use syn::parse::Parse;

    use super::{CratesPaths, MacroConfig};

    #[rstest]
    #[case::empty_block(quote!({}), CratesPaths::default())]
    #[case::simple(quote!({
        libparsec_types = "foobar"
    }), CratesPaths(HashMap::from_iter([
        ("libparsec_crypto".to_string(), "libparsec_crypto".to_string()),
        ("libparsec_types".to_string(), "foobar".to_string()),
        ("libparsec_client_types".to_string(), "libparsec_client_types".to_string()),
        ("libparsec_protocol".to_string(), "libparsec_protocol".to_string()),
    ])))]
    #[case::other(quote!({
        foobar = "foobar"
    }), CratesPaths(HashMap::from_iter([
        ("libparsec_crypto".to_string(), "libparsec_crypto".to_string()),
        ("libparsec_types".to_string(), "libparsec_types".to_string()),
        ("foobar".to_string(), "foobar".to_string()),
        ("libparsec_client_types".to_string(), "libparsec_client_types".to_string()),
        ("libparsec_protocol".to_string(), "libparsec_protocol".to_string()),
    ])))]
    fn test_crates_path(#[case] token_stream: TokenStream, #[case] expected_paths: CratesPaths) {
        // let token_stream = token_stream.into();
        let paths = syn::parse::Parser::parse2(CratesPaths::parse, token_stream);
        assert!(
            paths.is_ok(),
            "Parse result in error: {}",
            paths.unwrap_err()
        );
        assert_eq!(paths.unwrap(), expected_paths);
    }

    #[rstest]
    #[case::only_path(quote!("foo/bar"), MacroConfig {
        raw_path: "foo/bar".to_string(),
        crates_override: CratesPaths::default(),
    })]
    #[case::block_path_only(
        quote!({
            path = "foo/bar"
        }),
        MacroConfig { raw_path: "foo/bar".to_string(), crates_override: CratesPaths::default() },
    )]
    #[case::block_full(
        quote!({
            path = "foo/bar";
            crates = {
                libparsec_types = "foobar"
            }
        }),
        MacroConfig { raw_path: "foo/bar".to_string(), crates_override:  CratesPaths(HashMap::from_iter([
            ("libparsec_crypto".to_string(), "libparsec_crypto".to_string()),
            ("libparsec_types".to_string(), "foobar".to_string()),
            ("libparsec_client_types".to_string(), "libparsec_client_types".to_string()),
            ("libparsec_protocol".to_string(), "libparsec_protocol".to_string()),
        ])) }
    )]
    fn test_config(#[case] token_stream: TokenStream, #[case] expected_config: MacroConfig) {
        let config_res = syn::parse::Parser::parse2(MacroConfig::parse, token_stream);
        assert!(
            config_res.is_ok(),
            "Parse result in error: {}",
            config_res.unwrap_err()
        );
        assert_eq!(config_res.unwrap(), expected_config);
    }
}
