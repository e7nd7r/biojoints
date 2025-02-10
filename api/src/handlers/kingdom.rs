use actix_web::{get, post, web, Error, Responder};
use models::{data::crud::{Create, Fetch}, neo4j_impl::queries::FetchKingdomQueryBuilder, records::kingdom::Kingdom};
use serde::{Deserialize, Serialize};

use crate::service::service_bundle::ServiceBundle;

#[derive(Serialize, Deserialize)]
struct KingdomQuery {
    kingdom: Option<String>,
    superkingdom: Option<String>,
    sort_by: Option<String>,
    page: Option<i32>,
}

#[get("/kingdom")]
pub async fn list_kingdom(bundle: web::Data<ServiceBundle>, _query: web::Query<KingdomQuery>) -> Result<impl Responder, Error> {
    let builder = FetchKingdomQueryBuilder{};
    let records = Kingdom::fetch(bundle.graph.clone(), &builder).await;

    let records = records.expect("Failed to fetch kingdoms");

    Ok(web::Json(records))
}

#[post("/kingdom")]
pub async fn create_kingdom(bundle: web::Data<ServiceBundle>, kingdom: web::Json<Kingdom>) -> Result<impl Responder, Error> {
    println!("{:?}", kingdom);
    let kingdom = kingdom.create(bundle.graph.clone()).await.expect("Failed to create kingdom");

    Ok(web::Json(kingdom))
}

