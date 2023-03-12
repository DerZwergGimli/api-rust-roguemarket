use json::object;
use log::info;
use reqwest::header;

pub async fn request_token(key: String) -> Option<String> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse().unwrap());

    let client = reqwest::Client::new();
    let res = client.post("https://auth.streamingfast.io/v1/auth/issue")
        .headers(headers)
        .body(object! {"api_key": key}.to_string())
        .send()
        .await.unwrap()
        .text().await.unwrap();

    let json_response = json::parse(res.clone().as_str()).expect("Error parsing response!");


    let d = json_response["token"].to_string();
    info!("token_request_info={:?}", res.clone());

    Some(d.clone())
}