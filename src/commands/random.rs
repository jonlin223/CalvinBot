use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::scrapers::calvin_scraper;

#[command]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    let url = "https://www.gocomics.com/random/calvinandhobbes";
    let res = calvin_scraper::scrape(url).await;
    match res {
        Some(x) => msg.reply(ctx, x).await?,
        None => msg.reply(ctx, "Could not find a random comic").await?,
    };

    Ok(())
}
