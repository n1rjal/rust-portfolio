use crate::model::Testomonial;

use super::model::Project;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::Database;

pub struct AppState {
    pub created_by: String,
    pub db: Database,
}

#[get("/projects")]
async fn handle_projects(app_state: web::Data<AppState>) -> impl Responder {
    let project = Project::find_projects(&app_state.db).await;
    match project {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(error) => panic!("Error occured {:?}", error),
    }
}

#[get("/testomonials")]
async fn handle_testonomials(app_state: web::Data<AppState>) -> impl Responder {
    let testomonials = Testomonial::find_testonmonials(&app_state.db).await;

    match testomonials {
        Ok(testomonials) => HttpResponse::Ok().json(testomonials),
        Err(error) => panic!("Error occured {:?}", error),
    }
}

pub fn project_config(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_projects);
}

pub fn testomonials_config(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_testonomials);
}
