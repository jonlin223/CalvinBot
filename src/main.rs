use std::env;

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    prelude::{EventHandler, GatewayIntents},
    Client,
};

mod commands;
mod scrapers;

use crate::commands::ping::*;
use crate::commands::today::*;
use crate::commands::random::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[group]
#[commands(ping, today, random)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!!"))
        .group(&GENERAL_GROUP);

    let token = env::var("TOKEN").expect("Could not find token in .env file");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("An error occurred while running the client: {:?}", e);
    }
}
