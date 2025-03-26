use std::collections::HashMap;

use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::{
    domain::{
        errors::DbError,
        models::{
            internal_address_model::InternalAddressModel,
            internal_address_to_create_model::InternalAddressToCreateModel,
            internal_address_to_update_model::InternalAddressToUpdateModel,
        },
        repository::address_repository::AddressRepository,
    },
    infrastructure::dao::{
        adapters::internal_address_entity_adapter::InternalAddressEntityAdapter,
        entity::internal_address_entity::InternalAddressEntity,
    },
};

pub struct FileRepository {
    file_path: String,
}

impl FileRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    /// Load all addresses from the JSONL file
    // async fn load_addresses(&self) -> Result<HashMap<String, InternalAddressModel>, DbError> {
    //     if !Path::new(&self.file_path).exists() {
    //         return Ok(HashMap::new());
    //     }

    //     let data = fs::read_to_string(&self.file_path).await.map_err(|e| {
    //         DbError::IoError(format!("Failed to read file: {:?}", self.file_path), e)
    //     })?;

    //     let mut addresses = HashMap::new();
    //     for line in data.lines() {
    //         let address: InternalAddressModel = serde_json::from_str(line)
    //             .map_err(|e| DbError::SerdeError("Failed to parse address line".to_string(), e))?;
    //         addresses.insert(address.id.clone(), address);
    //     }

    //     Ok(addresses)
    // }

    /// Append a new address to the JSONL file
    async fn append_address(
        &self,
        entity: &InternalAddressEntity,
    ) -> Result<InternalAddressEntity, DbError> {
        let json_line = serde_json::to_string(entity)?;

        // Ensure parent directories exist
        if let Some(parent) = std::path::Path::new(&self.file_path).parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                DbError::IoError(format!("Failed to create directory: {:?}", parent), e)
            })?;
        }

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await
            .map_err(|e| {
                DbError::IoError(format!("Failed to open file: {:?}", self.file_path), e)
            })?;

        file.write(json_line.as_bytes()).await.map_err(|e| {
            DbError::IoError(format!("Failed to write to file: {:?}", self.file_path), e)
        })?;
        file.write_all(b"\n").await.map_err(|e| {
            DbError::IoError(
                format!("Failed to write newline to file: {:?}", self.file_path),
                e,
            )
        })?;

        Ok(entity.clone())
    }

    async fn rewrite_addresses(
        &self,
        addresses: HashMap<String, InternalAddressModel>,
    ) -> Result<(), DbError> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_path)
            .await
            .map_err(|e| {
                DbError::IoError(format!("Failed to open file: {:?}", self.file_path), e)
            })?;

        for address in addresses.values() {
            let json_line = serde_json::to_string(address)?;
            file.write_all(json_line.as_bytes()).await.map_err(|e| {
                DbError::IoError(format!("Failed to write to file: {:?}", self.file_path), e)
            })?;
            file.write_all(b"\n").await.map_err(|e| {
                DbError::IoError(
                    format!("Failed to write newline to file: {:?}", self.file_path),
                    e,
                )
            })?;
        }

        Ok(())
    }
}

impl AddressRepository for FileRepository {
    async fn save(
        &self,
        address_to_create: InternalAddressToCreateModel,
    ) -> Result<InternalAddressModel, DbError> {
        let entity = InternalAddressEntityAdapter::create_to_entity(address_to_create);
        let result = self.append_address(&entity).await?;

        Ok(InternalAddressEntityAdapter::entity_to_model(result))
    }

    async fn update(
        &self,
        id: u32,
        address_to_update: InternalAddressToUpdateModel,
    ) -> Result<(), DbError> {
        Ok(())
    }

    async fn delete(&self, id: u32) -> Result<(), DbError> {
        Ok(())
    }

    async fn find_by_id(&self, id: u32) -> Result<Option<InternalAddressModel>, DbError> {
        Ok(None)
    }
}
