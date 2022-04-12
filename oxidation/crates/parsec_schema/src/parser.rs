// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use syn::parse::{Parse, ParseStream};
use syn::{Attribute, ItemEnum, ItemStruct, Token};

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
