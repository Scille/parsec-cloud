// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parenthesized, Attribute, Ident, ItemEnum, ItemStruct, LitStr, Token};

pub(crate) enum Schema {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

impl Parse for Schema {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Attribute::parse_outer(input)?;
        Ok(if input.peek(Token![pub]) && input.peek2(Token![struct]) {
            let mut schema = input.parse::<ItemStruct>()?;
            schema.attrs.append(&mut attrs);
            Schema::Struct(schema)
        } else if input.peek(Token![pub]) && input.peek2(Token![enum]) {
            let mut schema = input.parse::<ItemEnum>()?;
            schema.attrs.append(&mut attrs);
            Schema::Enum(schema)
        } else {
            unimplemented!()
        })
    }
}

pub(crate) struct SerdeAttrField {
    pub(crate) ident: Ident,
    _eq_token: Token![=],
    pub(crate) literal: LitStr,
}

impl Parse for SerdeAttrField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            _eq_token: input.parse()?,
            literal: input.parse()?,
        })
    }
}

pub(crate) struct SerdeAttr {
    _paren_token: Paren,
    pub(crate) fields: Punctuated<SerdeAttrField, Token![,]>,
}

impl Parse for SerdeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _paren_token = parenthesized!(content in input);
        Ok(Self {
            _paren_token,
            fields: content.parse_terminated(SerdeAttrField::parse)?,
        })
    }
}
