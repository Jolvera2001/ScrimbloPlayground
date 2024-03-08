// pizza model
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message="Pizza name required"))]
    pub pizza_name: String
}