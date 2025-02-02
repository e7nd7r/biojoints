use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::{http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::GraphQL;

pub struct GraphService {
}


pub struct SpecieRecord {
    pub name: String,
    pub genus: String,
    pub family: String,
}

impl SpecieRecord {
    pub fn new() -> Self {
        return Self {
            name: "familiaris".to_string(),
            genus: "canis".to_string(),
            family: "canidae".to_string(),
        }
    }
}

pub struct Specie<'a>(&'a SpecieRecord);


#[Object]
impl<'a> Specie<'a> {
    async fn name(&self) -> &str {
        let Specie(record) = self;

        &record.name
    }

    async fn genus(&self) -> &str {
        let Specie(record) = self;

        &record.genus
    }

    async fn family(&self) -> &str {
        let Specie(record) = self;

        &record.family
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn specie<'a>(&self, ctx: &Context<'a>) -> Specie<'a>{
        let record = ctx.data_unchecked::<SpecieRecord>();

        Specie(record)
    }
}

async fn index_graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish())
}

impl GraphService {
    pub fn new() -> Self {
        return Self {}
    }

    pub fn run(self) {
         let _ = actix_web::rt::System::new()
            .block_on(
                HttpServer::new(|| {
                    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                        .data(SpecieRecord::new())
                        .finish();

                    App::new()
                        .service(
                            web::resource("/")
                                .guard(guard::Post())
                                .to(GraphQL::new(schema))
                        )
                        .service(web::resource("/").guard(guard::Get()).to(index_graphiql))

                })
                .bind("127.0.0.1:8000").expect("unable to start server")
                .run()
            );
    }
}

