//! iLEAP Data Model Extension data model
use std::iter::Product;

use chrono::{DateTime, Datelike, TimeZone, Utc};
use pact_data_model::{
    CarbonFootprint, CharacterizationFactors, CompanyIdSet, CrossSectoralStandard,
    CrossSectoralStandardSet, DataModelExtension, DeclaredUnit, ExemptedEmissionsPercent,
    GeographicScope, IpccCharacterizationFactorsSource, IpccCharacterizationFactorsSources,
    NonEmptyString, PfId, PfStatus, ProductFootprint, ProductIdSet, ProductOrSectorSpecificRule,
    ProductOrSectorSpecificRuleOperator, ProductOrSectorSpecificRuleSet, SpecVersionString, Urn,
    VersionInteger, WrappedDecimal,
};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub prev_tce_ids: Option<Vec<String>>,
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
    pub co2e_intensity_wtw: WrappedDecimal,
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
    pub feedstock_percentage: Option<f64>,
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
            panic!("LOCODE must be 5 characters long")
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

pub fn toc_to_pcf(
    toc: Toc,
    pf_id: Option<PfId>,
    company_name: NonEmptyString,
    company_ids: CompanyIdSet,
) -> ProductFootprint {
    ProductFootprint {
        id: pf_id.unwrap_or(PfId(Uuid::new_v4())),
        spec_version: SpecVersionString("2.2.0".to_string()),
        preceding_pf_ids: None, // Kept None for simplicity
        version: VersionInteger(1),
        created: Utc::now(),
        updated: None, // Kept None for simplicity
        status: PfStatus::Active,
        status_comment: None,        // Kept None for simplicity
        validity_period_start: None, // Kept None for simplicity
        validity_period_end: None,   // Kept None for simplicity
        company_name,
        company_ids,
        product_description: format!("TOC emission profile with id {}", toc.toc_id).into(),
        product_ids: ProductIdSet(vec![Urn(format!(
            "urn:pathfinder:product:customcode:vendor-assigned:toc:{}",
            toc.toc_id
        ))]), // Kept as vendor assigned only for simplicity
        product_category_cpc: "83117".to_string().into(),
        product_name_company: format!("TOC emission profile with id {}", toc.toc_id).into(), // Assuming that TOCs do not usually have trade names
        comment: "".to_string(), // Kept empty for simplicty
        extensions: Some(vec![DataModelExtension {
            spec_version: "0.2.1".to_string().into(), // TODO: verify if this should be the extension's or the PACT version
            data_schema: "https://api.ileap.sine.dev/toc.json".to_string(),
            documentation: Some("https://sine-fdn.github.io/ileap-extension/".to_string()),
            data: serde_json::from_str(&serde_json::to_string(&toc).unwrap()).unwrap(),
        }]),
        pcf: CarbonFootprint {
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: Decimal::from(1).into(),
            p_cf_excluding_biogenic: toc.co2e_intensity_wtw.0.into(),
            p_cf_including_biogenic: None, // Kept None for simplicity until more clarity is gained
            fossil_ghg_emissions: toc.co2e_intensity_wtw.0.into(),
            fossil_carbon_content: toc.co2e_intensity_wtw.0.into(), // No guidance in Tech Specs
            biogenic_carbon_content: toc.co2e_intensity_wtw.0.into(), // No guidance in Tech Specs
            d_luc_ghg_emissions: None, // Kept None for simplicity; no guidance in the Tech Specs
            land_management_ghg_emissions: None, // Kept None for simplicity; no guidance in the Tech Specs
            other_biogenic_ghg_emissions: None, // Kept None for simplicity; no guidance in the Tech Specs
            i_luc_ghg_emissions: None, // Kept None for simplicity; no guidance in the Tech Specs
            biogenic_carbon_withdrawal: None, // Kept None for simplicity; no guidance in the Tech Specs
            aircraft_ghg_emissions: None, // Kept None for simplicity; no guidance in the Tech Specs
            characterization_factors: CharacterizationFactors::Ar5, // Conservative option
            ipcc_characterization_factors_sources: IpccCharacterizationFactorsSources(vec![
                "AR5".to_string().into(),
                "AR6".to_string().into(),
            ]), // Assume the more general option for simplicity
            cross_sectoral_standards_used: CrossSectoralStandardSet(vec![
                CrossSectoralStandard::Ghgp,
            ]), // Assume one variant until more clarity is gained, no guidance in the Tech Specs
            product_or_sector_specific_rules: ProductOrSectorSpecificRuleSet(vec![
                ProductOrSectorSpecificRule {
                    operator: ProductOrSectorSpecificRuleOperator::Other,
                    rule_names: vec!["ISO 14083".to_string().into()].into(),
                    other_operator_name: Some("SFC".to_string().into()),
                },
            ]), // Further clarification is needed
            biogenic_accounting_methodology: None, // Kept None for simplicity; no guidance in the Tech Specs
            boundary_processes_description: "".to_string(), // Kept empty for simplicity; will become optional in v3
            reference_period_start: Utc
                .with_ymd_and_hms(
                    Utc::now().year() - 1,
                    Utc::now().month(),
                    Utc::now().day(),
                    0,
                    0,
                    0,
                )
                .unwrap(), // Kept as one year before the creation date for simplicity
            reference_period_end: Utc
                .with_ymd_and_hms(
                    Utc::now().year(),
                    Utc::now().month(),
                    Utc::now().day() - 1,
                    0,
                    0,
                    0,
                )
                .unwrap(), // Kept as one day before the creation date for simplicity
            geographic_scope: None,                         // Assume global for simplicity
            secondary_emission_factor_sources: None, // Kept None for simplicity; no guidance in the Tech Specs
            exempted_emissions_percent: ExemptedEmissionsPercent(0.0), // Kept at 0% for simplicity
            exempted_emissions_description: "".to_string(), // Kept empty for simplicity
            packaging_emissions_included: false,
            packaging_ghg_emissions: None,
            allocation_rules_description: None, // Kept None for simplicity; no guidance in the Tech Specs
            uncertainty_assessment_description: None, // Kept None for simplicity; no guidance in the Tech Specs
            primary_data_share: None, // Kept None for simplicity; no guidance in the Tech Specs
            dqi: None,                // Kept None for simplicity; no guidance in the Tech Specs
            assurance: None,          // Kept None for simplicity; no guidance in the Tech Specs
        },
    }
}
