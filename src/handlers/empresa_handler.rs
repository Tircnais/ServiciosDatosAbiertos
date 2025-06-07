use actix_web::{web, HttpResponse, Responder};
// use futures::{stream::TryStreamExt, StreamExt};
use futures::StreamExt;
use mongodb::{Collection, bson::{doc, Bson}};
use crate::models::empresa_model::Empresa;


/// Handler que responde a GET /empresas
pub async fn get_empresas(collection: web::Data<Collection<Empresa>>) -> impl Responder {
    let cursor = collection.find(doc! {}).await;
    match cursor {
        Ok(mut cursor)=>{
            let mut empresas = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(empresa)=> empresas.push(empresa),
                    Err(e)=> return HttpResponse::InternalServerError().body(format!("Error al iterar: {}", e)),
                }
            }
            HttpResponse::Ok().json(empresas)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error al consultar: {}", e)),
    }
}


/// Obtener una empresa por su RUC
pub async fn get_empresa_by_ruc(collection: web::Data<Collection<Empresa>>, ruc: web::Path<String>) -> impl Responder {
    let filter = doc! { "NUMERO_RUC": ruc.into_inner() };
    match collection.find_one(filter).await {
        Ok(Some(empresa)) => HttpResponse::Ok().json(empresa),
        Ok(None) => HttpResponse::NotFound().body("RUC no encontrado"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

/// Obtener una empresa por su RUC
pub async fn get_empresa_by_razon_social(collection: web::Data<Collection<Empresa>>, razon_social: web::Path<String>) -> impl Responder {
    let filter = doc! { "RAZON_SOCIAL": razon_social.into_inner() };
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
