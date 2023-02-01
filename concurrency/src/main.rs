use reqwest;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let zipcodes = vec!["92606", "92707"];
    for i in 0..zipcodes.len() {
        let zipcode = zipcodes[i].to_owned();
        tokio::spawn(async move {
            process_zip_info(zipcode).await;
            sleep(Duration::from_millis(100)).await;
        });
    }
    println!("about to close"); //Child thread may outlived the main thread
}

async fn process_zip_info(zipcode: String) {
    let request_url = format!("https://api.zippopotam.us/us/{}", zipcode);
    println!("{}", request_url);
    let response = reqwest::get(request_url).await.unwrap().text().await;
    println!("{:?}", response);
}
