use axum::{
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;

// use axum::{ extract::Form, response::Html, routing::post };

mod routes;
use routes::get_latest_version::latest_version_handler;

fn get_parameter(parameter: &str, default: &str, params: HashMap<String, String>) -> String {
    return params.get(parameter).cloned().unwrap_or(default.to_string());
}

// fn post_parameter(parameter: &str, default: &str, params: HashMap<String, String>) -> String {
//     return params.get(parameter).unwrap_or(&default.to_string()).to_string();
// }

// async fn add_handler(Form(params): Form<HashMap<String, String>>) -> Html<String> {
//     let a = post_parameter("a", "0", params.clone()).parse::<i32>().unwrap();
//     let b = post_parameter("b", "0", params.clone()).parse::<i32>().unwrap();
//     Html(format!("Sum is: {}", a + b))
// }

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/latest", get(latest_version_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
