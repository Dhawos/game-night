use actix_web::{web, App, HttpServer, Responder};
use stores::{Store, postgres::PostgresStore};

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
    let store = PostgresStore::new();
    let state = AppState{store: store};
        
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/games").route("", web::get().to(list_games::<PostgresStore>))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
