// Build me a tool in Rust that takes 2 JSON inputs:
// 1 is a JSON input with an unknown data structure 
//and the 2nd is the schema for that data structure.
//It then converts that data into a POEditor import data structure
// (https://poeditor.com/localization/files/json)
// Brownie points if it can handle the reverse case too.
//Take a POEditor JSON structure as input with a JSON representing the "unknown JSON" structure,
//and then convert the POEditor JSON into the structure defined by the input schema

//cut down initial scope, 3 "things"
// - a poe file which is a json file which contains an array of poe configuration objects
// - a json file with unknown structure that contains "somewhere",
//   the data representing an array of poe objects
// - the schema is a file of unknown type that maps "things" in the json to poe's
//   - favor partial representaion of schema, describes an interface that matches the relevant peices
//   - probably assume we're not collecting equivalent things from "many" places 
//part 2
// - "crazy schema" 




fn main() {
    poe_bottler::run().unwrap();
    //NEXT (unordered)
    // - got it functioning-ish
    // - turn hard coded file references into cli args
    // - unbreak the PoeConfig type to support maps as well at some point
    // - support Poe -> original file, probably using poe'd file, original file and schema as an input
    // - add groupBy support
    // - do proper error handling
    // - refactor for better organization
    // - test
    // - more complex schema
    //    - multiple paths in the same file (not everything in the same array)
    //    - needs to support in and out, so everything needs to be declared in some reversible manner. Maybe just multiple duplicate toml groups?
    //    - more derivations required? Potentially for more than just term. Or maybe just more detailed term.
    // - screw around with multi-threading (for the hell of it)
}
