use anyhow::anyhow;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;
use shuttle_secrets::SecretStore;

mod commands;
mod scrapers;

use crate::commands::get::*;
use crate::commands::ping::*;
use crate::commands::random::*;
use crate::commands::today::*;
use crate::commands::help::*;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[group]
#[commands(ping, today, random, get, help)]
struct General;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    // Set up framework
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!!"))
        .group(&GENERAL_GROUP);

    // Create Client
    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    Ok(client.into())
}
