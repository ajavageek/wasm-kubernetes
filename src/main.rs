mod model;

use crate::model::BinResponse;
use axum::extract::{OriginalUri, Query};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use std::collections::HashMap;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/get", axum::routing::get(get));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn get(
    OriginalUri(uri): OriginalUri,
    Query(params): Query<HashMap<String, String>>,
    header_map: HeaderMap,
) -> Response {
    let headers = header_map.iter().map(|(k, v)| {
        let value = v.to_str().unwrap().to_string();
        (k.to_string(), value )
    }).collect();
    //headers.iter().for_each(|(k, v)| println!("{}: {:?}", k, v));
    let bin_response = BinResponse::new(params, headers, uri.to_string());
    Json(bin_response).into_response()
}
