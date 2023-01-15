use actix_web::{web, App, HttpResponse, HttpServer};
use cxx::let_cxx_string;
use serde::Deserialize;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("coordinates-webserver/src/wrapper.h");
        type RWMol;
        fn smiles_to_mol(smiles: &CxxString) -> SharedPtr<RWMol>;
        fn generate_conformers(mol: &SharedPtr<RWMol>);
        fn mol_to_sdf(mol: &SharedPtr<RWMol>) -> String;
    }
}

#[derive(Deserialize)]
struct CoordinateRequest {
    smiles: String,
}

async fn coordinate_generation(coordinate_request: web::Json<CoordinateRequest>) -> HttpResponse {
    let_cxx_string!(smiles = &coordinate_request.smiles);
    let mol = ffi::smiles_to_mol(&smiles);
    ffi::generate_conformers(&mol);
    HttpResponse::Ok()
        .content_type("application/sdf")
        .body(ffi::mol_to_sdf(&mol))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default().limit(4096);

        App::new().service(
            web::resource("/")
                .app_data(json_config)
                .route(web::post().to(coordinate_generation)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
