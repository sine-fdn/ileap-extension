use ileap_extension::*;
use pact_data_model::{Urn, WrappedDecimal};
use quickcheck::{Arbitrary, Gen};
use regex::Regex;
use rust_decimal::Decimal;
use schemars::schema_for;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    generate_schema::<ShipmentFootprint>()?;
    generate_schema::<Toc>()?;
    generate_schema::<Tad>()?;
    generate_schema::<Hoc>()?;

    generate_demo_data()?;

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

    let schema_json = to_string_pretty(&schema)
        .unwrap_or_else(|_| panic!("Failed to serialize {type_name} schema"));

    let mut schema_file = File::create(format!("./schemas/{schema_name}.json"))?;

    schema_file.write_all(schema_json.as_bytes())?;

    println!("{schema_name}.json successfully created");

    Ok(())
}

fn generate_demo_data() -> Result<(), Error> {
    let mut og = Gen::new(10);

    let mut shipment_footprints = vec![];
    let mut tocs = vec![];
    let mut hocs = vec![];
    for _ in 0..1 {
        let mut ship_foot = ShipmentFootprint::arbitrary(&mut og);

        let mut tces: Vec<Tce> = vec![];
        let mut prev_tces: Vec<String> = vec![];

        let mut i = 0;
        let limit = u8::arbitrary(&mut og) % 5 + 1;
        loop {
            let mut tce = Tce::arbitrary(&mut og);

            if (tce.toc_id.is_none() && tce.hoc_id.is_none())
                || tce.toc_id.is_some() && tce.hoc_id.is_some()
            {
                panic!("Either Toc or Hoc, but not both, must be provided");
            }

            if let Some(prev_tce) = tces.last() {
                // Updates prevTceIds for the current TCE
                prev_tces.push(prev_tce.tce_id.clone());
                tce.prev_tce_ids = Some(prev_tces.clone());

                // Avoids having two HOCs follow one another
                if prev_tce.hoc_id.is_some() && tce.hoc_id.is_some() {
                    continue;
                }
            };

            if tce.hoc_id.is_some() {
                // Avoids having an HOC as the first or the last TCE
                if i == 0 || i == limit - 1 {
                    continue;
                }

                let mut hoc = Hoc::arbitrary(&mut og);
                hoc.hoc_id = tce.hoc_id.clone().unwrap();

                tce.hoc_id = Some(hoc.hoc_id.clone());

                tce.distance = GlecDistance::Actual(Decimal::from(0).into());
                tce.transport_activity = Decimal::from(0).into();

                // TODO: Double-check divisor
                tce.co2e_wtw = WrappedDecimal::from(
                    ((hoc.co2e_intensity_wtw.0 * tce.mass.0) / Decimal::from(1000000)).round_dp(2),
                );
                tce.co2e_ttw = WrappedDecimal::from(
                    ((hoc.co2e_intensity_ttw.0 * tce.mass.0) / Decimal::from(1000000)).round_dp(2),
                );

                let hoc = to_pcf(
                    ILeapType::Hoc(hoc),
                    "SINE Foundation",
                    "urn:sine:example",
                    Some(HocTeuContainerSize::Normal),
                    Some(vec![CharFactors::Ar6]),
                );

                hocs.push(hoc);
            }

            if tce.toc_id.is_some() {
                let mut toc = Toc::arbitrary(&mut og);
                toc.toc_id = tce.toc_id.clone().unwrap();

                tce.transport_activity = (tce.mass.0 * tce.distance.get_distance())
                    .round_dp(2)
                    .into();

                tce.toc_id = Some(toc.toc_id.clone());
                tce.co2e_wtw = WrappedDecimal::from(
                    (toc.co2e_intensity_wtw.0 * tce.transport_activity.0).round_dp(2),
                );
                tce.co2e_ttw = WrappedDecimal::from(
                    (toc.co2e_intensity_ttw.0 * tce.transport_activity.0).round_dp(2),
                );

                let toc = to_pcf(
                    ILeapType::Toc(toc),
                    "SINE Foundation",
                    "urn:sine:example",
                    None,
                    Some(vec![CharFactors::Ar6]),
                );

                tocs.push(toc.clone());
            }

            tces.push(tce);
            i += 1;
            if i == limit {
                break;
            }
        }

        ship_foot.tces = NonEmptyVec::from(tces);

        let ship_foot = to_pcf(
            ILeapType::ShipmentFootprint(ship_foot),
            "SINE Foundation",
            "urn:sine:example",
            Some(HocTeuContainerSize::Normal),
            Some(vec![CharFactors::Ar6]),
        );

        shipment_footprints.push(ship_foot);
    }

    println!("{shipment_footprints:#?}");
    println!("{tocs:#?}");
    println!("{hocs:#?}");

    Ok(())
}
