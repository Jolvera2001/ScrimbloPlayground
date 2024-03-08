#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/dashboard/<name>")]
fn dashboard(name: &str) -> String {
    format!("Welcome to your dashboard, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![dashboard])
        .mount("/", routes![index])
}