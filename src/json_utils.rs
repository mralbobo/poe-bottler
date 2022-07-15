use crate::poe::PoeDefinition;



pub fn grab_nested_val(val: &serde_json::Value, path: &str) -> Option<String> {
	let thing = path.split('/')
	.fold(val, |acc, item| {
			&acc[item]
	});

	// do horrible thing that converts numbers into strings
	match thing {
			serde_json::Value::String(val) => Some(val.clone()),
			serde_json::Value::Number(val) => Some(val.to_string()),
			_ => None
	}
}

pub fn grab_nested_val_optional(val: &serde_json::Value, path: Option<&str>) -> Option<String> {
	grab_nested_val(val, path?)
}

pub fn grab_nested_map(val: &serde_json::Value, path: &str) -> Result<PoeDefinition, serde_json::Error>{
	let thing = path.split('/')
	.fold(val, |acc, item| {
			&acc[item]
	});

	// let other:Result<PoeDefinition, serde_json::Error> = serde_json::from_value(thing.clone());

	serde_json::from_value(thing.clone())
}