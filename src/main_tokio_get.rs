use reqwest::Error;

#[tokio::main]
async fn b_main() -> Result<(), Error> {
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;

    println!("body = {:?}", body);

    Ok(())
}