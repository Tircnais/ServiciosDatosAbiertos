use actix_web::{web, HttpResponse, Responder};
use mongodb::{Collection, bson::{doc, Bson}};
// use futures::{stream::TryStreamExt, StreamExt};
use futures::stream::StreamExt;
use serde::Deserialize; // Importar Deserialize para los parámetros de consulta
use crate::models::empresa_model::Empresa;
use std::cmp::max; // Importar max para asegurar que la página sea al menos 1


// Estructura para los parámetros de paginación
#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>, // Número de página (opcional)
    pub limit: Option<i64>, // Límite de documentos por página (opcional)
}


/// Handler que responde a GET /empresas
pub async fn get_empresas(collection: web::Data<Collection<Empresa>>, params: web::Query<PaginationParams>,) -> impl Responder {
    // Obtener página y límite, con valores por defecto si no se proporcionan
    let page = max(params.page.unwrap_or(1), 1); // Asegurar que la página sea al menos 1
    let limit = params.limit.unwrap_or(100); // Límite por defecto, puedes ajustarlo

    // Calcular cuántos documentos saltar
    let skip = (page - 1) * limit as u64;

    // 1. Iniciar la operación find. Esto devuelve un objeto Find.
    let mut find_operation = collection.find(doc! {}); // find con un argumento (filtro)

    // 2. Aplicar skip y limit al objeto Find
    find_operation = find_operation.skip(skip);
    find_operation = find_operation.limit(limit);

    // 3. Ejecutar la operación find llamando a .await para obtener el Result<Cursor, Error>
    let mut cursor = match find_operation.await {
        Ok(cursor) => cursor,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Error al consultar: {}", e)),
    };

    let mut empresas = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(empresa)=> empresas.push(empresa),
            Err(e)=> return HttpResponse::InternalServerError().body(format!("Error al iterar: {}", e)),
        }
    }
    HttpResponse::Ok().json(empresas)
}


/// Obtener una empresa por su RUC
pub async fn get_empresa_by_ruc(collection: web::Data<Collection<Empresa>>, ruc: web::Path<String>) -> impl Responder {
    let ruc_value = ruc.into_inner();
    // println!("DEBUG: RUC recibido en handler: '{}'", ruc_value); // Línea de depuración opcional

    // Asegúrate de que el nombre del campo "numero_ruc" coincida exactamente con tu DB
    let filter = doc! { "NUMERO_RUC": ruc_value.clone() };
    // let filter = doc! { "NUMERO_RUC": ruc.into_inner() };
    match collection.find_one(filter).await {
        Ok(Some(empresa)) => HttpResponse::Ok().json(empresa),
        Ok(None) => HttpResponse::NotFound().body("RUC no encontrado"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// Obtener una empresa por su RUC
pub async fn get_empresa_by_razon_social(collection: web::Data<Collection<Empresa>>, razon_social: web::Path<String>) -> impl Responder {
    let razon_social_value = razon_social.into_inner();
    // Convertir la razón social a mayúsculas
    let razon_social_uppercase = razon_social_value.to_uppercase();
    // println!("DEBUG: Razón Social recibida: '{}', en mayúsculas: '{}'", razon_social_value, razon_social_uppercase); // Línea de depuración opcional

    // Asegúrate de que el nombre del campo "razon_social" coincida exactamente con tu DB
    // y que los valores en la DB estén en mayúsculas o uses una consulta insensible a mayúsculas/minúsculas
    let filter = doc! { "RAZON_SOCIAL": razon_social_uppercase };

    // Esto a menudo requiere índices de texto o expresiones regulares, que pueden afectar el rendimiento.
    // Una alternativa simple (pero menos eficiente sin índice adecuado) sería:
    // let filter = doc! { "RAZON_SOCIAL": { "$regex": format!("^{}$", regex::escape(&razon_social_uppercase)), "$options": "i" } };
    // Para usar regex, necesitarías la crate `regex` y posiblemente `bson::Regex`.
    match collection.find_one(filter).await {
        Ok(Some(empresa)) => HttpResponse::Ok().json(empresa),
        Ok(None) => HttpResponse::NotFound().body("RAZON_SOCIAL no encontrado"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


/// Crear una nueva empresa
pub async fn create_empresa(collection: web::Data<Collection<Empresa>>, nueva_empresa: web::Json<Empresa>) -> impl Responder {
    match collection.insert_one(nueva_empresa.into_inner()).await {
        Ok(insert_result) => HttpResponse::Created().json(insert_result.inserted_id),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


/// Actualizar una empresa
pub async fn update_empresa_by_ruc(collection: web::Data<Collection<Empresa>>, path: web::Path<String>, empresa_actualizada: web::Json<Empresa>) -> impl Responder {
    let ruc = path.into_inner();
    let filter = doc! { "NUMERO_RUC": &ruc };
    let update = doc! {
        "$set": {
            "NUMERO_RUC": &empresa_actualizada.numero_ruc,
            "RAZON_SOCIAL": &empresa_actualizada.razon_social,
            "PROVINCIA_JURISDICION": &empresa_actualizada.provincia_jurisdiccion,
            "ESTADO_CONTRIBUYENTE": &empresa_actualizada.estado_contribuyente,
            "CLASE_CONTRIBUYENTE": &empresa_actualizada.clase_contribuyente,
            "FECHA_INICIO_ACTIVIDADES": empresa_actualizada.fecha_inicio_actividades.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "FECHA_ACTUALIZACION": empresa_actualizada.fecha_actualizacion.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "FECHA_SUSPENSION_DEFINITIVA": empresa_actualizada.fecha_suspension_definitiva.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "FECHA_REINICIO_ACTIVIDADES": empresa_actualizada.fecha_reinicio_actividades.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "OBLIGADO": &empresa_actualizada.obligado,
            "TIPO_CONTRIBUYENTE": &empresa_actualizada.tipo_contribuyente,
            "NUMERO_ESTABLECIMIENTO": &empresa_actualizada.numero_establecimiento,
            "NOMBRE_FANTASIA_COMERCIAL": &empresa_actualizada.nombre_fantasia_comercial,
            "ESTADO_ESTABLECIMIENTO": &empresa_actualizada.estado_establecimiento,
            "DESCRIPCION_PROVINCIA_EST": &empresa_actualizada.descripcion_provincia_est,
            "DESCRIPCION_CANTON_EST": &empresa_actualizada.descripcion_canton_est,
            "DESCPRIPCION_PARROQUIA_EST": &empresa_actualizada.descripcion_parroquia_est,
            "CODIGO_CIIU": &empresa_actualizada.codigo_ciiu,
            "ACTIVIDAD_ECONOMICA": &empresa_actualizada.actividad_economica,
            "AGENTE_RETENCION": &empresa_actualizada.agente_retencion,
            "ESPECIAL": &empresa_actualizada.especial,
        }
    };
    match collection.update_one(filter, update).await {
        Ok(result) => {
            if result.modified_count > 0 {
                HttpResponse::Ok().body("Empresa actualizada")
            } else {
                HttpResponse::NotFound().body("Empresa no encontrada")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}



/// Eliminar una empresa
pub async fn delete_empresa_por_ruc(collection: web::Data<Collection<Empresa>>, ruc: web::Path<String>) -> impl Responder {
    let ruc = ruc.into_inner();
    let filter = doc! { "NUMERO_RUC": &ruc };
    match collection.delete_one(filter).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                HttpResponse::Ok().body("Empresa eliminada")
            } else {
                HttpResponse::NotFound().body("RUC no encontrado")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
