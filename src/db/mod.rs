use dotenv::dotenv;
use mongodb::{error::Result, options::ClientOptions, Client};
use std::env::var;

static mut DB: Option<Client> = None;

pub async fn set_up_connection() -> Result<()> {
    dotenv().ok().expect("Some error occurred in dot env");
    let uri = var("MONGODB_URI").expect("MONGODB_URI must be provided");

    let client_options = ClientOptions::parse(uri).await?;

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    unsafe {
        DB = Some(client);
    }

    Ok(())
}
