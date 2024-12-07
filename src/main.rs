use std::io::stdin;
use serde_json::Value;

async fn fetch_from_url(url: &str) -> reqwest::Result<String> {
    let response = reqwest::get(url).await?;
    Ok(match response.status() {
        reqwest::StatusCode::OK => response.text().await?,
        reqwest::StatusCode::BAD_REQUEST => String::from("{ \"title\": \"error\", \"content\": \"400 bad request\" }"),
        reqwest::StatusCode::UNAUTHORIZED => String::from("{ \"title\": \"error\", \"content\": \"401 unauthorized\" }"),
        reqwest::StatusCode::PAYMENT_REQUIRED => String::from("{ \"title\": \"error\", \"content\": \"403 payment required\" }"),
        reqwest::StatusCode::FORBIDDEN => String::from("{ \"title\": \"error\", \"content\": \"403 forbidden\" }"),
        reqwest::StatusCode::NOT_FOUND => String::from("{ \"title\": \"error\", \"content\": \"404 not found\" }"),
        reqwest::StatusCode::METHOD_NOT_ALLOWED => String::from("{ \"title\": \"error\", \"content\": \"405 method not allowed\" }"),
        reqwest::StatusCode::NOT_ACCEPTABLE => String::from("{ \"title\": \"error\", \"content\": \"406 not acceptable\" }"),
        reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED => String::from("{ \"title\": \"error\", \"content\": \"407 proxy authentication required\" }"),
        reqwest::StatusCode::REQUEST_TIMEOUT => String::from("{ \"title\": \"error\", \"content\": \"408 request timeout\" }"),
        reqwest::StatusCode::CONFLICT => String::from("{ \"title\": \"error\", \"content\": \"409 conflict\" }"),
        reqwest::StatusCode::GONE => String::from("{ \"title\": \"error\", \"content\": \"410 gone\" }"),
        reqwest::StatusCode::LENGTH_REQUIRED => String::from("{ \"title\": \"error\", \"content\": \"411 length required\" }"),
        reqwest::StatusCode::PRECONDITION_FAILED => String::from("{ \"title\": \"error\", \"content\": \"412 precondition failed\" }"),
        reqwest::StatusCode::PAYLOAD_TOO_LARGE => String::from("{ \"title\": \"error\", \"content\": \"413 payload too large\" }"),
        reqwest::StatusCode::URI_TOO_LONG => String::from("{ \"title\": \"error\", \"content\": \"414 URI too long\" }"),
        reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE => String::from("{ \"title\": \"error\", \"content\": \"415 unsupported media type\" }"),
        reqwest::StatusCode::RANGE_NOT_SATISFIABLE => String::from("{ \"title\": \"error\", \"content\": \"416 range not satisfiable\" }"),
        reqwest::StatusCode::EXPECTATION_FAILED => String::from("{ \"title\": \"error\", \"content\": \"417 expectation failed\" }"),
        reqwest::StatusCode::IM_A_TEAPOT => String::from("{ \"title\": \"error\", \"content\": \"418 i'm a teapot!! >.<\" }"),
        reqwest::StatusCode::MISDIRECTED_REQUEST => String::from("{ \"title\": \"error\", \"content\": \"421 misdirected request\" }"),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => String::from("{ \"title\": \"error\", \"content\": \"422 unprocessable entity\" }"),
        reqwest::StatusCode::LOCKED => String::from("{ \"title\": \"error\", \"content\": \"423 locked\" }"),
        reqwest::StatusCode::FAILED_DEPENDENCY => String::from("{ \"title\": \"error\", \"content\": \"424 failed dependency\" }"),
        reqwest::StatusCode::TOO_EARLY => String::from("{ \"title\": \"error\", \"content\": \"425 too early\" }"),
        reqwest::StatusCode::UPGRADE_REQUIRED => String::from("{ \"title\": \"error\", \"content\": \"426 upgrade required\" }"),
        reqwest::StatusCode::PRECONDITION_REQUIRED => String::from("{ \"title\": \"error\", \"content\": \"428 precondition required\" }"),
        reqwest::StatusCode::TOO_MANY_REQUESTS => String::from("{ \"title\": \"error\", \"content\": \"429 too many requests\" }"),
        reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => String::from("{ \"title\": \"error\", \"content\": \"431 request header fields too large\" }"),
        reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => String::from("{ \"title\": \"error\", \"content\": \"451 unavailable for legal reasons\" }"),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => String::from("{ \"title\": \"error\", \"content\": \"500 internal server error\" }"),
        reqwest::StatusCode::NOT_IMPLEMENTED => String::from("{ \"title\": \"error\", \"content\": \"501 not implemented\" }"),
        reqwest::StatusCode::BAD_GATEWAY => String::from("{ \"title\": \"error\", \"content\": \"502 bad gateway\" }"),
        reqwest::StatusCode::SERVICE_UNAVAILABLE => String::from("{ \"title\": \"error\", \"content\": \"503 service unavailable\" }"),
        reqwest::StatusCode::GATEWAY_TIMEOUT => String::from("{ \"title\": \"error\", \"content\": \"504 gateway timeout\" }"),
        reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED => String::from("{ \"title\": \"error\", \"content\": \"505 HTTP version not supported\" }"),
        reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES => String::from("{ \"title\": \"error\", \"content\": \"506 variant also negotiates\" }"),
        reqwest::StatusCode::INSUFFICIENT_STORAGE => String::from("{ \"title\": \"error\", \"content\": \"507 insufficient storage\" }"),
        reqwest::StatusCode::LOOP_DETECTED => String::from("{ \"title\": \"error\", \"content\": \"508 loop detected\" }"),
        reqwest::StatusCode::NOT_EXTENDED => String::from("{ \"title\": \"error\", \"content\": \"510 not extended\" }"),
        reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED => String::from("{ \"title\": \"error\", \"content\": \"511 network authentication required\" }"),
        _ => String::from("{ \"title\": \"error\", \"content\": \"there was an unknown error loading the site.\" }"),
    })
}

#[tokio::main]
async fn main() {
    println!("\x1b[1mwelcome to termwebbrowser!\x1b[0m");
    println!("\x1b[2mtermwebbrowser  \x1b[1m{}\x1b[0m", env!("CARGO_PKG_VERSION", "unknown"));
    let client = reqwest::Client::builder().user_agent("sillybreakfast/termwebbrowser").build().unwrap();
    let current_web_release = client.get("https://api.github.com/repos/sillybreakfast/termwebsites/releases/latest").send().await.unwrap().text().await.unwrap();
    let current_web_release: Value = serde_json::from_str(current_web_release.as_str()).unwrap();
    let current_web_release_name = match current_web_release.get("tag_name") {
        Some(tag_name) => tag_name.as_str().unwrap(),
        _ => "unknown"
    };
    println!("\x1b[2mtermwebsites    \x1b[1m{}\x1b[0m", current_web_release_name);
    let mut site_name = String::new();
    stdin().read_line(&mut site_name).unwrap().to_string();
    site_name = format!("{}/", site_name.trim());
    let fetched_site = match fetch_from_url(format!("https://raw.githubusercontent.com/sillybreakfast/termwebsites/refs/tags/{}/sites/{}.json", current_web_release_name, site_name).as_str()).await {
        Ok(response) => response,
        Err(_) => String::from("{ \"title\": \"error\", \"content\": \"there was an error loading the site.\" }")
    };
    let site_json: Value = serde_json::from_str(fetched_site.as_str()).unwrap();
    println!("\x1b[1m{}\x1b[0m", match site_json.get("title") {
        Some(title) => title.as_str().unwrap(),
        _ => "untitled"
    });
    println!("{}", match site_json.get("content") {
        Some(content) => content.as_str().unwrap(),
        _ => ""
    });
}