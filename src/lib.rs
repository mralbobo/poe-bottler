use std::{error::Error, path::Path, fs::File, io::Read, io::Write, io::BufWriter};

use clap::Parser;
use crate::{conversion_schema::ConversionSchema, poe::PoeConfig};

mod json_utils;
mod poe;
mod conversion_schema;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliConfig {
	/// Input file "random json"
	#[clap(short, long, value_parser)]
	pub input: String,
	/// Schema to define mapping from random json to poe
	#[clap(short, long, value_parser)]
	pub schema: String,
	/// Place to write the output file
	#[clap(short, long, value_parser)]
	pub output: String
}


pub fn run() -> Result<(), Box<dyn Error>> {
	let cli_config = CliConfig::parse();

	// let s = read_from_file("seafloor_survey_01.json");
	let s = read_from_file(&cli_config.input);
	let random_json:serde_json::Value = serde_json::from_str(&s).unwrap();

	// println!("{:#?}", randomJson);

	// let s = read_from_file("conversion_schema.toml");
	let s = read_from_file(&cli_config.schema);
	let conversion_schema: ConversionSchema = toml::from_str(&s).unwrap();

	assert_eq!(conversion_schema.base_path, "Dialogue");

	let base = &random_json[&conversion_schema.base_path];
	assert!(base.is_array());

	let base = base.as_array().unwrap();

	//this index access thing seems to let you nest through
	//multiple layers without validation, and you just get
	//a null at the end if it doesn't exist
	//...despite rust confidently declaring nulls don't exist
	// let definition = base[0][&conversion_schema.definition].to_string();

	let poed_list:Vec<PoeConfig> = base.iter()
			.enumerate()
			.filter_map(|(i, item)| {
				match PoeConfig::from_value(item, &conversion_schema, i){
					Ok(p) => Some(p),
					Err(_e) => {
						println!("Error parsing map or string on item number {}", i);
						None
					}
				}
			})
			.collect();

	// let file = File::create("test-output.json").unwrap();
	let file = File::create(&cli_config.output).unwrap();

	let mut writer = BufWriter::new(file);

	serde_json::to_writer_pretty(&mut writer, &poed_list).unwrap();
	writer.flush().unwrap();
	Ok(())
}

//todo - replace with direct buffer reading, maybe
fn read_from_file(string_path: &str) -> String {
	let path = Path::new(string_path);
	let mut file = File::open(path).unwrap();

	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	s
}