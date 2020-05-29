use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::aws::aws::run_command;

use std::env;

#[command]
fn start(ctx: &mut Context, msg: &Message) -> CommandResult {
    run_command(
        env::var("AWS_GROUP_NAME").expect("Expected a token in the environment"),
        "docker start mc".to_string(),
    );

    let _ = msg.channel_id.say(&ctx.http, "たぶんスタートしました！");

    Ok(())
}

#[command]
#[owners_only]
fn restart(ctx: &mut Context, msg: &Message) -> CommandResult {
    run_command(
        env::var("AWS_GROUP_NAME").expect("Expected a token in the environment"),
        "docker restart mc".to_string(),
    );

    let _ = msg.channel_id.say(&ctx.http, "たぶんリスタートしました！");

    Ok(())
}
