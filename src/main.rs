#![allow(dead_code)]

mod common;

// use egg_mode::user;
// use futures;
// use futures::StreamExt;


use egg_mode;
use egg_mode::user;
use egg_mode::search::{self, ResultType};
use std::io::{stdin, BufRead};
// use std;
// use std::io::{Read, Write};
use tokio;


#[tokio::main]
async fn main() {

    // let access_token = env::var("ACCESS_TOKEN").expect("Please set ACCESS_TOKEN");
    // let access_token_secret = env::var("ACCESS_TOKEN_SECRET").expect("Please set ACCESS_TOKEN_SECRET");

    // let con_token = egg_mode::KeyPair::new(api_key, api_key_secret);
    // let egg_access_token = egg_mode::KeyPair::new(access_token, access_token_secret);
    // let token = egg_mode::Token::Access {
        // consumer: con_token,
        // access: egg_access_token,
    // };

    // let rustlang = egg_mode::user::show("rustlang", &token).await.unwrap();

    // println!("{} (@{})", rustlang.name, rustlang.screen_name);

    let config = common::Config::load().await;

    println!("");
    println!("Heterogeneous multi-user lookup:");

    let mut users: Vec<egg_mode::user::UserID> = vec![];
    users.push("KNeferhetep".into());

        for user in user::lookup(users, &config.token)
        .await
        .unwrap()
        .response
        .iter()
    {
        print_user(user)
    }

    println!("Search term:");
    let line = stdin().lock().lines().next().unwrap().unwrap();

    println!("\n");

    let search = search::search(line)
        .result_type(ResultType::Recent)
        .count(10)
        .call(&config.token)
        .await
        .unwrap();

    for tweet in &search.statuses {
        common::print_tweet(tweet);
        println!()
    }
}

fn print_user(user: &user::TwitterUser) {
    println!("");
    println!("{} (@{})", user.name, user.screen_name);
    println!("Created at {}", user.created_at);
    println!(
        "Follows {}, followed by {}",
        user.friends_count, user.followers_count
    );
    if let Some(ref desc) = user.description {
        println!("{}", desc);
    } else {
        println!("(no description provided)");
    }
    match (&user.location, &user.url) {
        (&Some(ref loc), &Some(ref link)) => println!("{} | {}", loc, link),
        (&None, &Some(ref link)) => println!("{}", link),
        (&Some(ref loc), &None) => println!("{}", loc),
        (&None, &None) => (),
    }
}
