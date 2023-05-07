use mongodb::bson::oid::ObjectId;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    _id: ObjectId,
    name: String,
    description: String,
    technologies: Vec<String>,
    github_url: String,
    live_url: String,
    images: Vec<String>,
}

pub struct Testomonial {
    _id: ObjectId,
    by: String,
    body: String,
}

pub struct SocialMedia {
    _id: ObjectId,
    name: String,
    image: String,
    url: String,
}

pub struct ContactMe {
    name: String,
    link: String,
}
