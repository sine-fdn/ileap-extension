use chrono::Utc;
use pact_data_model::{
    CarbonFootprint, CharacterizationFactors, CompanyIdSet, CrossSectoralStandard,
    CrossSectoralStandardSet, DataModelExtension, DeclaredUnit, ExemptedEmissionsPercent,
    IpccCharacterizationFactorsSource, PfId, PfStatus, ProductFootprint, ProductIdSet,
    SpecVersionString, Urn, VersionInteger,
};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Hoc, HocCo2eIntensityThroughput, ShipmentFootprint, Toc};

pub enum HocTeuContainerSize {
    Normal,
    Light,
    Heavy,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq, Clone)]
pub enum ILeapType {
    ShipmentFootprint(ShipmentFootprint),
    Toc(Toc),
    Hoc(Hoc),
}

pub fn to_pcf(
    ileap_type: ILeapType,
    company_name: &str,
    company_urn: &str,
    hoc_container_size: Option<HocTeuContainerSize>,
    characterization_factors: Option<Vec<CharacterizationFactors>>,
) -> ProductFootprint {
    let (characterization_factors, characterization_factors_sources) =
        match characterization_factors {
            None => (
                CharacterizationFactors::Ar5,
                vec![IpccCharacterizationFactorsSource::from("AR5".to_string())],
            ),
            Some(cf) => {
                if cf.is_empty() {
                    (
                        CharacterizationFactors::Ar5,
                        vec![IpccCharacterizationFactorsSource::from("AR5".to_string())],
                    )
                } else {
                    let cf: Vec<IpccCharacterizationFactorsSource> = cf
                        .iter()
                        .map(|cf| match cf {
                            CharacterizationFactors::Ar5 => {
                                IpccCharacterizationFactorsSource::from("AR5".to_string())
                            }
                            CharacterizationFactors::Ar6 => {
                                IpccCharacterizationFactorsSource::from("AR6".to_string())
                            }
                        })
                        .collect();

                    let characterization_factors = if cf
                        .contains(&IpccCharacterizationFactorsSource::from("AR5".to_string()))
                    {
                        CharacterizationFactors::Ar5
                    } else {
                        CharacterizationFactors::Ar6
                    };

                    (characterization_factors, cf)
                }
            }
        };

    struct MappedFields {
        product_id_type: String,
        id: String,
        product_name_company: String,
        declared_unit: DeclaredUnit,
        unitary_product_amount: Decimal,
        p_cf_excluding_biogenic: Decimal,
    }

    let MappedFields {
        product_id_type,
        id,
        product_name_company,
        declared_unit,
        unitary_product_amount,
        p_cf_excluding_biogenic,
    } = match ileap_type {
        ILeapType::ShipmentFootprint(ref shipment) => MappedFields {
            product_id_type: "shipment".to_string(),
            id: shipment.shipment_id.clone(),
            product_name_company: format!("ShipmentFootprint with id {}", shipment.shipment_id),
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: shipment
                .tces
                .0
                .iter()
                .fold(Decimal::from(0), |acc, tce| acc + tce.co2e_wtw.0),
            p_cf_excluding_biogenic: shipment
                .tces
                .0
                .iter()
                .fold(Decimal::from(0), |acc, tce| acc + tce.co2e_wtw.0),
        },
        ILeapType::Toc(ref toc) => MappedFields {
            product_id_type: "toc".to_string(),
            id: toc.toc_id.clone(),
            product_name_company: format!("TOC with ID {}", toc.toc_id),
            declared_unit: DeclaredUnit::TonKilometer,
            unitary_product_amount: Decimal::from(1),
            p_cf_excluding_biogenic: toc.co2e_intensity_wtw.0,
        },
        ILeapType::Hoc(ref hoc) => MappedFields {
            product_id_type: "hoc".to_string(),
            id: hoc.hoc_id.clone(),
            product_name_company: format!("HOC with ID {}", hoc.hoc_id),
            declared_unit: DeclaredUnit::Kilogram,
            unitary_product_amount: Decimal::from(1000),
            p_cf_excluding_biogenic: match hoc.co2e_intensity_throughput {
                HocCo2eIntensityThroughput::TEU => {
                    get_teu_co2e_intensity_wtw(hoc.co2e_intensity_wtw.0, hoc_container_size)
                }
                HocCo2eIntensityThroughput::Tonnes => hoc.co2e_intensity_wtw.0,
            },
        },
    };

    fn get_teu_co2e_intensity_wtw(
        hoc_co2e_intensity_wtw: Decimal,
        hoc_container_size: Option<HocTeuContainerSize>,
    ) -> Decimal {
        match hoc_container_size {
            Some(HocTeuContainerSize::Normal) => hoc_co2e_intensity_wtw * Decimal::from(10000),
            Some(HocTeuContainerSize::Light) => hoc_co2e_intensity_wtw * Decimal::from(6000),
            Some(HocTeuContainerSize::Heavy) => hoc_co2e_intensity_wtw * Decimal::from(14050),
            None => {
                println!("Warning: HOC TEU container size not specified, using normal container");
                hoc_co2e_intensity_wtw * Decimal::from(10000)
            }
        }
    }

    let data_schema_id = if product_id_type == "shipment" {
        "shipment-footprint"
    } else {
        product_id_type.as_str()
    };

    ProductFootprint {
        id: PfId(Uuid::new_v4()),
        spec_version: SpecVersionString("2.2.0".to_string()),
        preceding_pf_ids: None,
        version: VersionInteger(1),
        created: Utc::now(),
        updated: None,
        status: PfStatus::Active,
        status_comment: None,
        validity_period_start: None,
        validity_period_end: None,
        company_name: company_name.to_string().into(),
        company_ids: CompanyIdSet(vec![Urn::from(company_urn.to_string())]),
        product_description: "".to_string(),
        product_ids: ProductIdSet(vec![Urn::from(format!(
            "urn:pathfinder:product:customcode:vendor-assigned:{product_id_type}:{id}"
        ))]),
        product_category_cpc: String::from("83117").into(),
        product_name_company: product_name_company.into(),
        comment: "".to_string(),
        pcf: CarbonFootprint {
            declared_unit,
            unitary_product_amount: unitary_product_amount.into(),
            p_cf_excluding_biogenic: p_cf_excluding_biogenic.into(),
            p_cf_including_biogenic: Some(p_cf_excluding_biogenic.into()), // TODO: to be clarified in the Tech Specs
            fossil_ghg_emissions: p_cf_excluding_biogenic.into(),
            fossil_carbon_content: p_cf_excluding_biogenic.into(), // TODO: to be clarified in the Tech Specs
            biogenic_carbon_content: p_cf_excluding_biogenic.into(), // TODO: to be clarified in the Tech Specs
            d_luc_ghg_emissions: None,
            land_management_ghg_emissions: None,
            other_biogenic_ghg_emissions: None,
            i_luc_ghg_emissions: None,
            biogenic_carbon_withdrawal: None,
            aircraft_ghg_emissions: None,
            characterization_factors,
            ipcc_characterization_factors_sources: characterization_factors_sources.into(),
            cross_sectoral_standards_used: CrossSectoralStandardSet(vec![
                CrossSectoralStandard::ISO14083,
            ]),
            product_or_sector_specific_rules: None, // TODO: get clarity on whether GLEC should be specified
            biogenic_accounting_methodology: None,
            boundary_processes_description: "".to_string(),
            reference_period_start: Utc::now(),
            reference_period_end: (Utc::now() + chrono::Duration::days(364)),
            geographic_scope: None,
            secondary_emission_factor_sources: None,
            exempted_emissions_percent: ExemptedEmissionsPercent(0.into()),
            exempted_emissions_description: "".to_string(),
            packaging_emissions_included: false,
            packaging_ghg_emissions: None,
            allocation_rules_description: None,
            uncertainty_assessment_description: None,
            primary_data_share: None,
            dqi: None,
            assurance: None,
        },
        extensions: Some(vec![DataModelExtension {
            spec_version: SpecVersionString::from("0.2.0".to_string()),
            data_schema: format!("https://api.ileap.sine.dev/{data_schema_id}.json"),
            documentation: Some("https://sine-fdn.github.io/ileap-extension/".to_string()),
            data: serde_json::to_value(&ileap_type)
                .unwrap()
                .as_object()
                .unwrap()
                .to_owned(),
        }]),
    }
}
