
use reqwest::Client;
use reqwest::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let resp = client.post("http://httpbin.org/post")
                .body("possibly too large")
                .send()
                .await?; 

    if resp.status().is_success() {
        println!("success!");
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }

    println!("Received response status: {:?}", resp.status());

    Ok(())
}