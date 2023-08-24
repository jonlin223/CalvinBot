use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

use crate::scrapers::calvin_scraper;

#[command]
async fn get(ctx: &Context, msg: &Message) -> CommandResult {
    let cnt = &msg.content;

    // Must be in format !!get YYYY/MM/DD
    let n = cnt.split(' ').count();
    if n != 2 {
        msg.reply(ctx, "Usage: !!get YYYY/MM/DD").await?;
        return Ok(());
    }

    // Get date parameter and check if formatted somewhat correctly
    let date = cnt.split(' ').nth(1).unwrap();
    let n = date.split('/').count();
    if n != 3 {
        msg.reply(ctx, "Usage: !!get YYYY/MM/DD").await?;
        return Ok(());
    }

    // Parse the date
    let parsed_date = dateparser::parse(date);
    let parsed_date = match parsed_date {
        Ok(d) => d,
        Err(_) => {
            msg.reply(ctx, "Error: Invalid Date").await?;
            return Ok(());
        }
    };

    // Format the date back into a string
    let date_string = format!("{}", parsed_date.format("%Y/%m/%d"));

    // Call the scraper
    let url = &("https://www.gocomics.com/calvinandhobbes/".to_owned() + &date_string);
    let res = calvin_scraper::scrape(url).await;
    match res {
        Some(x) => msg.reply(ctx, x).await?,
        None => {
            msg.reply(ctx, format!("No comic found for {date_string}"))
                .await?
        }
    };

    Ok(())
}
