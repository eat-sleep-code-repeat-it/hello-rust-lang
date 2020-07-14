use hyper::{Client, Uri};

// https://stackoverflow.com/questions/14154753/how-do-i-make-an-http-request-from-rust/14189088#14189088

#[tokio::main]
async fn main() {
    let path = "https://httpbin.org/ip";
    match reqwest::get(path).await {
        Ok(resp) => {
            match resp.text().await {
                Ok(text) => {
                    println!("RESPONSE: {} bytes from {}", text.len(), path);
                }
                Err(_) => println!("ERROR reading {}", path),
            }
        }
        Err(_) => println!("ERROR downloading {}", path),
    }
    Ok(())
}