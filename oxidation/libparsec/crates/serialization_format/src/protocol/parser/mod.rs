pub mod collection;
pub mod custom_type;
pub mod field;
pub mod major_minor_version;

use std::collections::HashMap;

pub use collection::ProtocolCollection;
pub use custom_type::{CustomEnum, CustomStruct, CustomType, Variant};
pub use field::Field;
pub use major_minor_version::MajorMinorVersion;

use serde::Deserialize;

use super::utils::{quote_fields, to_pascal_case};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Deserialize)]
pub struct Protocol {
    pub variants: Vec<Cmd>,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Cmd {
    pub label: String,
    pub major_versions: Vec<u32>,
    pub req: Request,
    pub possible_responses: Vec<Response>,
    // TODO: May need to be put in a option.
    pub nested_types: Vec<CustomType>,
}

#[cfg(test)]
impl Default for Cmd {
    fn default() -> Self {
        Self {
            label: "FooCmd".to_string(),
            major_versions: vec![],
            req: Request::default(),
            nested_types: vec![],
            possible_responses: vec![],
        }
    }
}

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
        let fields = quote_fields(&self.other_fields, types);

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
