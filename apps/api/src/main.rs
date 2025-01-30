use crate::repository::Repository;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, middleware::Logger};

mod response;
mod saga;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    //wrap using Mutex: consente accesso mutabile e thread-safe
    let repo = web::Data::new(Mutex::new(Repository::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(repo.clone()) //non clona il contenuto, ma solo il riferimento ... che è thread-safe
            .service(saga::list)
            .service(saga::create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

