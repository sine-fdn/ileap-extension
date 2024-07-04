use ileap_extension::*;
use quickcheck::{Arbitrary, Gen};
use regex::Regex;
use schemars::schema_for;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    generate_schema::<ShipmentFootprint>()?;
    generate_schema::<Toc>()?;
    generate_schema::<Tad>()?;
    generate_schema::<Hoc>()?;

    // let mut og = Gen::new(10);

    // let tce = Tce::arbitrary(&mut og);

    // println!("tce: {tce:?}");

    Ok(())
}

fn generate_schema<T: schemars::JsonSchema>() -> Result<(), Error> {
    let type_name = std::any::type_name::<T>();
    let type_name = type_name.split("::").collect::<Vec<&str>>();
    let type_name = type_name.last().unwrap_or(&"Could not parse type name");

    let regex = Regex::new(r"([^A-Z])([A-Z])").unwrap();

    let schema_name = regex
        .replace_all(type_name, "$1-$2")
        .into_owned()
        .to_lowercase();

    let schema = schema_for!(T);

    let schema_json =
        to_string_pretty(&schema).expect(&format!("Failed to serialize {type_name} schema"));

    let mut schema_file = File::create(format!("./schemas/{schema_name}.json"))?;

    schema_file.write_all(schema_json.as_bytes())?;

    println!("{schema_name}.json successfully created");

    Ok(())
}
