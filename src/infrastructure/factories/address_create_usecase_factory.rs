use std::env;

use crate::{
    app::config::Configuration,
    infrastructure::repository::address_filedb_repository::FileRepository,
    usecases::address_create_usecase::AddressCreateUsecase,
};
pub struct AddressCreateUsecaseFactory;

impl AddressCreateUsecaseFactory {
    pub fn create_usecase() -> AddressCreateUsecase<FileRepository> {
        let config_database_path = Configuration::get_instance()
            .get_string("database_file_path")
            .unwrap();

        let current_path = env::current_dir().unwrap();
        let database_path = current_path
            .join(config_database_path)
            .to_string_lossy()
            .to_string();

        AddressCreateUsecase {
            repository: FileRepository::new(database_path),
        }
    }
}
