use crate::poe::PoeDefinition;


pub fn grab_nested_value<'a>(val: &'a serde_json::Value, path: &str) -> &'a serde_json::Value {
	path.split('/')
	.fold(val, |acc, item| {
			&acc[item]
	})
}

pub fn grab_nested_value_mut<'a>(val: &'a mut serde_json::Value, path: &str) -> Option<&'a mut serde_json::Value> {
	path.split('/')
	.fold(Some(val), |acc, item| {
			let acc = acc?;
			
			//convert to a number to index arrays if it's a number
			//will technically prevent writing back to objects indexed by numbers
			match item.parse::<usize>() {
					Ok(num) => acc.get_mut(num),
					__ => acc.get_mut(item)
			}
	})
}

pub fn grab_nested_val(val: &serde_json::Value, path: &str) -> Option<String> {
	let thing = grab_nested_value(val, path);

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
	let thing = grab_nested_value(val, path);

	// let other:Result<PoeDefinition, serde_json::Error> = serde_json::from_value(thing.clone());

	serde_json::from_value(thing.clone())
}