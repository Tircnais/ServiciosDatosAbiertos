mod api_service;
mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use api_service::routes::configure_routes;
use crate::models::empresa_model::Empresa;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reemplaza con tu cadena de conexión
    // dotenv::dotenv().ok();
    dotenv().ok();
    // Intentar obtener la URI completa de la base de datos de la variable de entorno DB_URI
    let client_uri = match env::var("DB_URL") {
        Ok(uri) => uri, // Usar DB_URL si está definida
        Err(_) => {
            // Si DB_URL no está definida, construir la URI usando DB_HOST y DB_PORT
            let host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = env::var("DB_PORT").unwrap_or_else(|_| "27017".to_string());
            // let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "DatosAbiertosEcuador".to_string());
            format!("mongodb://{}:{}", host, port)
        }
    };

    // Añadimos esta línea para depurar
    println!("DEBUG: MongoDB client_uri is: {}", client_uri);

    println!("Conectando a la base de datos en: {}", client_uri); // Imprimir la URI que se está usando
    let client_options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(client_options)?;

    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "DatosAbiertosEcuador".to_string());
    let name_collection = env::var("USER_COLLECTION_NAME").unwrap_or_else(|_| "Empresas".to_string());
    let db = client.database(&db_name);
    let collection: mongodb::Collection<Empresa> = db.collection(&name_collection);

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
