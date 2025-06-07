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
    let filter = doc! { "numero_ruc": ruc.into_inner() };
    match collection.find_one(filter).await {
        Ok(Some(empresa)) => HttpResponse::Ok().json(empresa),
        Ok(None) => HttpResponse::NotFound().body("RUC no encontrado"),
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
    let filter = doc! { "numero_ruc": &ruc };
    let update = doc! {
        "$set": {
            "numero_ruc": &empresa_actualizada.numero_ruc,
            "razon_social": &empresa_actualizada.razon_social,
            "provincia_jurisdiccion": &empresa_actualizada.provincia_jurisdiccion,
            "estado_contribuyente": &empresa_actualizada.estado_contribuyente,
            "clase_contribuyente": &empresa_actualizada.clase_contribuyente,
            "fecha_inicio_actividades": empresa_actualizada.fecha_inicio_actividades.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "fecha_actualizacion": empresa_actualizada.fecha_actualizacion.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "fecha_suspension_definitiva": empresa_actualizada.fecha_suspension_definitiva.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "fecha_reinicio_actividades": empresa_actualizada.fecha_reinicio_actividades.as_ref().map(|f| {
                Bson::DateTime(bson::DateTime::from_millis(f.timestamp_millis()))
            }),
            "obligado": &empresa_actualizada.obligado,
            "tipo_contribuyente": &empresa_actualizada.tipo_contribuyente,
            "numero_establecimiento": &empresa_actualizada.numero_establecimiento,
            "nombre_fantasia_comercial": &empresa_actualizada.nombre_fantasia_comercial,
            "estado_establecimiento": &empresa_actualizada.estado_establecimiento,
            "descripcion_provincia_est": &empresa_actualizada.descripcion_provincia_est,
            "descripcion_canton_est": &empresa_actualizada.descripcion_canton_est,
            "descripcion_parroquia_est": &empresa_actualizada.descripcion_parroquia_est,
            "codigo_ciiu": &empresa_actualizada.codigo_ciiu,
            "actividad_economica": &empresa_actualizada.actividad_economica,
            "agente_retencion": &empresa_actualizada.agente_retencion,
            "especial": &empresa_actualizada.especial,
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
    let filter = doc! { "numero_ruc": &ruc };
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
