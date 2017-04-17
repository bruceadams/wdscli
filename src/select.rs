use clap;
use info::{DiscoveryServiceInfo, EnvironmentInfo};
use wdsapi::collection::Collection;
use wdsapi::configuration::Configuration;

pub fn read_only_environment(info: &DiscoveryServiceInfo) -> EnvironmentInfo {
    let read_only: Vec<EnvironmentInfo> =
        info.environments
            .clone()
            .into_iter()
            .filter({
                |env| env.environment.read_only
            })
            .collect();

    assert!(read_only.len() <= 1,
            format!("Multiple read only environments found {:?}", read_only));

    read_only.first().expect("No read only environment found").clone()
}

pub fn writable_environment(info: &DiscoveryServiceInfo) -> EnvironmentInfo {
    let writables: Vec<EnvironmentInfo> =
        info.environments
            .clone()
            .into_iter()
            .filter({
                |env| !env.environment.read_only
            })
            .collect();

    assert!(writables.len() <= 1,
            format!("Multiple writable environments found {:?}", writables));

    writables.first().expect("No writable environment found").clone()
}

pub fn oldest_collection(env: &EnvironmentInfo) -> Collection {
    env.collections
       .clone()
       .into_iter()
       .min_by_key({
           |i| i.created
       })
       .expect("No collections found")
}

pub fn newest_collection(env: &EnvironmentInfo) -> Collection {
    env.collections
       .clone()
       .into_iter()
       .max_by_key({
           |i| i.created
       })
       .expect("No collections found")
}

pub fn collection_with_name(env: &EnvironmentInfo, name: &str) -> Collection {
    let f: Vec<Collection> = env.collections
                                .clone()
                                .into_iter()
                                .filter({
                                    |i| i.name == name
                                })
                                .collect();
    assert_eq!(f.len(), 1, "No collection matched {}", name);
    f.first().expect("Internal error: count=1, but no last!?").clone()
}

pub fn collection_with_id(env: &EnvironmentInfo, id: &str) -> Collection {
    let f: Vec<Collection> = env.collections
                                .clone()
                                .into_iter()
                                .filter({
                                    |i| i.collection_id == id
                                })
                                .collect();
    assert_eq!(f.len(), 1, "No collection matched {}", id);
    f.last().expect("Internal error: count=1, but no last!?").clone()
}

pub fn select_collection(env_info: &EnvironmentInfo,
                         matches: &clap::ArgMatches)
                         -> Collection {
    if matches.is_present("name") {
        collection_with_name(env_info, matches.value_of("name").unwrap())
    } else if matches.is_present("id") {
        collection_with_id(env_info, matches.value_of("id").unwrap())
    } else if matches.is_present("oldest") {
        oldest_collection(env_info)
    } else {
        newest_collection(env_info)
    }
}

pub fn configuration_with_name(env: &EnvironmentInfo,
                               configuration_name: &str)
                               -> Configuration {
    env.configurations
       .clone()
       .into_iter()
       .filter({
           |c| c.name == configuration_name
       })
       .last()
       .expect("No configuration found")
       .clone()
}

pub fn configuration_with_id(env: &EnvironmentInfo,
                             configuration_id: &str)
                             -> Configuration {
    env.configurations
       .clone()
       .into_iter()
       .filter({
           |c| c.configuration_id == Some(configuration_id.to_string())
       })
       .last()
       .expect("No configuration found")
       .clone()
}

pub fn oldest_configuration(env: &EnvironmentInfo) -> Configuration {
    env.configurations
       .clone()
       .into_iter()
       .min_by_key({
           |i| i.created
       })
       .expect("No configuration found")
}

pub fn newest_configuration(env: &EnvironmentInfo) -> Configuration {
    env.configurations
       .clone()
       .into_iter()
       .max_by_key({
           |i| i.created
       })
       .expect("No configuration found")
}

pub fn select_configuration(env_info: &EnvironmentInfo,
                            matches: &clap::ArgMatches)
                            -> Configuration {
    if matches.is_present("name") {
        configuration_with_name(env_info, matches.value_of("name").unwrap())
    } else if matches.is_present("id") {
        configuration_with_id(env_info, matches.value_of("id").unwrap())
    } else if matches.is_present("oldest") {
        oldest_configuration(env_info)
    } else {
        newest_configuration(env_info)
    }
}
