use actix_web::{get, middleware::Logger, web::Data, App, HttpServer, Scope};
use goodmorning_services::{structs::Jobs, *};

#[tokio::main]
async fn main() {
    init().await;
    logs_init(&LogOptions {
        loglabel: "services".to_string(),
        termlogging: true,
        writelogging: true,
        term_log_level: LevelFilterSerde::Debug,
        write_log_level: LevelFilterSerde::Debug,
    });
    let jobs: Data<Jobs> = Data::new(Jobs::default());

    HttpServer::new(move || {
        // let backend = InMemoryBackend::builder().build();
        // let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 5)
        //     .real_ip_key()
        //     .build();
        // let middleware = RateLimiter::builder(backend, input).add_headers().build();

        App::new()
            .app_data(jobs.clone())
            .service(api::scope())
            .service(Scope::new("/static/services").service(r#static))
            .service(pong)
            .service(pages::scope())
            .wrap(if *FORWARDED.get().unwrap() {
                Logger::new(r#"%{Forwarded}i "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#)
            } else {
                Logger::default()
            })
        // .app_data(Data::new(storage_limits))
        // .wrap(middleware)
    })
    .bind(("0.0.0.0", *HTTP_PORT.get().unwrap()))
    .expect("cannot bind to port")
    .run()
    .await
    .expect("server down");
}

#[get("/")]
async fn pong() -> &'static str {
    "Pong!"
}
