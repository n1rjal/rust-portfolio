pub mod db;
pub mod model;
pub mod web;

pub async fn set_up() {
    db::set_up_connection()
        .await
        .expect("Something went wrong with db connection");
}
