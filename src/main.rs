use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use portfolio::web::AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[allow(dead_code)]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Server is listening");
    let database = portfolio::db::set_up_connection()
        .await
        .expect("Error connecting to DB");
    HttpServer::new(move || {
        App::new()
            .configure(portfolio::web::portfolio_config)
            .configure(portfolio::web::project_config)
            .app_data(web::Data::new(AppState {
                created_by: String::from("Nirjal Paudel"),
                db: database.clone(),
            }))
            .service(hello)
            .service(web::scope("/hello").route("", web::get().to(manual_hello)))
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
