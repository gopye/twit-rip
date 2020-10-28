use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("Please set API_KEY");
    let api_key_secret = env::var("API_KEY_SECRET").expect("Please set API_KEY_SECRET");

    println!("{}", api_key);
    println!("{}", api_key_secret);
}
