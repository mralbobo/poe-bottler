use crate::poe::PoeDefinition;


pub fn grab_nested_value<'a>(val: &'a serde_json::Value, path: &str) -> Option<&'a serde_json::Value> {
	let path = format!("/{}", path);
	val.pointer(&path)
}

//TODO: whelp... this grab nested thing is _basically_ pointless. Should just switch all these to use Value.pointer
pub fn grab_nested_value_mut<'a>(val: &'a mut serde_json::Value, path: &str) -> Option<&'a mut serde_json::Value> {
	let path = format!("/{}", path);
	val.pointer_mut(&path)
}

pub fn grab_nested_val(val: &serde_json::Value, path: &str) -> Option<String> {
	let thing = grab_nested_value(val, path);

	// do horrible thing that converts numbers into strings
	match thing {
			Some(serde_json::Value::String(val)) => Some(val.clone()),
			Some(serde_json::Value::Number(val)) => Some(val.to_string()),
			_ => None
	}
}

pub fn grab_nested_val_optional(val: &serde_json::Value, path: Option<&str>) -> Option<String> {
	grab_nested_val(val, path?)
}

pub fn grab_nested_map(val: &serde_json::Value, path: &str) -> Result<PoeDefinition, serde_json::Error>{
	let thing = grab_nested_value(val, path).unwrap();

	// let other:Result<PoeDefinition, serde_json::Error> = serde_json::from_value(thing.clone());

	serde_json::from_value(thing.clone())
}