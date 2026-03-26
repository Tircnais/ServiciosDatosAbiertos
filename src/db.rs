// use mongodb::{options::{ClientOptions, Credential }, Client, Collection };
use mongodb::{options::ClientOptions, Client, Collection };
use std::env;
use crate::models::empresa_model::Empresa;

pub async fn get_mongo_client() -> Result<Client, mongodb::error::Error> {
    // CAMBIADO: antes tenías lógica confusa donde DB_URL se usaba dos veces
    // Ahora es simple: si existe DB_URL la usas, si no construyes la URI
    let client_uri = match env::var("DB_URL") {
        Ok(url) => url,
        Err(_) => {
            let mdbhost = env::var("MDB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
            let mdbport = env::var("MDB_PORT").unwrap_or_else(|_| "27017".to_string());
            format!("mongodb://{}:{}", mdbhost, mdbport)
        }
    };

    println!("Conectando a MongoDB en: {}", client_uri);

    // CAMBIADO: antes usabas .unwrap(), ahora propagas el error con ?
    let options = ClientOptions::parse(client_uri).await?;
    Client::with_options(options)
}

pub async fn get_empresa_collection() -> Collection<Empresa> {
    // CAMBIADO: antes el error era silencioso con .unwrap()
    // Ahora si falla la conexión el mensaje es claro
    let client = get_mongo_client().await.expect(
        "Error al conectar con MongoDB. Verifica DB_URL en tu archivo .env"
    );

    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "DatosAbiertosEcuador".to_string());
    let collection_name = env::var("USER_COLLECTION_NAME").unwrap_or_else(|_| "Empresas".to_string());
    client.database(&db_name).collection::<Empresa>(&collection_name)
}
