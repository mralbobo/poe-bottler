use serde::Deserialize;

//blarg - is deserialize necessary?
#[derive(Deserialize, Default)]
pub struct ConversionSchema {
    pub base_path: String,
    pub definition: String,
    pub context: Option<String>,
    pub term_plural: Option<String>,
    pub reference: Option<String>,
    pub commment: Option<String>
}