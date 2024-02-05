use actix_web::{web, App, HttpServer};
use env_logger;

mod config;
mod routes;
mod utils;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = config::Config::new();
    let addr = format!("{}:{}", &conf.host, &conf.port);
    env_logger::init();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conf.to_owned()))
            .service(routes::index)
            .route("/gcd", web::post().to(routes::post_gcd))
            .route("/mdb", web::get().to(routes::get_mandelbrot))
    });
    // let conf = ;
    println!("Serving on {}", addr);
    return server
        .bind(addr)
        .expect("error binding server to the address!")
        .run()
        .await;
}
