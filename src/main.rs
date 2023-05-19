use actix_web::{get, http::Method, middleware, web, App, HttpResponse, HttpServer, Responder};
use portfolio::web::AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hi welcome to my backend. It is made in rust ")
}

async fn default_service() -> impl Responder {
    HttpResponse::NotFound().body("Method not implemented")
}

/// This is a Rust function that sets up an HTTP server using Actix-web and connects to a database, with
/// routes for handling requests related to a portfolio and projects.
///
/// Returns:
///
/// The `main` function is returning a `std::io::Result<()>`, which indicates that it may return an
/// `io::Error` if there is an issue with input/output operations.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = portfolio::db::set_up_connection()
        .await
        .expect("Error connecting to DB");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .service(hello)
            .configure(portfolio::web::project_config)
            .configure(portfolio::web::testomonials_config)
            .app_data(web::Data::new(AppState {
                created_by: String::from("Nirjal Paudel"),
                db: database.clone(),
            }))
            .default_service(web::route().method(Method::GET).to(default_service))
    })
    .bind(("127.0.0.1", 8000))?
    .run();

    println!("Server is running");
    server.await
}
