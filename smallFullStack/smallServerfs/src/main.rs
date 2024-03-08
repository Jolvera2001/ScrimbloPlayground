#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/dashboard/<name>")]
fn dashboard(name: &str) -> String {
    format!("Welcome to your dashboard, {}!", name)
}

#[get("/applications")]
async fn get_applications() -> Json<Vec<Application>> {
    let db = db as &DatabaseConnection;
    
    let applications = Applications::find()
        .all(db)
        .await
        .expect("Error getting applications");

    Json(applications)
}

#[launch]
fn rocket() -> _ {
    let db = set_up_db().await {
        ok(db) => db,
        Err(err) => panic!("Error connecting to db: {}", err)
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![dashboard, index, get_applications])
}