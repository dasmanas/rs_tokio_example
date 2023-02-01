use reqwest;

#[tokio::main]
async fn main() {
    let zipcodes = vec!["92606", "92707"];
    for zipcode in zipcodes {
        process_zip_info(zipcode).await;
    }
}

async fn process_zip_info(zipcode: &str) {
    let request_url = format!("https://api.zippopotam.us/us/{}", zipcode);
    println!("{}", request_url);
    let response = reqwest::get(request_url).await.unwrap().text().await;
    println!("{:?}", response);
}
