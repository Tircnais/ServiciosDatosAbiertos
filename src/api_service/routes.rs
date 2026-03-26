use actix_web::web;
use crate::handlers::empresa_handler::{ 
    get_empresas,
    get_empresa_by_ruc,
    get_empresa_by_razon_social,
    search_empresas_by_razon_social, // AGREGADO
    create_empresa,
    update_empresa_by_ruc,
    delete_empresa_por_ruc
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/empresas", web::get().to(get_empresas))
            .route("/empresa_ruc/{ruc}", web::get().to(get_empresa_by_ruc))
            // Consulta Exacta. Retorna. Solo si el nombre es idéntico
            .route("/empresa_razon_social/{razon_social}", web::get().to(get_empresa_by_razon_social))
            // Consulta Parcial. Retorna empresa con "nombres similares"
            .route("/empresa_search/{razon_social}", web::get().to(search_empresas_by_razon_social))
            .route("/empresa", web::post().to(create_empresa))
            .route("/empresa_update/{ruc}", web::put().to(update_empresa_by_ruc))
            .route("/empresa_delete/{ruc}", web::delete().to(delete_empresa_por_ruc)),
    );
}
