async fn fetch_from_url(url: &str) -> reqwest::Result<String> {
    let response_text = reqwest::get(url).await?.text().await?;
    Ok(response_text)
}

#[tokio::main]
async fn main() {
    match fetch_from_url("").await {
        Ok(response) => println!("{}", response),
        Err(err) => eprintln!("{:?}", err)
    }
}