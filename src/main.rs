use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use dotenv::dotenv;
use goodmorning_services::{functions::*, structs::StorageLimits, *};
use std::{env, time::Duration};

#[tokio::main]
async fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let port = env::var("PORT")
        .expect("cannot find `PORT` in env")
        .parse::<u16>()
        .expect("cannot parse port to u16");
    let ip = env::var("IP").expect("cannot find `IP` in env");
    let db = get_prod_database(&get_client().await);

    let storage_limits = StorageLimits {
        _1: env::var("STORAGE_LIMIT_1")
            .expect("cannot find `STORAGE_LIMIT_1` in env")
            .parse()
            .expect("cannot parse STORAGE_LIMIT_1 to u64"),
    };

    println!("Starting server at {ip}:{port}");

    HttpServer::new(move || {
        let backend = InMemoryBackend::builder().build();
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 5)
            .real_ip_key()
            .build();
        let middleware = RateLimiter::builder(backend, input).add_headers().build();
        App::new()
            .service(api::scope())
            .route("/", web::get().to(pong))
            .wrap(Logger::default())
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(EMAIL_VERIFICATION_DURATION))
            .app_data(Data::new(storage_limits))
            .wrap(middleware)
    })
    .bind((ip, port))
    .expect("cannot bind to port")
    .run()
    .await
    .expect("server down");
}

async fn pong() -> &'static str {
    "Pong!"
}
