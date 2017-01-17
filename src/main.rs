extern crate clap;
extern crate serde_json;
extern crate wdsapi;
extern crate rayon;

mod cli;

use rayon::prelude::*;
use serde_json::de::from_reader;
use serde_json::ser::to_string_pretty;
use std::thread::{JoinHandle, spawn};

use wdsapi::collection;
use wdsapi::collection::{Collection, NewCollection};
use wdsapi::common::{Credentials, Status, credentials_from_file};
use wdsapi::configuration;
use wdsapi::configuration::Configuration;
use wdsapi::environment;
use wdsapi::environment::{Environment, NewEnvironment};

#[derive(Clone, Debug)]
struct EnvironmentInfo {
    environment: Environment,
    configurations: Vec<Configuration>,
    collections: Vec<Collection>,
}

#[derive(Clone, Debug)]
struct DiscoveryServiceInfo {
    creds: Credentials,
    environments: Vec<EnvironmentInfo>,
}

fn get_configurations_thread(creds: &Credentials,
                             env_id: &str)
                             -> JoinHandle<Vec<Configuration>> {
    let creds = creds.clone();
    let env_id = env_id.to_string();
    spawn(move || {
        configuration::list(&creds, &env_id)
            .expect("Failed to get configuration information, in thread")
            .configurations
    })
}

fn get_collections_thread(creds: &Credentials,
                          env_id: &str)
                          -> JoinHandle<Vec<Collection>> {
    let creds = creds.clone();
    let env_id = env_id.to_string();
    spawn(move || {
        collection::list(&creds, &env_id)
            .expect("Failed to get collection information, in thread")
            .collections
            .par_iter()
            .map({
                |col| {
                    collection::detail(&creds, &env_id, &col.collection_id)
                        .expect("Failed to get collection detail, in thread")
                }
            })
            .weight_max()
            .collect()
    })
}

fn environment_info(creds: &Credentials, env: &Environment) -> EnvironmentInfo {
    let env_id = env.environment_id.clone();
    // launch threads for these API calls
    let conf_thread = get_configurations_thread(creds, &env_id);
    let col_thread = get_collections_thread(creds, &env_id);
    // Gather the results from the threads
    let configurations = conf_thread.join()
                                    .expect("Failed to get configuration \
                                             information");
    let collections = col_thread.join()
                                .expect("Failed to get collection information");
    EnvironmentInfo {
        environment: env.clone(),
        configurations: configurations,
        collections: collections,
    }
}

fn discovery_service_info(matches: &clap::ArgMatches) -> DiscoveryServiceInfo {
    let creds_file = matches.value_of("credentials")
                            .expect("Internal error: Missing credentials?");
    let creds = credentials_from_file(creds_file).expect("Invalid credentials");
    let environments = environment::list(&creds)
        .expect("Failed to get environment information")
        .environments
        .par_iter()
        .map({
            |env| environment_info(&creds, env)
        })
        .weight_max()
        .collect();
    DiscoveryServiceInfo {
        creds: creds,
        environments: environments,
    }
}

fn writable_environment(info: &DiscoveryServiceInfo) -> EnvironmentInfo {
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

    writables.first()
             .expect("No writable environment found")
             .clone()
}

fn newest_configuration(env: &EnvironmentInfo) -> Configuration {
    env.configurations
       .clone()
       .into_iter()
       .max_by_key({
           |i| i.created
       })
       .expect("No configurations found")
}

fn oldest_collection(env: &EnvironmentInfo) -> Collection {
    env.collections
       .clone()
       .into_iter()
       .min_by_key({
           |i| i.created
       })
       .expect("No collections found")
}

fn newest_collection(env: &EnvironmentInfo) -> Collection {
    env.collections
       .clone()
       .into_iter()
       .max_by_key({
           |i| i.created
       })
       .expect("No collections found")
}

fn collection_with_name(env: &EnvironmentInfo, name: &str) -> Collection {
    let f: Vec<Collection> = env.collections
                                .clone()
                                .into_iter()
                                .filter({
                                    |i| i.name == name
                                })
                                .collect();
    assert!(f.len() == 1, format!("No collection matched {}", name));
    f.first().expect("Internal error: count=1, but no last!?").clone()
}

fn collection_with_id(env: &EnvironmentInfo, id: &str) -> Collection {
    let f: Vec<Collection> = env.collections
                                .clone()
                                .into_iter()
                                .filter({
                                    |i| i.collection_id == id
                                })
                                .collect();
    assert!(f.len() == 1, format!("No collection matched {}", id));
    f.last().expect("Internal error: count=1, but no last!?").clone()
}

fn configuration(env: &EnvironmentInfo,
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

fn print_env_children(env: &EnvironmentInfo) {
    let mut first = true;
    let configs: &Vec<Configuration> = &env.configurations;
    let collections: &Vec<Collection> = &env.collections;
    for conf in configs {
        if first {
            first = false;
            println!("   Configurations: {}", conf.name)
        } else {
            println!("                   {}", conf.name)
        }
    }
    first = true;
    for col in collections {
        let counts = col.document_counts.clone().unwrap();
        let formatted_counts = if counts.failed > 0 {
            format!("{} available, {} processing, {} failed",
                    counts.available,
                    counts.processing,
                    counts.failed)
        } else if counts.processing > 0 {
            format!("{} available, {} processing",
                    counts.available,
                    counts.processing)
        } else {
            format!("{} available", counts.available)
        };

        if first {
            first = false;
            println!("   Collections: {}, {}", col.name, formatted_counts)
        } else {
            println!("                {}, {}", col.name, formatted_counts)
        }
    }
}

fn show(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    for env_info in info.environments {
        let status = match env_info.environment.status {
            Status::Pending => " - pending",
            _ => {
                if env_info.environment.read_only {
                    " - read only"
                } else {
                    ""
                }
            }
        };
        let capacity = env_info.environment.index_capacity.as_ref();
        match capacity {
            Some(index_capacity) => {
                println!("Environment: {}, {} disk, {} memory{}",
                         env_info.environment.name,
                         index_capacity.disk_usage.total,
                         index_capacity.memory_usage.total,
                         status)
            }
            None => {
                println!("Environment: {}{}",
                         env_info.environment.name,
                         status);
            }
        }
        print_env_children(&env_info)
    }
}


// I suppose there is a standard library way to do this...
fn optional_string(s: &Option<&str>) -> Option<String> {
    match *s {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

fn create_environment(matches: &clap::ArgMatches) {
    let creds_file = matches.value_of("credentials").unwrap();
    let creds = credentials_from_file(creds_file).unwrap(); // FIXME

    let env_options = NewEnvironment {
        name: matches.value_of("name").unwrap().to_string(),
        description: optional_string(&matches.value_of("description")),
        size: matches.value_of("size").unwrap_or("0").parse::<u64>().unwrap(),
    };
    match environment::create(&creds, &env_options) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  create_environment response"))
        }
        Err(e) => println!("Failed to create environment {}", e),
    }
}

fn delete_environment(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_id = writable_environment(&info).environment.environment_id;

    match environment::delete(&info.creds, &env_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  delete_environment response"))
        }
        Err(e) => println!("Failed to delete environment {}", e),
    }
}

fn create_collection(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();

    let col_options = NewCollection {
        name: matches.value_of("name").unwrap().to_string(),
        description: optional_string(&matches.value_of("description")),
        configuration_id: match matches.value_of("configuration_id") {
            Some(s) => Some(s.to_string()),
            None => newest_configuration(&env_info).configuration_id,
        },
    };

    match collection::create(&info.creds, &env_id, &col_options) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  create_collection response"))
        }
        Err(e) => println!("Failed to create collection {}", e),
    }
}

fn select_collection(env_info: &EnvironmentInfo,
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

fn delete_collection(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    if matches.is_present("all") {
        assert!(false, "Deleting all collections is not yet implemented")
    }
    let collection_id = select_collection(&env_info, matches).collection_id;

    match collection::delete(&info.creds, &env_id, &collection_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  delete_collection response"))
        }
        Err(e) => println!("Failed to delete collection {}", e),
    }
}

fn create_configuration(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let config_filename =
        matches.value_of("configuration").unwrap().to_string();
    let config_file = std::fs::File::open(config_filename)
        .expect("Failed to read configuration JSON file");
    let config: Configuration = from_reader(config_file)
        .expect("Failed to parse configuration JSON");

    match configuration::create(&info.creds, &env_id, &config) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  create_collection response"))
        }
        Err(e) => println!("Failed to create collection {}", e),
    }
}

fn crawler_configuration(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);

    let collection = select_collection(&env_info, matches);
    let config = configuration(&env_info, &collection.configuration_id);

    println!("# discovery_service.conf \
              generated by https://github.com/bruceadams/wdsapi
{{
    environment_id   = {:?} # {}
    collection_id    = {:?} # {}
    configuration_id = {:?} # {}

    base_url = {:?}
    credentials {{
            username = {:?}
            password = {:?}
    }}

    api_version = \"2016-11-07\"
    check_for_completion = true
    concurrent_upload_connection_limit = 10
    http_timeout = 125
    send_stats {{
            jvm = true
            os  = true
    }}
}}
",
             env_info.environment.environment_id,
             env_info.environment.name,
             collection.collection_id,
             collection.name,
             collection.configuration_id,
             config.name,
             info.creds.url,
             info.creds.username,
             info.creds.password);
}

fn main() {
    rayon::initialize(rayon::Configuration::new().set_num_threads(64))
        .expect("Failed to initialize thread pool");

    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("show", Some(m)) => show(m),
        ("create-environment", Some(m)) => create_environment(m),
        ("delete-environment", Some(m)) => delete_environment(m),
        ("create-collection", Some(m)) => create_collection(m),
        ("delete-collection", Some(m)) => delete_collection(m),
        ("create-configuration", Some(m)) => create_configuration(m),
        ("crawler-configuration", Some(m)) => crawler_configuration(m),
        _ => println!("Not implemented yet; sorry!"),
    }
}
