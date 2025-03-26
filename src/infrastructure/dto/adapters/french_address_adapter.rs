use crate::domain::{
    interfaces::dto::french_address_dto::{FrenchAddressDtoRequest, FrenchAddressDtoResponse},
    models::{
        internal_address_model::InternalAddressModel,
        internal_address_to_create_model::InternalAddressToCreateModel,
    },
};

pub struct FrenchAddressDtoAdapter;

impl FrenchAddressDtoAdapter {
    pub fn to_iso_20022(address: FrenchAddressDtoRequest) -> InternalAddressToCreateModel {
        let postal_code_and_city = address
            .postal_code_and_city
            .split_whitespace()
            .collect::<Vec<&str>>();

        InternalAddressToCreateModel {
            department: None,
            sub_department: None,
            street_name: address.street,
            building_number: None,
            building_name: None,
            floor: None,
            post_box: None,
            room: None,
            post_code: postal_code_and_city[0].to_string(),
            town_name: postal_code_and_city[1].to_string(),
            town_location_name: None,
            district_name: None,
            country_subdivision: None,
            country: address.country,
        }
    }

    pub fn from_iso_20022(address: InternalAddressModel) -> FrenchAddressDtoResponse {
        let postal_code_and_city = format!("{} {}", address.post_code, address.town_name);

        FrenchAddressDtoResponse {
            street: address.street_name,
            postal_code_and_city,
            country: address.country,
        }
    }
}
