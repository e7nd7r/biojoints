use actix_web::{get, post, web, Error, Responder};
use models::{neo4j_impl::{graph_layer::GraphLayer, kingdom::KingdomModel}, records::kingdom::Kingdom};
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
    let graph_layer = GraphLayer::new(bundle.graph.clone());
    let model = KingdomModel::new(graph_layer);

    let records = model.fetch().await;

    let records = records.expect("Failed to fetch kingdoms");

    Ok(web::Json(records))
}

#[post("/kingdom")]
pub async fn create_kingdom(bundle: web::Data<ServiceBundle>, kingdom: web::Json<Kingdom>) -> Result<impl Responder, Error> {
    let graph_layer = GraphLayer::new(bundle.graph.clone());
    let model = KingdomModel::new(graph_layer);
    let kingdom = kingdom.into_inner();

    let kingdom = model.create(kingdom).await.expect("Failed to create kingdom");

    Ok(web::Json(kingdom))
}

