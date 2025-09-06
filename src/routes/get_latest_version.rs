use axum::{
    extract::Query,
    response::Html,
};
use std::collections::HashMap;

use crate::get_parameter;

pub async fn latest_version_handler(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    let latest_ver: String = "ALPHA".to_string();
    let client_ver: String = get_parameter("version", "null", params);

    let allowed_versions: Vec<String> = vec![
        "ALPHA".to_string()
    ];

    let access_level: String = if allowed_versions.contains(&client_ver) {
        "all".to_string()
    } else {
        "none".to_string()
    };

    Html(format!("{};{}", latest_ver, access_level))
}
