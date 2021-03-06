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
