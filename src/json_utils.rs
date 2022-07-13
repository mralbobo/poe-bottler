use crate::poe::PoeDefinition;



pub fn grab_nested_val(val: &serde_json::Value, path: &str) -> Option<String> {
	let thing = path.split('/')
	.fold(val, |acc, item| {
			&acc[item]
	});

	// do horrible thing that converts numbers into strings
	match thing {
			serde_json::Value::String(val) => Some(val.clone()),
			serde_json::Value::Number(val) => Some(format!("{}", val)),
			_ => None
	}
}

pub fn grab_nested_val_optional(val: &serde_json::Value, path: Option<&str>) -> Option<String> {
	grab_nested_val(val, path?)
}

pub fn grab_nested_map(val: &serde_json::Value, path: &str) -> PoeDefinition{
	let thing = path.split('/')
	.fold(val, |acc, item| {
			&acc[item]
	});
	serde_json::from_value(thing.clone()).unwrap()
}