use ileap_extension::*;
use regex::Regex;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use schemars::schema_for;
use serde_json::{to_string_pretty, Map, Value};
use std::env;
use std::fs::File;
use std::io::{Error, Write};
use url::Url;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    generate_schema::<ShipmentFootprint>()?;
    generate_schema::<Toc>()?;
    generate_schema::<Tad>()?;

    generate_example("d9be4477-e351-45b3-acd9-e1da05e6f633", "shipment-footprint")
        .await
        .unwrap();

    generate_example("f3c04ec8-b33a-43b1-9fa7-d6a448fd60af", "toc-footprint")
        .await
        .unwrap();

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

async fn generate_example(id: &str, file_name: &str) -> Result<(), Error> {
    let url = Url::parse(&format!("https://api.ileap.sine.dev/2/footprints/{id}")).unwrap();

    let client = reqwest::Client::new();

    let token = env::var("TOKEN").unwrap();

    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    let response = res.text().await.unwrap();

    let json_response = serde_json::from_str::<Map<String, Value>>(&response).unwrap();
    let example = serde_json::to_string_pretty(&json_response.get("data").unwrap()).unwrap();

    let mut example_file = File::create(format!("../specs/examples/{file_name}.json"))?;

    example_file.write_all(example.as_bytes())?;

    println!("specs/examples/{file_name}.json successfully created");

    Ok(())
}
