use serenity::{framework::standard::{macros::command, CommandResult}, prelude::Context, model::prelude::Message};

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "List of commands: `!!today`, `!!random`, `!!get`").await?;
    Ok(())
}