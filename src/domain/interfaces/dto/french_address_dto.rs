use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct FrenchAddressDtoRequest {
    pub street: String,
    pub postal_code_and_city: String,
    pub country: String,
}

pub struct FrenchAddressDtoResponse {
    pub street: String,
    pub postal_code_and_city: String,
    pub country: String,
}
