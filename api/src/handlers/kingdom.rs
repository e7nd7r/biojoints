use actix_web::{get, web, Error, Responder};
use models::{data::crud::Fetch, neo4j_impl::queries::FetchKingdomQueryBuilder, records::kingdom::Kingdom};
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

