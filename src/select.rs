use clap;
use info::{DiscoveryServiceInfo, EnvironmentInfo};
use serde_json::Value;

pub fn read_only_environment(info: &DiscoveryServiceInfo) -> EnvironmentInfo {
    let read_only: Vec<EnvironmentInfo> =
        info.environments
            .clone()
            .into_iter()
            .filter({
            |env| env.environment["read_only"].as_bool().unwrap_or(false)
        })
            .collect();

    assert!(
        read_only.len() <= 1,
        format!("Multiple read only environments found {:?}", read_only)
    );

    read_only.first()
             .expect("No read only environment found")
             .clone()
}

pub fn writable_environment(info: &DiscoveryServiceInfo) -> EnvironmentInfo {
    let writables: Vec<EnvironmentInfo> =
        info.environments
            .clone()
            .into_iter()
            .filter({
            |env| !env.environment["read_only"].as_bool().unwrap_or(true)
        })
            .collect();

    assert!(
        writables.len() <= 1,
        format!("Multiple writable environments found {:?}", writables)
    );

    writables.first()
             .expect("No writable environment found")
             .clone()
}

pub fn oldest_collection(env: &EnvironmentInfo) -> Value {
    env.collections
       .first()
       .expect("No collections found")
       .clone()
}

pub fn newest_collection(env: &EnvironmentInfo) -> Value {
    env.collections
       .last()
       .expect("No collections found")
       .clone()
}

pub fn collection_with_name(env: &EnvironmentInfo, name: &str) -> Value {
    let f: Vec<Value> = env.collections
                           .clone()
                           .into_iter()
                           .filter({
        |i| i["name"].as_str() == Some(name)
    })
                           .collect();
    assert_eq!(f.len(), 1, "No collection matched {}", name);
    f.first()
     .expect("Internal error: count=1, but no last!?")
     .clone()
}

pub fn collection_with_id(env: &EnvironmentInfo, id: &str) -> Value {
    let f: Vec<Value> = env.collections
                           .clone()
                           .into_iter()
                           .filter({
        |i| i["collection_id"].as_str() == Some(id)
    })
                           .collect();
    assert_eq!(f.len(), 1, "No collection matched {}", id);
    f.last()
     .expect("Internal error: count=1, but no last!?")
     .clone()
}

pub fn select_collection(
    env_info: &EnvironmentInfo,
    matches: &clap::ArgMatches,
) -> Value {
    if matches.is_present("named") {
        collection_with_name(env_info, matches.value_of("named").unwrap())
    } else if matches.is_present("id") {
        collection_with_id(env_info, matches.value_of("id").unwrap())
    } else if matches.is_present("oldest") {
        oldest_collection(env_info)
    } else {
        newest_collection(env_info)
    }
}

pub fn configuration_with_name(
    env: &EnvironmentInfo,
    configuration_name: &str,
) -> Value {
    env.configurations
       .clone()
       .into_iter()
       .filter({
        |c| c["name"].as_str() == Some(configuration_name)
    })
       .last()
       .expect("No configuration found")
       .clone()
}

pub fn configuration_with_id(
    env: &EnvironmentInfo,
    configuration_id: &str,
) -> Value {
    env.configurations
       .clone()
       .into_iter()
       .filter({
        |c| c["configuration_id"].as_str() == Some(configuration_id)
    })
       .last()
       .expect("No configuration found")
       .clone()
}

pub fn oldest_configuration(env: &EnvironmentInfo) -> Value {
    env.configurations
       .first()
       .expect("No configurations found")
       .clone()
}

pub fn newest_configuration(env: &EnvironmentInfo) -> Value {
    env.configurations
       .last()
       .expect("No configurations found")
       .clone()
}

pub fn select_configuration(
    env_info: &EnvironmentInfo,
    matches: &clap::ArgMatches,
) -> Value {
    if matches.is_present("named") {
        configuration_with_name(env_info, matches.value_of("named").unwrap())
    } else if matches.is_present("id") {
        configuration_with_id(env_info, matches.value_of("id").unwrap())
    } else if matches.is_present("oldest") {
        oldest_configuration(env_info)
    } else {
        newest_configuration(env_info)
    }
}
