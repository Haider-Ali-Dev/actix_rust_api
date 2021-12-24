use actix_web::{App, HttpServer};
use api::http::{register, sign_in};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(sign_in)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    
}