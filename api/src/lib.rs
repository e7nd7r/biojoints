mod handlers;
mod service;

use actix_web::{web::Data, App, HttpServer};

use handlers::*;
use service::service_bundle::ServiceBundle;

pub struct ApiService {
}

impl ApiService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
        let system = actix_web::rt::System::new();
        let config = service::config::ApiConfig::from_toml().unwrap();
        let service_bundle = system.block_on(ServiceBundle::new(config)).unwrap();

        let _ = actix_web::rt::System::new()
            .block_on(
                HttpServer::new(move || {
                    App::new()
                        .app_data(Data::new(service_bundle.clone()))
                        .service(kingdom::list_kingdom) 
                })
                .bind("127.0.0.1:8080")
                .expect("Failed to start the server")
                .run()
            );
    }
}

impl Default for ApiService {
    fn default() -> Self {
        Self::new()
    }
}

