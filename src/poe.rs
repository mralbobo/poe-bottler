use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::{json_utils::{grab_nested_map, grab_nested_val_optional, grab_nested_value_mut}, conversion_schema::ConversionSchema};


#[derive(Debug, Serialize, Deserialize)]
pub struct PoeConfig {
    pub term: String,
    pub definition: PoeDefinition,
    pub context: Option<String>,
    pub term_plural: Option<String>,
    pub reference: Option<String>,
    pub comment: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PoeDefinition {
    String(String),
    Map(HashMap<String, String>)
}

impl PoeConfig {
    pub fn from_value(val: &serde_json::Value, schema: &ConversionSchema, index: usize) -> Result<PoeConfig, serde_json::Error> {
        //in initial cut:
        // - value is an item that contains the entire contents of a PoeConfig

        //special case
        let term = PoeConfig::derive_term(val, schema, index).unwrap();
        // println!("def, {}", term);
        // let definition = grab_nested_val(val, &schema.definition);
        // println!("def: {:?}", definition);

        // let definition = grab_nested_map(val, &schema.definition);
        let definition =  match grab_nested_map(val, &schema.definition) {
            Ok(def) => def,
            Err(e) => return Err(e),
        };

        let context = grab_nested_val_optional(val, schema.context.as_deref());

        let term_plural = grab_nested_val_optional(val, schema.term_plural.as_deref());
        let reference = grab_nested_val_optional(val, schema.reference.as_deref());
        let comment = grab_nested_val_optional(val, schema.commment.as_deref());

        Ok(PoeConfig { term, definition, context, term_plural, reference, comment })
    }

    fn derive_term(val: &serde_json::Value, schema: &ConversionSchema, index: usize) -> Option<String> {
        // let should_derive_term = schema.deriveTerm.unwrap_or(false);
        let should_derive_term = schema.derive_term;

        if should_derive_term {
            Some(format!("{}/{}", schema.base_path, index))
        }
        else {
            grab_nested_val_optional(val, schema.term.as_deref())
        }
    }
}

// merge every property that's in both the poeDefintion and the schema to the orignal Value
pub fn write_poe_to_value(value: &mut serde_json::Value, conversion_schema: &ConversionSchema, poe: PoeConfig) {

    // reference to the object that contains all the poe relevant fields
    let original_obj = grab_nested_value_mut(value, &poe.term).unwrap();

    // merge every property that's in both the poeDefintion and the schema to the orignal Value
    original_obj[&conversion_schema.definition] = serde_json::to_value(poe.definition).unwrap();

    if let Some(context) = &conversion_schema.context {
        original_obj[context] = serde_json::to_value(poe.context).unwrap();
    }
    if let Some(term_plural) = &conversion_schema.term_plural {
        original_obj[term_plural] = serde_json::to_value(poe.term_plural).unwrap();
    }
    if let Some(reference) = &conversion_schema.reference {
        original_obj[reference] = serde_json::to_value(poe.reference).unwrap();
    }
    if let Some(commment) = &conversion_schema.commment {
        original_obj[commment] = serde_json::to_value(poe.comment).unwrap();
    }
}