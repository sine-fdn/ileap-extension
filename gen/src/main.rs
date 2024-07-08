use ileap_extension::*;
use pact_data_model::WrappedDecimal;
use quickcheck::{Arbitrary, Gen};
use regex::Regex;
use schemars::schema_for;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

pub(crate) fn update_arbitrary_tce(tce: &mut Tce, toc: Option<Toc>, hoc: Option<Hoc>) -> Tce {
    tce.toc_id = match toc.clone() {
        Some(toc) => Some(toc.toc_id),
        None => None,
    };

    tce.hoc_id = match hoc.clone() {
        Some(hoc) => Some(hoc.hoc_id),
        None => None,
    };

    if (toc.is_none() && hoc.is_none()) && toc.is_some() && hoc.is_some() {
        panic!("Either Toc or Hoc, but not both, must be provided");
    }

    match tce.toc_id {
        Some(_) => {
            let toc = toc.unwrap();
            tce.co2e_wtw =
                WrappedDecimal::from(toc.co2e_intensity_wtw.0 * tce.transport_activity.0);
            tce.co2e_ttw =
                WrappedDecimal::from(toc.co2e_intensity_ttw.0 * tce.transport_activity.0);
        }
        None => {
            let hoc = hoc.unwrap();
            tce.co2e_wtw = WrappedDecimal::from(hoc.co2e_intensity_wtw);
            tce.co2e_ttw = WrappedDecimal::from(hoc.co2e_intensity_ttw);
        }
    }

    tce.to_owned()
}

fn main() -> Result<(), Error> {
    // generate_schema::<ShipmentFootprint>()?;
    // generate_schema::<Toc>()?;
    // generate_schema::<Tad>()?;
    // generate_schema::<Hoc>()?;

    let mut og = Gen::new(10);

    let mut tce_1 = Tce::arbitrary(&mut og);
    let toc = Toc::arbitrary(&mut og);

    println!("toc: {toc:?}");

    let mut tce_2 = Tce::arbitrary(&mut og);
    let hoc = Hoc::arbitrary(&mut og);

    println!("hoc: {hoc:?}");

    let tce_toc = update_arbitrary_tce(&mut tce_1, Some(toc), None);
    let tce_hoc = update_arbitrary_tce(&mut tce_2, None, Some(hoc));

    println!("tce_toc: {tce_toc:?}");
    println!("tce_hoc: {tce_hoc:?}");

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
