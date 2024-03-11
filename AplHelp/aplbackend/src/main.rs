use actix_web::{get, post, patch, App, HttpResponse, HttpServer, Responder, web::Json, web::Path, web::Data};
use validator::Validate;
use uuid;

mod models;
mod db;

use crate::db::Database;
use crate::models::{ BuyPizzaRequest, UpdatePizzaURL, Pizza };

#[get("/pizzas")]
async fn get_pizzas(db: Data<Database>) -> impl Responder {
    let pizzas = db.get_all_pizzas().await;
    match pizzas {
        Some(found_pizzas) => HttpResponse::Ok().body(format!("{:?}", found_pizzas)),
        None => HttpResponse::Ok().body("No pizzas found"),
    }
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>, db: Data<Database>) -> impl Responder {
    // validation
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizza = db.add_pizza(Pizza::new(
                String::from(new_uuid),
                pizza_name
            )).await;

            match new_pizza {
                Some(pizza) => return HttpResponse::Ok().body(format!("Created new Pizza {:?}", pizza)),
                None => return HttpResponse::Ok().body(format!("Error buying pizza"))
            }
        }
        Err(_e) => HttpResponse::Ok().body("Pizza name required"),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating pizza with {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = Database::init()
        .await
        .expect("Failed to connect to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
