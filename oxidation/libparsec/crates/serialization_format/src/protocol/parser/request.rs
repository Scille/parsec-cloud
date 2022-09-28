use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Field};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub cmd: String,
    pub unit: Option<String>,
    pub other_fields: Vec<Field>,
}

#[cfg(test)]
impl Default for Request {
    fn default() -> Self {
        Self {
            cmd: "foo_cmd".to_string(),
            unit: None,
            other_fields: vec![],
        }
    }
}

impl Request {
    pub fn quote_name(&self) -> syn::Ident {
        syn::parse_str(&self.cmd).expect("A valid request cmd name")
    }

    pub fn quote(&self, types: &HashMap<String, String>) -> syn::ItemStruct {
        if let Some(unit) = &self.unit {
            self.quote_unit(unit)
        } else if self.other_fields.is_empty() {
            self.quote_empty()
        } else {
            self.quote_fields(types)
        }
    }

    fn quote_unit(&self, unit: &str) -> syn::ItemStruct {
        let unit = syn::parse_str::<syn::Ident>(unit).expect("A valid unit name");
        let shared_attr = Request::shared_derive();

        syn::parse_quote! {
            #shared_attr
            pub struct Req(pub #unit)

            impl Req {
                pub fn new(value: #unit) -> Self {
                    Self(Value)
                }
            }
        }
    }

    fn quote_empty(&self) -> syn::ItemStruct {
        let shared_attr = Request::shared_derive();

        syn::parse_quote! {
            #shared_attr
            pub struct Req;

            impl Req {
                pub fn new() -> Self { Self }
            }
        }
    }

    fn quote_fields(&self, types: &HashMap<String, String>) -> syn::ItemStruct {
        let shared_derive = Request::shared_derive();
        let fields = quote_fields(&self.other_fields, None, types);

        syn::parse_quote! {
            #[::serde_with::serde_as]
            #shared_derive
            pub struct Req {
                #(#fields)*
            }

            impl Req {
                pub fn new(#(#fields),*) -> Self {
                    Self {
                        #(#fields.name),*
                    }
                }
            }
        }
    }

    fn shared_derive() -> syn::Attribute {
        syn::parse_quote!(#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)])
    }
}
