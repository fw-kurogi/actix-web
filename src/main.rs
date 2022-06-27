use actix_web::{App, HttpServer, HttpResponse, Responder, web };

use { app::routes::api as api_route, };

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(api_route::router)
    })
    .bind("0.0.0.0:9200")?
    .run()
    .await
}

