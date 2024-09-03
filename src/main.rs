use actix_web::{ middleware, web, App, HttpResponse, HttpServer};
use serde_json::Error;

mod request;

async fn echo(body: web::Bytes) -> Result<HttpResponse, Error> {
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<request::Request>(&body)?;
    Ok(HttpResponse::Ok().insert_header(("x-jacob-test", "value")).json(obj)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    rustagent::init_tracer();
    log::info!("starting HTTP server at port 5001");

    HttpServer::new(|| {
        App::new()
            .wrap(rustagent::RustAgent::default())
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/echo").route(web::post().to(echo)))    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
