pub mod collection;
pub mod custom_type;
pub mod field;
pub mod major_minor_version;

pub use collection::ProtocolCollection;
pub use custom_type::{CustomEnum, CustomStruct, CustomType, Variant};
pub use field::Field;
pub use major_minor_version::MajorMinorVersion;

use serde::Deserialize;

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
