mod handlers;

use actix_web::{web::Data, App, HttpServer};

use handlers::*;
use mysql::Pool;
use neo4rs::Graph;

pub struct ApiService {
}

async fn create_pools() -> (Pool, Graph) {
    let url = "";
    let pool = Pool::new(url).expect("Error creating the pool");

    let graph = Graph::new(
        "",
        "",
        ""
    ).await.unwrap();

    (pool, graph)
}

impl ApiService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
        // mysql://root:password@localhost:3307/db_name
        let system = actix_web::rt::System::new();
        let (pool, graph) = system.block_on(create_pools());

        let _ = actix_web::rt::System::new()
            .block_on(
                HttpServer::new(move || {
                    App::new()
                        .app_data(Data::new(pool.clone()))
                        .app_data(Data::new(graph.clone()))
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

