use std::env;

use actix_web::{web, App, HttpServer, Responder, middleware};
use env_logger::Env;
use stores::{Store, postgres::PostgresStore};
use log::info;
mod db_schema;
mod stores;

#[derive(Clone,Copy)]
struct AppState<T: Store + Clone> {
    store: T,
}

async fn list_games<T: Store + Clone>(data: web::Data<AppState<T>>) -> impl Responder {
    let result = data.store.select_boardgames();
    web::Json(result)
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    let mut builder = env_logger::Builder::from_env(
        Env::default().default_filter_or("info")
    );
    builder
        .target(env_logger::Target::Stdout)
        .init();
    let store = PostgresStore::new();
    let state = AppState{store};
        
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/games").route("", web::get().to(list_games::<PostgresStore>))
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
