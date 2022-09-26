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

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub cmd: String,
    pub unit: Option<String>,
    pub other_fields: Vec<Field>,
}

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    pub status: String,
    #[serde(default)]
    pub unit: Option<String>,
    pub other_fields: Vec<Field>,
}
