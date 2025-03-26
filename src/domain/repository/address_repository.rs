use crate::domain::{
    errors::DbError,
    models::{
        internal_address_model::InternalAddressModel,
        internal_address_to_create_model::InternalAddressToCreateModel,
        internal_address_to_update_model::InternalAddressToUpdateModel,
    },
};

/// Repository trait for managing address data persistence
pub trait AddressRepository {
    /// Creates a new address record
    /// Returns the created address model on success, or a database error on failure
    fn save(
        &self,
        address: InternalAddressToCreateModel,
    ) -> impl std::future::Future<Output = Result<InternalAddressModel, DbError>> + Send;

    /// Updates an existing address record by ID
    /// Returns Ok(()) on success, or a database error on failure
    fn update(
        &self,
        id: u32,
        address: InternalAddressToUpdateModel,
    ) -> impl std::future::Future<Output = Result<(), DbError>> + Send;

    /// Deletes an address record by ID
    /// Returns Ok(()) on success, or a database error on failure
    fn delete(&self, id: u32) -> impl std::future::Future<Output = Result<(), DbError>> + Send;

    /// Retrieves an address record by ID
    /// Returns Some(address) if found, None if not found, or a database error on failure
    fn find_by_id(
        &self,
        id: u32,
    ) -> impl std::future::Future<Output = Result<Option<InternalAddressModel>, DbError>> + Send;
}
