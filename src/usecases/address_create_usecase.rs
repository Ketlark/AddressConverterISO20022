use crate::domain::{
    errors::DbError,
    models::{
        internal_address_model::InternalAddressModel,
        internal_address_to_create_model::InternalAddressToCreateModel,
    },
    repository::address_repository::AddressRepository,
};

pub struct AddressCreateUsecase<R: AddressRepository> {
    pub repository: R,
}

impl<R: AddressRepository> AddressCreateUsecase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        address_to_create: InternalAddressToCreateModel,
    ) -> Result<InternalAddressModel, DbError> {
        return self.repository.save(address_to_create).await;
    }
}
