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
    let client_uri = match env::var("DB_URI") {
        Ok(uri) => uri, // Usar DB_URI si está definida
        Err(_) => {
            // Si DB_URI no está definida, construir la URI usando DB_HOST y DB_PORT
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