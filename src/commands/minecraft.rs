use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::aws::aws::run_command;

static GROUP_NAME: &'static str = "mc-omnifactory";

#[command]
fn start(ctx: &mut Context, msg: &Message) -> CommandResult {
    run_command(GROUP_NAME.to_string(), "docker start mc".to_string());

    let _ = msg.channel_id.say(&ctx.http, "たぶんスタートしました！");

    Ok(())
}

#[command]
#[owners_only]
fn restart(ctx: &mut Context, msg: &Message) -> CommandResult {
    run_command(GROUP_NAME.to_string(), "docker restart mc".to_string());

    let _ = msg.channel_id.say(&ctx.http, "たぶんリスタートしました！");

    Ok(())
}
