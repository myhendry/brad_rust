extern crate reqwest;

// HTTP GET REQWEST L38
pub fn run() {
    // Shorter Get Method
    let response_text = reqwest::get("https://swapi.co/api/people/3")
        .expect("Could not make request")
        .text()
        .expect("Could not read response text");

    println!("Response Text {}", response_text);

    // Longer Get Method
    match reqwest::get("https://swapi.co/api/people/2") {
        Ok(mut response) => {
            // Check if 200 Ok
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text"),
                }
            } else {
                println!("Response was not 200 OK")
            }
        }
        Err(_) => println!("Could not make the request"),
    }
}
