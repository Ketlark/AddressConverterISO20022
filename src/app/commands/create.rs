use std::io::Error;

use dialoguer::{theme::ColorfulTheme, Input};

use crate::{
    domain::interfaces::dto::french_address_dto::FrenchAddressDtoRequest,
    infrastructure::{
        dto::adapters::french_address_adapter::FrenchAddressDtoAdapter,
        factories::address_create_usecase_factory::AddressCreateUsecaseFactory,
    },
};

pub struct CreateCommand;

impl CreateCommand {
    pub async fn execute() -> Result<(), Error> {
        let theme = ColorfulTheme::default();

        // Interactive prompts
        let street: String = Input::with_theme(&theme)
            .with_prompt("Enter street address")
            .interact_text()?;

        let postal_code_and_city: String = Input::with_theme(&theme)
            .with_prompt("Enter postal code and city (e.g., '75001 Paris')")
            .interact_text()?;

        let country: String = Input::with_theme(&theme)
            .with_prompt("Enter country")
            .interact_text()?;

        // Build DTO from user input
        let french_dto = FrenchAddressDtoRequest {
            street,
            postal_code_and_city,
            country,
        };

        // Convert to ISO 20022 format
        let new_address = FrenchAddressDtoAdapter::to_iso_20022(french_dto);

        // Execute use case
        let usecase = AddressCreateUsecaseFactory::create_usecase();
        let saved_address = usecase.execute(new_address).await;

        println!("\n✅ Address saved successfully:\n{:?}", saved_address);
        Ok(())
    }
}
