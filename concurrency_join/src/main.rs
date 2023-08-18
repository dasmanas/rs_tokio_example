use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use common_lib::model::Zipcode;
use reqwest;
use tokio::time::{sleep, Duration};
use serde_json;
use rand::Rng;


#[tokio::main]
async fn main() {

    let zips: Vec<u32> = (210..=99950).collect();
    // let zips: Vec<u32> = (209..=215).collect();
    let s_zips: Vec<String> = zips.into_iter().map(|x| { format!("{:0>5}", x) }).collect();

    let mut handles = Vec::new();
    let file_path = "zipcode.json";
    let file_mutex = Arc::new(Mutex::new(File::create(file_path).expect("File create error")));
    let first_line = Arc::new(AtomicBool::new(true));

    for i in 0..s_zips.len() {
        let zipcode = s_zips[i].to_owned();
        let cloned_file_mutex = file_mutex.clone();
        let cloned_counter = first_line.clone();
        let random_number = rand::thread_rng().gen_range(0..=4_000_000);
        println!("random_number: {}, zipcode: {}",random_number,zipcode);

        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(random_number as u64)).await;
            process_zip_info(zipcode, cloned_file_mutex, cloned_counter).await
        });
        handles.push(handle);
    }

    let mut good_response: u32 = 0;
    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => { good_response += 1; }
            Err(err) => { eprintln!("Error Zipcode: {}", err); }
        }
    }
    if good_response > 0 {
        let mut file = file_mutex.lock().unwrap();
        file.write_all(b"]").unwrap();
    }
    println!("about to close"); //Child thread may outlived the main thread
}

async fn process_zip_info(zipcode: String, file_mutex: Arc<Mutex<File>>, first_line: Arc<AtomicBool>) -> Result<(), String> {
    let request_url = format!("https://api.zippopotam.us/us/{}", zipcode);
    println!("{}", request_url);
    match reqwest::get(request_url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Zipcode>().await {
                    Ok(zipcode) => {
                        let zipcode_json = serde_json::to_string(&zipcode).unwrap();
                        let mut file = file_mutex.lock().unwrap();
                        if first_line.load(Ordering::SeqCst) {
                            file.write_all(b"[").unwrap();
                            first_line.store(false, Ordering::SeqCst);
                        } else {
                            file.write_all(b",\n").unwrap();
                        }
                        file.write_all(zipcode_json.as_bytes()).unwrap();
                        Ok(())
                    }
                    Err(err) => { //possible serde error
                        eprintln!("Error: {} ,Zipcode: {}", err, zipcode);
                        Err(zipcode)
                    }
                }
            } else { //possible response status is not 200
                eprintln!("Error with Status Code: {:#?} ,Zipcode: {}", response.status(), zipcode);
                Err(zipcode)
            }
        }
        Err(err) => {
            eprintln!("Error: {} ,Zipcode: {}", err, zipcode);
            Err(zipcode)
        }
    }
}
