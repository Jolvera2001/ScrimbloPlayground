// pizza model
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct BuyPizzaRequest {
    #[validate(length(min = 1, message="Pizza name required"))]
    pub pizza_name: String
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdatePizzaURL {
    pub uuid: String
}


// Pizza Model
#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Pizza {
    pub uuid: String,
    pub pizza_name: String
}

impl Pizza {
    pub fn new(uuid: String, pizza_name: String) -> Pizza {
        Pizza {uuid, pizza_name }
    }
}