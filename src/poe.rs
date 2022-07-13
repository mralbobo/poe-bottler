use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::{json_utils::{grab_nested_map, grab_nested_val_optional}, conversion_schema::ConversionSchema};


#[derive(Debug, Serialize, Deserialize)]
pub struct PoeConfig {
    term: String,
    definition: PoeDefinition,
    context: Option<String>,
    term_plural: Option<String>,
    reference: Option<String>,
    comment: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PoeDefinition {
    String(String),
    Map(HashMap<String, String>)
}

impl PoeConfig {
    pub fn from_value(val: &serde_json::Value, schema: &ConversionSchema, index: usize) -> PoeConfig {
        //in initial cut:
        // - value is an item that contains the entire contents of a PoeConfig

        //special case
        let term = PoeConfig::derive_term(val, schema, index).unwrap();
        // println!("def, {}", term);
        // let definition = grab_nested_val(val, &schema.definition);
        // println!("def: {:?}", definition);

        let definition = grab_nested_map(val, &schema.definition);

        let context = grab_nested_val_optional(val, schema.context.as_deref());

        let term_plural = grab_nested_val_optional(val, schema.term_plural.as_deref());
        let reference = grab_nested_val_optional(val, schema.reference.as_deref());
        let comment = grab_nested_val_optional(val, schema.commment.as_deref());

        PoeConfig { term, definition, context, term_plural, reference, comment }
    }

    fn derive_term(val: &serde_json::Value, schema: &ConversionSchema, index: usize) -> Option<String> {
        // let should_derive_term = schema.deriveTerm.unwrap_or(false);
        let should_derive_term = schema.derive_term;

        if should_derive_term {
            Some(format!("{}.{}", schema.base_path, index))
        }
        else {
            grab_nested_val_optional(val, schema.term.as_deref())
        }
    }
}