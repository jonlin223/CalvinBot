use serenity::{framework::standard::{macros::command, CommandResult}, prelude::Context, model::prelude::Message};

use crate::scrapers::calvin_scraper;

#[command]
async fn today(ctx: &Context, msg: &Message) -> CommandResult {
    let res = calvin_scraper::scrape("2023/100/22").await;
    match res {
        Ok(_) => println!("HI THERE"),
        Err(_) => println!("Oh no")
    }
    Ok(())
}