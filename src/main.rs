use std::env;

use serenity::{framework::StandardFramework, prelude::GatewayIntents};

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!!"));

    let token = env::var("TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
}