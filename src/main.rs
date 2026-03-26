mod api_service;
mod models;
mod handlers;
mod db; // AGREGADO: antes no se usaba este módulo

use actix_web::{web, App, HttpServer};
use api_service::routes::configure_routes;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reemplaza con tu cadena de conexión
    // dotenv::dotenv().ok();
    dotenv().ok();
    // CAMBIADO: antes toda esta lógica estaba aquí duplicando db.rs
    // Ahora usamos la función centralizada de db.rs
    let collection = db::get_empresa_collection().await;

    // AGREGADO: el host y puerto ahora vienen del .env
    let app_host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let app_port = env::var("DB_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", app_host, app_port);

    println!("Servidor escuchando en: {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(collection.clone()))
            .configure(configure_routes)
    })
    .bind(&bind_address)? // CAMBIADO: antes era "127.0.0.1:8080" fijo
    .run()
    .await?;

    Ok(())
}
