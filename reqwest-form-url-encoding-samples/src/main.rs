use std::collections::HashMap;
use reqwest::Response;
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let res = url_encoded_manually().await?;
    // let res = url_encode_via_serde_urlencoded().await?;
    let res = url_encode_via_serde_qs().await?;
    dbg!(res);
    Ok(())
}

// works well
async fn url_encoded_manually() -> Result<Response, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("id".to_string(), 42);

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    'label: for (i, v) in values.iter().enumerate() {
        params.insert(format!("values[{i}]"), *v);
    };

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:5129/accept-form-url-encoded")
        .form(&params)
        .send()
        .await?;
    Ok(res)
}

// non-working, crate is quite raw
async fn url_encode_via_serde_urlencoded() -> Result<Response, Box<dyn std::error::Error>> {
    let data = FormData {
        id: 42,
        values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };

    let encoded_data = serde_urlencoded::to_string(&data).unwrap();
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:5129/accept-form-url-encoded")
        .form(&encoded_data)
        .send().await?;
    Ok(response)
}

// works well - most generic version
async fn url_encode_via_serde_qs() -> Result<Response, Box<dyn std::error::Error>> {
    let data = FormData {
        id: 42,
        values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };

    let encoded_data = serde_qs::to_string(&data).unwrap();
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:5129/accept-form-url-encoded")
        .body(encoded_data)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .send().await?;
    Ok(response)
}

#[derive(Debug, Serialize)]
pub struct FormData {
    id: i32,
    values: Vec<i32>,
}