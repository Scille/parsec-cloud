// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{braced, LitStr, Token};

pub(crate) struct Schema(pub(crate) Vec<SchemaField>);

impl Parse for Schema {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _: Brace = braced!(content in input);
        let fields: Punctuated<SchemaField, Token![,]> =
            content.parse_terminated(SchemaField::parse)?;
        Ok(Self(fields.into_iter().collect()))
    }
}

pub(crate) struct SchemaField {
    pub(crate) name: String,
    pub(crate) fields: Vec<SchemaField>,
}

impl Parse for SchemaField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: LitStr = input.parse()?;
        let _: Token![:] = input.parse()?;
        let token = input.lookahead1();
        Ok(if token.peek(Brace) {
            let content;
            let _: Brace = braced!(content in input);
            let fields: Punctuated<SchemaField, Token![,]> =
                content.parse_terminated(SchemaField::parse)?;
            Self {
                name: name.value(),
                fields: fields.into_iter().collect(),
            }
        } else {
            let field: LitStr = input.parse()?;
            Self {
                name: name.value(),
                fields: vec![SchemaField {
                    name: field.value(),
                    fields: vec![],
                }],
            }
        })
    }
}
