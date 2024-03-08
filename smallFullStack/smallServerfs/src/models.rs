use rocket::serde::{Serialize, Deserialize, Debug};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    userId: i32,
    username: String,
    password: String,
    email: String,
    firstName: String,
    lastName: String,
    dateCreated: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Application {
    jobId : i32,
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