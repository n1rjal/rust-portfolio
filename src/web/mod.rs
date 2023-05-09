use super::model::Project;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::Database;

pub struct AppState {
    pub created_by: String,
    pub db: Database,
}

async fn get_portfolio() -> impl Responder {
    HttpResponse::Ok().body("Portfolio Page")
}

#[get("/projects/{id}")]
async fn handle_projects(
    project_id_from_path: web::Path<u32>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let project = Project::find_projects(&app_state.db).await.expect("Errr");
    HttpResponse::Ok().json(project)
}

pub fn project_config(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_projects);
}

pub fn portfolio_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/portfolio").route(web::get().to(get_portfolio)));
}
