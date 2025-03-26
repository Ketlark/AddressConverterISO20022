use uuid::Uuid;

use crate::{
    domain::models::{
        internal_address_model::InternalAddressModel,
        internal_address_to_create_model::InternalAddressToCreateModel,
    },
    infrastructure::dao::entity::internal_address_entity::InternalAddressEntity,
};

pub struct InternalAddressEntityAdapter {}

impl InternalAddressEntityAdapter {
    pub fn create_to_entity(model: InternalAddressToCreateModel) -> InternalAddressEntity {
        InternalAddressEntity {
            id: Uuid::new_v4().to_string(),
            department: model.department,
            sub_department: model.sub_department,
            floor: model.floor,
            room: model.room,
            building_name: model.building_name,
            street_name: model.street_name,
            building_number: model.building_number,
            post_box: model.post_box,
            town_location_name: model.town_location_name,
            post_code: model.post_code,
            town_name: model.town_name,
            district_name: model.district_name,
            country_subdivision: model.country_subdivision,
            country: model.country,
        }
    }

    pub fn entity_to_model(entity: InternalAddressEntity) -> InternalAddressModel {
        InternalAddressModel {
            id: entity.id,
            department: entity.department,
            sub_department: entity.sub_department,
            floor: entity.floor,
            room: entity.room,
            building_name: entity.building_name,
            street_name: entity.street_name,
            building_number: entity.building_number,
            post_box: entity.post_box,
            town_location_name: entity.town_location_name,
            post_code: entity.post_code,
            town_name: entity.town_name,
            district_name: entity.district_name,
            country_subdivision: entity.country_subdivision,
            country: entity.country,
        }
    }
}
