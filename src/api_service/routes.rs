use actix_web::web;
use crate::handlers::empresa_handler::{ 
    get_empresas,
    get_empresa_by_ruc,
    create_empresa,
    update_empresa_by_ruc,
    delete_empresa_por_ruc
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/empresas", web::get().to(get_empresas))
            .route("/empresa/{ruc}", web::get().to(get_empresa_by_ruc))
            .route("/empresa", web::post().to(create_empresa))
            .route("/empresa/{ruc}", web::put().to(update_empresa_by_ruc))
            .route("/empresa/{ruc}", web::delete().to(delete_empresa_por_ruc)),
    );
}
