use rusoto_autoscaling::{
    AutoScalingGroup, AutoScalingGroupNamesType, AutoScalingGroupsType, Autoscaling,
    AutoscalingClient, Instance,
};
use rusoto_core::Region;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

static GROUP_NAME: &'static str = "mc-omnifactory";

fn get_instance(asg: &AutoScalingGroup) -> Option<&Instance> {
    if let Some(instances) = &asg.instances {
        if let Some(instance) = instances.first() {
            return Some(instance.into());
        }
    }
    None
}

#[command]
fn test(ctx: &mut Context, msg: &Message) -> CommandResult {
    let client = AutoscalingClient::new(Region::ApNortheast1);
    let result = client.describe_auto_scaling_groups(AutoScalingGroupNamesType {
        auto_scaling_group_names: Some(vec![GROUP_NAME.to_string()]),
        max_records: None,
        next_token: None,
    });
    let mut rt = tokio::runtime::Runtime::new()?;
    let asgs: AutoScalingGroupsType = rt.block_on(result).unwrap();
    let asg = asgs
        .auto_scaling_groups
        .iter()
        .find(|&x| x.auto_scaling_group_name == GROUP_NAME.to_string());

    
    if let Some(g) = asg {
        if let Some(instance) = get_instance(g) {
            let _ = msg.channel_id.say(
                &ctx.http,
                format!("{:?}", instance)
            );

        }
    }
    let _ = msg.channel_id.say(&ctx.http, "test!");

    Ok(())
}

#[command]
fn restart(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Restarted!");

    Ok(())
}
