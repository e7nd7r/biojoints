use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub struct ApiService {
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix Web!")
}

impl ApiService {
    pub fn new() -> Self {
        return Self {}
    }

    pub fn run(self) {
        let _ = actix_web::rt::System::new()
        .block_on(
            HttpServer::new(|| App::new().service(hello))
            .bind("127.0.0.1:8080")
            .expect("Failed to start the server")
            .run()
        );
    }
}
