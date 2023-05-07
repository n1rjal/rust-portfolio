pub mod db;
pub mod model;
use actix_web::{App, HttpServer};

async fn set_up() {
    db::set_up_connection()
        .await
        .expect("Something went wrong with db connection");
    
}
