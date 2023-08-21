use std::env;

use serenity::{
    async_trait,
    framework::{StandardFramework, standard::{macros::{group, command}, CommandResult}},
    prelude::{EventHandler, GatewayIntents, Context},
    Client, model::prelude::Message,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[group]
#[commands(ping)]
struct General;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let framework = StandardFramework::new().configure(|c| c.prefix("!!")).group(&GENERAL_GROUP);

    let token = env::var("TOKEN").expect("token");
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
