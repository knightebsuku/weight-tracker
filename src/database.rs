use dotenv;
use postgres::{Client, NoTls, error};
use std::env;

pub fn get_client() -> Result<Client, error::Error>{
    dotenv::dotenv().ok();
    let db = env::var("DATABASE").unwrap();

    let client = Client::connect(&db, NoTls).unwrap();
    Ok(client)


}