use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let test_var = env::var("TESTVAR").expect("Please set TESTVAR");
    let test_var2 = env::var("TESTVAR2").expect("Please set TESTVAR2");

    println!("{}", test_var);
    println!("{}", test_var2);
}
