use std::{error::Error, path::Path, fs::File, io::Read, io::Write, io::BufWriter};

use clap::Parser;
use poe::write_poe_to_value;
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
	pub output: String,
	/// A poe formatted file to pull data from
	#[clap(short, long, value_parser)]
	pub poe_file: Option<String>
}

enum RunMode {
	ToPoe,
	FromPoe(String)
}

//hardcode to array for now, _eventually_ support maps as well, somehow
fn get_base_val<'a>(val: &'a serde_json::Value, base_path: &str) -> &'a Vec<serde_json::Value> {
	let base = &val[base_path];
	assert!(base.is_array());

	base.as_array().unwrap()
}

fn run_from_poe(conversion_schema: ConversionSchema, mut original_json: serde_json::Value, original_output: Vec<PoeConfig>, output_file_name: &str) {

	for poe in original_output {
		// merge every property that's in both the poeDefintion and the schema to the orignal Value
		write_poe_to_value(&mut original_json, &conversion_schema, poe);
	}

	let file = File::create(output_file_name).unwrap();

	let mut writer = BufWriter::new(file);

	serde_json::to_writer_pretty(&mut writer, &original_json).unwrap();
	writer.flush().unwrap();
}

fn run_to_poe(conversion_schema: ConversionSchema, original_json: serde_json::Value, output_file_name: &str) {
	let base = get_base_val(&original_json, &conversion_schema.base_path);

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
	let file = File::create(output_file_name).unwrap();

	let mut writer = BufWriter::new(file);

	serde_json::to_writer_pretty(&mut writer, &poed_list).unwrap();
	writer.flush().unwrap();
}

pub fn run() -> Result<(), Box<dyn Error>> {
	let cli_config = CliConfig::parse();

	let mode = detect_mode(&cli_config);

	let s = read_from_file(&cli_config.schema);
	let conversion_schema: ConversionSchema = toml::from_str(&s)?;

	let s = read_from_file(&cli_config.input);
	let original_json:serde_json::Value = serde_json::from_str(&s)?;

	match mode {
		RunMode::ToPoe => run_to_poe(conversion_schema, original_json, &cli_config.output),
		RunMode::FromPoe(file_name) => {
			let s = &read_from_file(&file_name);
			let original_output: Vec<PoeConfig> = serde_json::from_str(s).unwrap();
			run_from_poe(conversion_schema, original_json, original_output, &cli_config.output)
		}
	};
	
	Ok(())
}


fn detect_mode(cli_config: &CliConfig) -> RunMode {
	match &cli_config.poe_file {
		Some(file_name) => RunMode::FromPoe(file_name.clone()),
		None => RunMode::ToPoe
	}
}

//todo - replace with direct buffer reading, maybe
fn read_from_file(string_path: &str) -> String {
	let path = Path::new(string_path);
	let mut file = File::open(path).unwrap();

	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();
	s
}