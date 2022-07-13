
## mvp

## usage

Accepts an input json file, a conversion schema, and outputs to a file in the poe json format.

Currently exclusively supports input jsons that have one array somewhere in the json containing all the target text where said array has some consistent format.

build + run
```
cargo run -- --input="input_file.json" --output="test-output.json" --schema="conversion_schema.toml"
```

run
```
./poe-bottler --input="input_file.json" --output="test-output.json" --schema="conversion_schema.toml"
```

See `sample_files/conversion_schema.toml`







## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.