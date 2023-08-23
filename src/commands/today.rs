use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::scrapers::calvin_scraper::{self, get_today};

#[command]
async fn today(ctx: &Context, msg: &Message) -> CommandResult {
    let date = get_today().await;
    match date {
        Some(d) => {
            let url = &("https://www.gocomics.com/calvinandhobbes/".to_owned() + &d);
            let res = calvin_scraper::scrape(url).await;
            match res {
                Some(x) => msg.reply(ctx, x).await?,
                None => msg.reply(ctx, "Could not find today's comic").await?,
            }
        }
        None => msg.reply(ctx, "Could not find today's comic").await?,
    };

    Ok(())
}
