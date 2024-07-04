//! iLEAP Data Model Extension data model
use std::iter;

use chrono::{format, DateTime, Duration, Utc};
use pact_data_model::{GeographicScope, WrappedDecimal};
use quickcheck::Arbitrary;
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ShipmentFootprint {
    pub mass: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_of_items: Option<String>,
    pub shipment_id: String,
    pub tces: NonEmptyVec<Tce>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NonEmptyVec<T>(pub Vec<T>);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", rename = "TCE")]
pub struct Tce {
    pub tce_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toc_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoc_id: Option<String>,
    pub shipment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignment_id: Option<String>,
    pub mass: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    pub distance: GlecDistance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Location>,
    pub transport_activity: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voyage_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incoterms: Option<Incoterms>,
    #[serde(rename = "co2eWTW")]
    pub co2e_wtw: WrappedDecimal,
    #[serde(rename = "co2eTTW")]
    pub co2e_ttw: WrappedDecimal,
    #[serde(skip_serializing_if = "Option::is_none", rename = "noxTTW")]
    pub nox_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "soxTTW")]
    pub sox_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ch4TTW")]
    pub ch4_ttw: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "pmTTW")]
    pub pm_ttw: Option<WrappedDecimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Incoterms {
    Exw,
    Fca,
    Cpt,
    Cip,
    Dap,
    Dpu,
    Ddp,
    Fas,
    Fob,
    Cfr,
    Cif,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase", rename = "TOC")]
pub struct Toc {
    pub toc_id: String,
    pub is_verified: bool,
    pub is_accredited: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mode: TransportMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_distance_factor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TemperatureControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truck_loading_sequence: Option<TruckLoadingSequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_shipping_option: Option<AirShippingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_length: Option<FlightLength>,
    pub energy_carriers: NonEmptyVec<EnergyCarrier>,
    #[serde(rename = "co2eIntensityWTW")]
    pub co2e_intensity_wtw: WrappedDecimal,
    #[serde(rename = "co2eIntensityTTW")]
    pub co2e_intensity_ttw: WrappedDecimal,
    pub co2e_intensity_throughput: TocCo2eIntensityThroughput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glec_data_quality_index: Option<GlecDataQualityIndex>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TocCo2eIntensityThroughput {
    #[serde(rename = "TEUkm")]
    TEUkm,
    Tkm,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TemperatureControl {
    Ambient,
    Refrigerated,
    Mixed,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TruckLoadingSequence {
    Ltl,
    Ftl,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AirShippingOption {
    #[serde(rename = "belly freight")]
    BellyFreight,
    Freighter,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FlightLength {
    #[serde(rename = "short-haul")]
    ShortHaul,
    #[serde(rename = "long-haul")]
    LongHaul,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
// TODO: use a floating point or a decimal instead.
pub struct GlecDataQualityIndex(pub u8);

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", rename = "HOC")]
pub struct Hoc {
    pub hoc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_verified: bool,
    pub is_accredited: bool,
    pub hub_type: HubType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_control: Option<TemperatureControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transport_mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transport_mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    pub energy_carriers: NonEmptyVec<EnergyCarrier>,
    #[serde(rename = "co2eIntensityWTW")]
    pub co2e_intensity_wtw: HocCo2eIntensityThroughput,
    #[serde(rename = "co2eIntensityTTW")]
    pub co2e_intensity_ttw: WrappedDecimal,
    pub co2e_intensity_throughput: HocCo2eIntensityThroughput,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HocCo2eIntensityThroughput {
    #[serde(rename = "TEU")]
    TEU,
    Tonnes,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum HubType {
    Transshipment,
    StorageAndTransshipment,
    Warehouse,
    LiquidBulkTerminal,
    MaritimeContainerterminal,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase", rename = "TAD")]
/// Data Type "Transport Activity Data" of the iLEAP Technical Specifications
pub struct Tad {
    pub activity_id: ActivityId,              // Unique
    pub consignment_ids: Vec<ConsignementId>, // Unique
    pub distance: GlecDistance,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_factor: Option<WrappedDecimal>, // TODO replace with propoer type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_distance_factor: Option<WrappedDecimal>, // TODO replace with propoer type
    pub origin: Location,
    pub destination: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TransportMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_type: Option<PackagingOrTrEqType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging_or_tr_eq_amount: Option<usize>,
    // TODO: verify whether the absence of this property is intended.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub energy_carrier: EnergyCarrier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstocks: Option<Vec<Feedstock>>,
}

pub type ActivityId = String;
pub type ConsignementId = String;
pub type ShipmentId = String;

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GlecDistance {
    Actual(WrappedDecimal),
    Gcd(WrappedDecimal),
    Sfd(WrappedDecimal),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    pub city: String,
    pub country: GeographicScope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iata: Option<IataCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locode: Option<Locode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uic: Option<UicCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<WrappedDecimal>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum TransportMode {
    Road,
    Rail,
    Air,
    Sea,
    InlandWaterway,
    //Hub,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum PackagingOrTrEqType {
    Box,
    Pallet,
    Container,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnergyCarrier {
    pub energy_carrier: EnergyCarrierType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstocks: Option<Vec<Feedstock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_consumption_unit: Option<String>,
    #[serde(rename = "emissionFactorWTW")]
    pub emission_factor_wtw: WrappedDecimal,
    #[serde(rename = "emissionFactorTTW")]
    pub emission_factor_ttw: WrappedDecimal,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum EnergyCarrierType {
    Diesel,
    #[serde(rename = "HVO")]
    Hvo,
    Petrol,
    #[serde(rename = "CNG")]
    Cng,
    #[serde(rename = "LNG")]
    Lng,
    #[serde(rename = "LPG")]
    Lpg,
    #[serde(rename = "HFO")]
    Hfo,
    #[serde(rename = "MGO")]
    Mgo,
    #[serde(rename = "Aviation fuel")]
    AviationFuel,
    Hydrogen,
    Methanol,
    Electric,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Feedstock {
    pub feedstock: FeedstockType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedstock_percentage: Option<WrappedDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_provenance: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum FeedstockType {
    Fossil,
    #[serde(rename = "Natural gas")]
    NaturalGas,
    Grid,
    #[serde(rename = "Renewable electricity")]
    RenewableElectricity,
    #[serde(rename = "Cooking oil")]
    CookingOil,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IataCode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for IataCode {
    fn from(s: String) -> Self {
        if s.len() <= 3 {
            IataCode(s)
        } else {
            panic!("IATA code must be 3 characters long")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Locode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for Locode {
    fn from(s: String) -> Self {
        if s.len() == 5 {
            Locode(s)
        } else {
            panic!("LOCODE must be 5 characters long, got '{s}'")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UicCode(pub String);

// TODO: improve validation / Json Schema
impl From<String> for UicCode {
    fn from(s: String) -> Self {
        if s.len() == 2 {
            UicCode(s)
        } else {
            panic!("UIC code must be 2 characters long")
        }
    }
}

impl<T> From<Vec<T>> for NonEmptyVec<T> {
    fn from(v: Vec<T>) -> NonEmptyVec<T> {
        if v.is_empty() {
            panic!("Vector must not be empty")
        } else {
            NonEmptyVec(v)
        }
    }
}

impl From<u8> for GlecDataQualityIndex {
    fn from(v: u8) -> GlecDataQualityIndex {
        if v > 4 {
            panic!("Glec Data Quality Index must be between 0 and 4")
        } else {
            GlecDataQualityIndex(v)
        }
    }
}

#[derive(Clone)]
pub struct LowerAToZNumDash(String);

impl LowerAToZNumDash {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

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

impl Arbitrary for Toc {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Toc {
            toc_id: formatted_arbitrary_string("toc-", g),
            is_verified: bool::arbitrary(g),
            is_accredited: bool::arbitrary(g),
            description: Option::<String>::arbitrary(g),
            mode: TransportMode::arbitrary(g),
            load_factor: Option::<String>::arbitrary(g),
            empty_distance_factor: Option::<String>::arbitrary(g),
            temperature_control: Option::<TemperatureControl>::arbitrary(g),
            truck_loading_sequence: Option::<TruckLoadingSequence>::arbitrary(g),
            air_shipping_option: Option::<AirShippingOption>::arbitrary(g),
            flight_length: Option::<FlightLength>::arbitrary(g),
            energy_carriers: NonEmptyVec::<EnergyCarrier>::arbitrary(g),
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
        EnergyCarrier {
            energy_carrier: EnergyCarrierType::arbitrary(g),
            feedstocks: Option::<Vec<Feedstock>>::arbitrary(g),
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
        Tce {
            tce_id: formatted_arbitrary_string("tce-", g),
            toc_id: Option::<String>::arbitrary(g),
            hoc_id: Option::<String>::arbitrary(g),
            shipment_id: String::arbitrary(g),
            consignment_id: Option::<String>::arbitrary(g),
            mass: Decimal::from(u32::arbitrary(g)).into(),
            packaging_or_tr_eq_type: Option::<String>::arbitrary(g),
            packaging_or_tr_eq_amount: Option::<usize>::arbitrary(g),
            distance: GlecDistance::arbitrary(g),
            origin: Option::<Location>::arbitrary(g),
            destination: Option::<Location>::arbitrary(g),
            transport_activity: arbitrary_wrapped_decimal(g),
            departure_at: Option::<DateTime<Utc>>::from(
                Utc::now() + Duration::days(u8::arbitrary(g) as i64),
            ),
            arrival_at: Option::<DateTime<Utc>>::from(
                Utc::now() + Duration::days(u8::arbitrary(g) as i64),
            ),
            flight_no: Option::<String>::arbitrary(g),
            voyage_no: Option::<String>::arbitrary(g),
            incoterms: Option::<Incoterms>::arbitrary(g),
            co2e_wtw: Decimal::from(u32::arbitrary(g)).into(),
            co2e_ttw: Decimal::from(u32::arbitrary(g)).into(),
            nox_ttw: arbitrary_option_wrapped_decimal(g),
            sox_ttw: arbitrary_option_wrapped_decimal(g),
            ch4_ttw: arbitrary_option_wrapped_decimal(g),
            pm_ttw: arbitrary_option_wrapped_decimal(g),
        }
    }
}

impl Arbitrary for GlecDistance {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let glec_distance = &[
            GlecDistance::Actual(Decimal::from(u32::arbitrary(g)).into()),
            GlecDistance::Gcd(Decimal::from(u32::arbitrary(g)).into()),
            GlecDistance::Sfd(Decimal::from(u32::arbitrary(g)).into()),
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
    Decimal::from(u32::arbitrary(g)).into()
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
