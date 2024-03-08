use rocket::serde::{Serialize, Deserialize, Debug};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    email: String,
    firstName: String,
    lastName: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    id : i32,
    jobTitle: String,
    jobDescription: String,
    company: String,
    source: String,
    dateApplied: String,
    jobType: String,
    status: String,
    jobClosed: bool,
    userId: i32
}