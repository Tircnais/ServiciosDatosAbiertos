mod api_service;
mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use api_service::routes::configure_routes;
use crate::models::empresa_model::Empresa;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reemplaza con tu cadena de conexión
    dotenv::dotenv().ok();
    let host = env::var("DB_URL").unwrap_or_else(|_| "mongodb://127.0.0.1".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "27017".to_string());
    
    let client_url = format!("mongodb://{}:{}", host, port);
    let client_uri = env::var("DB_URL").unwrap_or_else(|_| client_url);

    println!("Conectando a la base de datos...");
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("DatosAbiertosEcuador");
    let collection: mongodb::Collection<Empresa> = db.collection("FuenteGobEc");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
/*
use actix_web::{web, App, HttpServer};

mod db;
mod models;
mod handlers;
mod api_service;

use db::get_empresa_collection;
use api_service::routes::configure_routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Aquí ya estás conectando a la base de datos y obteniendo la colección
    let collection = get_empresa_collection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 */