use serde::Deserialize;

//blarg - is deserialize necessary?
#[derive(Deserialize, Default)]
pub struct ConversionSchema {
    pub derive_term: bool,
    pub base_path: String,
    pub term: Option<String>,
    pub definition: String,
    pub context: Option<String>,
    pub term_plural: Option<String>,
    pub reference: Option<String>,
    pub commment: Option<String>
}