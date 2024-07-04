use crate::*;
use chrono::Duration;
use quickcheck::Arbitrary;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

#[derive(Clone)]
pub struct LowerAToZNumDash(String);

impl LowerAToZNumDash {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Arbitrary for LowerAToZNumDash {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let s: Vec<()> = Vec::arbitrary(g);
        let s: String = s
            .into_iter()
            .map(|_| {
                // ASCII characters -, 0..9, a..z
                let i = u8::arbitrary(g) % 37;
                match i {
                    0 => '-',
                    1..=10 => (i as u8 + 47) as char,
                    _ => (i as u8 + 86) as char,
                }
            })
            .collect();
        Self(s)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        let s = self.0.clone();
        let range = 0..self.len();
        let shrunk: Vec<_> = range
            .into_iter()
            .map(|len| Self(s[0..len].to_string()))
            .collect();
        Box::new(shrunk.into_iter())
    }
}

fn formatted_arbitrary_string(fixed: &str, g: &mut quickcheck::Gen) -> String {
    fixed.to_string() + &LowerAToZNumDash::arbitrary(g).0
}

impl Arbitrary for ShipmentFootprint {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        ShipmentFootprint {
            mass: String::arbitrary(g),
            volume: Option::<String>::arbitrary(g),
            number_of_items: Option::<String>::arbitrary(g),
            type_of_items: Option::<String>::arbitrary(g),
            shipment_id: formatted_arbitrary_string("shipment-", g),
            tces: NonEmptyVec::<Tce>::arbitrary(g),
        }
    }
}

impl Arbitrary for Hoc {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Hoc {
            hoc_id: formatted_arbitrary_string("hoc-", g),
            description: Option::<String>::arbitrary(g),
            is_verified: bool::arbitrary(g),
            is_accredited: bool::arbitrary(g),
            hub_type: HubType::arbitrary(g),
            temperature_control: Option::<TemperatureControl>::arbitrary(g),
            hub_location: Option::<Location>::arbitrary(g),
            inbound_transport_mode: Option::<TransportMode>::arbitrary(g),
            outbound_transport_mode: Option::<TransportMode>::arbitrary(g),
            packaging_or_tr_eq_type: Option::<PackagingOrTrEqType>::arbitrary(g),
            packaging_or_tr_eq_amount: Option::<usize>::arbitrary(g),
            energy_carriers: NonEmptyVec::<EnergyCarrier>::arbitrary(g),
            co2e_intensity_wtw: arbitrary_wrapped_decimal(g),
            co2e_intensity_ttw: arbitrary_wrapped_decimal(g),
            co2e_intensity_throughput: String::arbitrary(g),
        }
    }
}

impl Arbitrary for HubType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let hub_type = &[
            HubType::Transshipment,
            HubType::StorageAndTransshipment,
            HubType::Warehouse,
            HubType::LiquidBulkterminal,
            HubType::MaritimeContainerterminal,
        ];

        g.choose(hub_type).unwrap().to_owned()
    }
}

impl Arbitrary for PackagingOrTrEqType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let packaging_or_tr_eq_type = &[
            PackagingOrTrEqType::Box,
            PackagingOrTrEqType::Pallet,
            PackagingOrTrEqType::Container,
        ];

        g.choose(packaging_or_tr_eq_type).unwrap().to_owned()
    }
}

fn arbitrary_option_factor(g: &mut quickcheck::Gen) -> Option<String> {
    let rand_num = u8::arbitrary(g) % 10 + 1;
    let rand_factor: Decimal = Decimal::new(rand_num as i64, 1);

    Some(rand_factor.to_string())
}

impl Arbitrary for Toc {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mode = TransportMode::arbitrary(g);

        let (air_shipping_option, flight_length) = match mode {
            TransportMode::Air => (
                Option::<AirShippingOption>::arbitrary(g),
                Option::<FlightLength>::arbitrary(g),
            ),
            _ => (None, None),
        };

        let energy_carriers = NonEmptyVec::<EnergyCarrier>::arbitrary(g);

        Toc {
            toc_id: formatted_arbitrary_string("toc-", g),
            is_verified: bool::arbitrary(g),
            is_accredited: bool::arbitrary(g),
            // TODO: description is currently None for simplicity.
            description: None,
            mode,
            load_factor: arbitrary_option_factor(g),
            empty_distance_factor: arbitrary_option_factor(g),
            temperature_control: Option::<TemperatureControl>::arbitrary(g),
            truck_loading_sequence: Option::<TruckLoadingSequence>::arbitrary(g),
            air_shipping_option,
            flight_length,
            energy_carriers,
            co2e_intensity_wtw: arbitrary_wrapped_decimal(g),
            co2e_intensity_ttw: arbitrary_wrapped_decimal(g),
            co2e_intensity_throughput: String::arbitrary(g),
            glec_data_quality_index: Option::<GlecDataQualityIndex>::arbitrary(g),
        }
    }
}

impl<T: Arbitrary> Arbitrary for NonEmptyVec<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        NonEmptyVec(vec![T::arbitrary(g)])
    }
}

impl Arbitrary for TransportMode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let transport_mode = &[
            TransportMode::Road,
            TransportMode::Rail,
            TransportMode::Air,
            TransportMode::Sea,
            TransportMode::InlandWaterway,
        ];

        g.choose(transport_mode).unwrap().to_owned()
    }
}

impl Arbitrary for TemperatureControl {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let temperature_control = &[
            TemperatureControl::Ambient,
            TemperatureControl::Refrigerated,
            TemperatureControl::Mixed,
        ];

        g.choose(temperature_control).unwrap().to_owned()
    }
}

impl Arbitrary for TruckLoadingSequence {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let truck_loading_sequence = &[TruckLoadingSequence::Ftl, TruckLoadingSequence::Ltl];

        g.choose(truck_loading_sequence).unwrap().to_owned()
    }
}

impl Arbitrary for AirShippingOption {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let air_shipping_option = &[
            AirShippingOption::BellyFreight,
            AirShippingOption::Freighter,
        ];

        g.choose(air_shipping_option).unwrap().to_owned()
    }
}

impl Arbitrary for FlightLength {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let flight_length = &[FlightLength::ShortHaul, FlightLength::LongHaul];

        g.choose(flight_length).unwrap().to_owned()
    }
}

impl Arbitrary for EnergyCarrier {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let energy_carrier = EnergyCarrierType::arbitrary(g);

        let feedstocks = Option::<Vec<Feedstock>>::arbitrary(g);

        let Some(mut feedstocks) = feedstocks else {
            return EnergyCarrier {
                energy_carrier,
                feedstocks,
                // TODO: energy_consumption and energy_consumption_unit are currently None for simplicity.
                energy_consumption: None,
                energy_consumption_unit: None,
                emission_factor_wtw: arbitrary_wrapped_decimal(g),
                emission_factor_ttw: arbitrary_wrapped_decimal(g),
            };
        };

        // TODO: verify which feedstocks make sense for each energy carrier.
        feedstocks = feedstocks
            .iter()
            .filter(|f| match energy_carrier {
                EnergyCarrierType::Diesel => f.feedstock == FeedstockType::Fossil,
                EnergyCarrierType::Hvo => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Petrol => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Cng => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Lng => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Lpg => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Hfo => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Mgo => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::AviationFuel => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Hydrogen => f.feedstock == FeedstockType::CookingOil,
                EnergyCarrierType::Methanol => {
                    f.feedstock == FeedstockType::Fossil
                        || f.feedstock == FeedstockType::NaturalGas
                        || f.feedstock == FeedstockType::CookingOil
                }
                EnergyCarrierType::Electric => {
                    f.feedstock == FeedstockType::Grid
                        || f.feedstock == FeedstockType::RenewableElectricity
                }
            })
            .cloned()
            .collect::<Vec<Feedstock>>();

        EnergyCarrier {
            energy_carrier,
            feedstocks: Some(feedstocks),
            energy_consumption: arbitrary_option_wrapped_decimal(g),
            energy_consumption_unit: Option::<String>::arbitrary(g),
            emission_factor_wtw: arbitrary_wrapped_decimal(g),
            emission_factor_ttw: arbitrary_wrapped_decimal(g),
        }
    }
}

impl Arbitrary for EnergyCarrierType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let energy_carrier = &[
            EnergyCarrierType::Diesel,
            EnergyCarrierType::Hvo,
            EnergyCarrierType::Petrol,
            EnergyCarrierType::Cng,
            EnergyCarrierType::Lng,
            EnergyCarrierType::Lpg,
            EnergyCarrierType::Hfo,
            EnergyCarrierType::Mgo,
            EnergyCarrierType::AviationFuel,
            EnergyCarrierType::Hydrogen,
            EnergyCarrierType::Methanol,
            EnergyCarrierType::Electric,
        ];

        g.choose(energy_carrier).unwrap().to_owned()
    }
}

impl Arbitrary for Feedstock {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Feedstock {
            feedstock: FeedstockType::arbitrary(g),
            feedstock_percentage: arbitrary_option_wrapped_decimal(g),
            region_provenance: Option::<String>::arbitrary(g),
        }
    }
}

impl Arbitrary for FeedstockType {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let feedstock = &[
            FeedstockType::Fossil,
            FeedstockType::NaturalGas,
            FeedstockType::Grid,
            FeedstockType::RenewableElectricity,
            FeedstockType::CookingOil,
        ];

        g.choose(feedstock).unwrap().to_owned()
    }
}

impl Arbitrary for GlecDataQualityIndex {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        GlecDataQualityIndex(u8::arbitrary(g) % 5)
    }
}

impl Arbitrary for Tce {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let toc_id = if bool::arbitrary(g) {
            Some(formatted_arbitrary_string("toc-", g))
        } else {
            None
        };

        let hoc_id = match toc_id {
            Some(_) => None,
            None => Some(formatted_arbitrary_string("hoc-", g)),
        };

        let mass = arbitrary_wrapped_decimal(g);
        let glec_distance = GlecDistance::arbitrary(g);

        let distance = match &glec_distance {
            GlecDistance::Actual(d) => d,
            GlecDistance::Gcd(d) => d,
            GlecDistance::Sfd(d) => d,
        };

        let transport_activity = WrappedDecimal::from(mass.0 * distance.0);

        let departure_at =
            Option::<DateTime<Utc>>::from(Utc::now() + Duration::days(u8::arbitrary(g) as i64));

        let arrival_at = match departure_at {
            None => None,
            Some(departure) => {
                // Assuming an average speed of 100 km/h, calculate the arrival time based on the
                // distance, rounded.
                let hours = (distance.0 / Decimal::from(100)).round().to_i64().unwrap();

                Some(departure + Duration::hours(hours))
            }
        };

        let co2e_wtw = WrappedDecimal::from(match toc_id {
            // TODO: toc.co2e_intensity_wtw is currently hardcoded, based on the toc-road-1 example.
            Some(_) => Decimal::new(116, 3) * transport_activity.0,
            // TODO: if toc_id is None, then we must use hoc.co2e_intensity_wtw; this is currently
            // hardcoded, based on the hoc-transshipment-1 example. However, I do not know how to
            // calculate it.
            None => Decimal::new(33, 0),
        });

        let co2e_ttw = WrappedDecimal::from(match toc_id {
            // TODO: toc.co2e_intensity_ttw is currently hardcoded, based on the toc-road-1 example.
            Some(_) => Decimal::new(89, 3) * transport_activity.0,
            // TODO: if toc_id is None, then we must use hoc.co2e_intensity_ttw; this is currently
            // hardcoded, based on the hoc-transshipment-1 example. However, I do not know how to
            // calculate it.
            None => Decimal::new(10, 0),
        });

        Tce {
            tce_id: formatted_arbitrary_string("tce-", g),
            toc_id,
            hoc_id,
            shipment_id: formatted_arbitrary_string("shipment-", g),
            // TODO: consignment_id is currently None for simplicity.
            consignment_id: None,
            mass,
            packaging_or_tr_eq_type: Option::<PackagingOrTrEqType>::arbitrary(g),
            packaging_or_tr_eq_amount: Option::<usize>::arbitrary(g),
            distance: glec_distance,
            // TODO: origin and destination are currently None to avoid an inconsistencies with the
            // distance field. In order to fix this, we need to ensure that either the distance is
            // calculated from the origin and destination or that the origin and destination are set
            // based on the distance.
            origin: None,
            destination: None,
            transport_activity,
            departure_at,
            arrival_at,
            // TODO: flight_no and voyage_no are currently None for simplicity.
            flight_no: None,
            voyage_no: None,
            incoterms: Option::<Incoterms>::arbitrary(g),
            co2e_wtw,
            co2e_ttw,
            // TODO: all the following fields are currently None for simplicity.
            nox_ttw: None,
            sox_ttw: None,
            ch4_ttw: None,
            pm_ttw: None,
        }
    }
}

impl Arbitrary for GlecDistance {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let glec_distance = &[
            GlecDistance::Actual(Decimal::from(u16::arbitrary(g)).into()),
            GlecDistance::Gcd(Decimal::from(u16::arbitrary(g)).into()),
            GlecDistance::Sfd(Decimal::from(u16::arbitrary(g)).into()),
        ];

        g.choose(glec_distance).unwrap().to_owned()
    }
}

impl Arbitrary for Location {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Location {
            street: Option::<String>::arbitrary(g),
            zip: Option::<String>::arbitrary(g),
            city: String::arbitrary(g),
            country: GeographicScope::Country {
                geography_country: pact_data_model::ISO3166CC(String::arbitrary(g)),
            },
            iata: Option::<IataCode>::arbitrary(g),
            locode: Option::<Locode>::arbitrary(g),
            uic: Option::<UicCode>::arbitrary(g),
            lat: arbitrary_option_wrapped_decimal(g),
            lng: arbitrary_option_wrapped_decimal(g),
        }
    }
}

impl Arbitrary for IataCode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut s = String::new();

        for _ in 0..3 {
            let ascii_capital = ((u8::arbitrary(g) % 26) + 65) as char;
            s.push(ascii_capital)
        }

        IataCode::from(s)
    }
}

impl Arbitrary for Locode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut s = String::new();

        for _ in 0..5 {
            // 65..90 - ASCII A to Z
            let ascii_capital = ((u8::arbitrary(g) % 26) + 65) as char;
            s.push(ascii_capital)
        }

        Locode::from(s)
    }
}

impl Arbitrary for UicCode {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let mut s = String::new();

        for _ in 0..2 {
            let int = (u8::arbitrary(g) % 9) + 1;
            s.push(int as char)
        }

        UicCode::from(s)
    }
}

fn arbitrary_wrapped_decimal(g: &mut quickcheck::Gen) -> WrappedDecimal {
    Decimal::from(u16::arbitrary(g)).into()
}

fn arbitrary_option_wrapped_decimal(g: &mut quickcheck::Gen) -> Option<WrappedDecimal> {
    let option = &[Some(arbitrary_wrapped_decimal(g)), None];

    g.choose(option).unwrap().to_owned()
}

impl Arbitrary for Incoterms {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let incoterms = &[
            Incoterms::Exw,
            Incoterms::Fca,
            Incoterms::Cpt,
            Incoterms::Cip,
            Incoterms::Dap,
            Incoterms::Dpu,
            Incoterms::Ddp,
            Incoterms::Fas,
            Incoterms::Fob,
            Incoterms::Cfr,
            Incoterms::Cif,
        ];

        g.choose(incoterms).unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Hoc, ShipmentFootprint, Tce, Toc};
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn ser_and_deser_tce(tce: Tce) -> bool {
        let serialized = serde_json::to_string(&tce).unwrap();
        let deserialized = serde_json::from_str::<Tce>(&serialized).unwrap();

        println!("tce: {tce:?}");
        println!("serialized: {serialized}");
        println!("deserialized: {deserialized:?}");

        deserialized == tce
        // true
    }

    #[quickcheck]
    fn ser_and_deser_toc(toc: Toc) -> bool {
        let serialized = serde_json::to_string(&toc).unwrap();
        let deserialized = serde_json::from_str::<Toc>(&serialized).unwrap();

        if deserialized != toc {
            println!("toc: {toc:?}");
            // println!("serialized: {serialized}");
            println!("deserialized: {deserialized:?}");
        }

        deserialized == toc
        // true
    }

    #[quickcheck]
    fn ser_and_deser_hoc(hoc: Hoc) -> bool {
        let serialized = serde_json::to_string(&hoc).unwrap();
        let deserialized = serde_json::from_str::<Hoc>(&serialized).unwrap();

        if deserialized != hoc {
            println!("toc: {hoc:?}");
            // println!("serialized: {serialized}");
            println!("deserialized: {deserialized:?}");
        }

        deserialized == hoc
        // true
    }

    #[quickcheck]
    fn ser_and_deser_ship_foot(ship_foot: ShipmentFootprint) {
        let serialized = serde_json::to_string(&ship_foot).unwrap();
        let deserialized = serde_json::from_str::<ShipmentFootprint>(&serialized).unwrap();

        if deserialized != ship_foot {
            println!("ship_foot: {ship_foot:?}");
            // println!("serialized: {serialized}");
            println!("deserialized: {deserialized:?}");
        }

        assert_eq!(deserialized, ship_foot);
    }
}
