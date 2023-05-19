use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Project {
    _id: ObjectId,
    name: String,

    #[serde(rename = "desp")]
    description: String,

    #[serde(rename = "techs")]
    technologies: Option<Vec<String>>,

    #[serde(rename = "githubURL")]
    github_url: Option<String>,

    #[serde(rename = "liveURL")]
    live_url: Option<String>,

    images: Vec<String>,
}

impl Project {
    pub async fn find_projects(db: &Database) -> Result<Vec<Project>, mongodb::error::Error> {
        let collection = db.collection::<Project>("projects");
        let mut cursor = collection.find(None, None).await?;
        let mut project_vec: Vec<Project> = vec![];

        while cursor.advance().await? {
            let deserialized_project = cursor.deserialize_current()?;
            project_vec.push(deserialized_project);
        }

        Ok(project_vec)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Testomonial {
    _id: ObjectId,
    author: String,
    body: String,
}

impl Testomonial {
    pub async fn find_testonmonials(
        db: &Database,
    ) -> Result<Vec<Testomonial>, mongodb::error::Error> {
        let collection = db.collection::<Testomonial>("testomonials");
        let mut cursor = collection.find(None, None).await?;
        let mut testonmonial_vec: Vec<Testomonial> = vec![];

        while cursor.advance().await? {
            let deserialized_testomonial = cursor.deserialize_current()?;
            testonmonial_vec.push(deserialized_testomonial);
        }

        Ok(testonmonial_vec)
    }
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
