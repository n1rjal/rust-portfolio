use dotenv::dotenv;
use mongodb::{error::Result, options::ClientOptions, Client, Database};
use std::env::var;

pub async fn set_up_connection() -> Result<Database> {
    dotenv().ok().expect("Some error occurred in dot env");
    let uri = var("MONGODB_URI").expect("MONGODB_URI must be provided");
    let db = var("MONGO_DATABASE").expect("MONGO_DATABASE must be provided");

    let client_options = ClientOptions::parse(uri).await?;

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    Ok(client.database(&db))
}
