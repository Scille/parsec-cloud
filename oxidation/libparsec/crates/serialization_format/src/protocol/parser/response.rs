use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Field};
use crate::protocol::utils::to_pascal_case;

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    pub status: String,
    #[serde(default)]
    pub unit: Option<String>,
    pub other_fields: Vec<Field>,
}

#[cfg(test)]
impl Default for Response {
    fn default() -> Self {
        Self {
            status: "foo_response".to_string(),
            unit: None,
            other_fields: vec![],
        }
    }
}

impl Response {
    pub fn quote(&self, types: &HashMap<String, String>) -> syn::Variant {
        if let Some(unit) = &self.unit {
            self.quote_unit(unit)
        } else if self.other_fields.is_empty() {
            self.quote_empty()
        } else {
            self.quote_fields(types)
        }
    }

    fn quote_unit(&self, unit: &str) -> syn::Variant {
        let name = self.quote_name();
        let rename = &self.status;
        let unit = syn::parse_str::<syn::Type>(unit).expect("A valid unit");

        syn::parse_quote! {
            #[serde(rename = #rename)]
            #name(#unit)
        }
    }

    fn quote_empty(&self) -> syn::Variant {
        let name = self.quote_name();
        let rename = &self.status;

        syn::parse_quote! {
            #[serde(rename = #rename)]
            #name
        }
    }

    fn quote_fields(&self, types: &HashMap<String, String>) -> syn::Variant {
        let name = self.quote_name();
        let rename = &self.status;
        let fields = quote_fields(&self.other_fields, types);

        syn::parse_quote! {
            #[serde(rename = #rename)]
            #name {
                #(#fields),*
            }
        }
    }

    fn quote_name(&self) -> syn::Ident {
        syn::parse_str(&to_pascal_case(&self.status)).expect("A valid status")
    }
}
