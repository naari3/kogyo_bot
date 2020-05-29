use rusoto_autoscaling::{
    AutoScalingGroup, AutoScalingGroupNamesType, AutoScalingGroupsType, Autoscaling,
    AutoscalingClient, Instance,
};
use rusoto_core::Region;
use rusoto_ssm::{SendCommandRequest, Ssm, SsmClient};

use std::collections::HashMap;
use std::default::Default;

static GROUP_NAME: &'static str = "mc-omnifactory";

fn get_instance(asg: &AutoScalingGroup) -> Option<&Instance> {
    if let Some(instances) = &asg.instances {
        if let Some(instance) = instances.first() {
            return Some(instance.into());
        }
    }
    None
}

fn get_auto_scaling_group(group_name: String) -> Option<AutoScalingGroup> {
    let client = AutoscalingClient::new(Region::ApNortheast1);
    let result = client.describe_auto_scaling_groups(AutoScalingGroupNamesType {
        auto_scaling_group_names: Some(vec![GROUP_NAME.to_string()]),
        max_records: None,
        next_token: None,
    });
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let asgs: AutoScalingGroupsType = rt.block_on(result).unwrap();
    let asg = asgs
        .auto_scaling_groups
        .into_iter()
        .find(|x| x.auto_scaling_group_name == group_name);

    if let Some(asg) = asg {
        return Some(asg);
    }
    None
}

pub fn run_command(group_name: String, command: String) {
    let asg = get_auto_scaling_group(group_name);
    let mut instance_ids: Option<Vec<String>> = None;
    if let Some(g) = asg {
        if let Some(instance) = get_instance(&g) {
            instance_ids = Some(vec![instance.instance_id.clone()]);
        };
    }

    let ssm_client = SsmClient::new(Region::ApNortheast1);
    let mut params = HashMap::new();
    params.insert("commands".to_string(), vec![command]);
    let scr = SendCommandRequest {
        document_name: "AWS-RunShellScript".to_string(),
        instance_ids: instance_ids,
        parameters: Some(params),
        ..Default::default()
    };

    let result = ssm_client.send_command(scr);
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.block_on(result).unwrap();
}
