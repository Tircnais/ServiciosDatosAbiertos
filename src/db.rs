// use mongodb::{options::{ClientOptions, Credential }, Client, Collection };
use mongodb::{options::ClientOptions, Client, Collection };
use std::env;
use crate::models::empresa_model::Empresa;

pub async fn get_mongo_client() -> Client {
    dotenv::dotenv().ok();
    // let username = env::var("DB_USERNAME").expect("DB_USERNAME debe ser asignado");
    // let password = env::var("DB_PASSWORD").expect("DB_PASSWORD debe ser asignado");
    let host = env::var("DB_URL").unwrap_or_else(|_| "mongodb://127.0.0.1".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "27017".to_string());
    
    let client_url = format!("mongodb://{}:{}", host, port);
    let client_uri = env::var("DB_URL").unwrap_or_else(|_| client_url);
    /*
    let credential = Credential::builder()
        .username(Some(username))
        .password(Some(password))
        .source(Some(auth_source))
        .build();
    */
    
    let options = ClientOptions::parse(client_uri).await.unwrap();
    // options.credential = Some(credential);

    Client::with_options(options).unwrap()
}

pub async fn get_empresa_collection() -> Collection<Empresa> {
    let client = get_mongo_client().await;
    let db_name = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "DatosAbiertosEcuador".to_string());
    let colection_name = std::env::var("USER_COLLECTION_NAME").unwrap_or_else(|_| "FuenteGobEc".to_string());
    let db = client.database(&db_name);
    db.collection::<Empresa>(&colection_name)
}
