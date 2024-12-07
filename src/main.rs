use std::io::stdin;
use serde_json::Value;

async fn fetch_from_url(url: &str) -> reqwest::Result<String> {
    let response_text = reqwest::get(url).await?.text().await?;
    Ok(response_text)
}

#[tokio::main]
async fn main() {
    let mut site_name = String::new();
    stdin().read_line(&mut site_name).unwrap().to_string().trim().parse::<String>().unwrap();
    let fetched_site = match fetch_from_url(format!("https://raw.githubusercontent.com/sillybreakfast/termwebsites/refs/heads/master/sites/{}.json", site_name).as_str()).await {
        Ok(response) => response,
        Err(err) => format!("{{ \"title\": \"error\", \"content\": \"{:?}\" }}", err).as_str().to_owned()
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