extern crate clap;
extern crate serde_json;
extern crate wdsapi;
extern crate rayon;

use clap::{App, AppSettings, Arg, SubCommand};
use rayon::prelude::*;
use serde_json::de::from_reader;
use serde_json::ser::to_string_pretty;
use std::thread::{JoinHandle, spawn};
use wdsapi::{Collection, Configuration, Credentials, Environment,
             NewCollection, NewEnvironment, Status, create_collection,
             create_configuration, create_environment, credentials_from_file,
             delete_environment, get_collection_detail, get_collections,
             get_configurations, get_environments};

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
        get_configurations(&creds, &env_id)
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
        get_collections(&creds, &env_id)
            .expect("Failed to get collection information, in thread")
            .collections
            .par_iter()
            .map({
                |col| {
                    get_collection_detail(&creds, &env_id, &col.collection_id)
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
    let environments = get_environments(&creds)
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
    let mut newest = env.configurations
                        .clone()
                        .into_iter()
                        .last()
                        .expect("No configuration found");
    for conf in env.configurations.clone() {
        if conf.created > newest.created {
            newest = conf
        }
    }
    newest
}


fn newest_collection(env: &EnvironmentInfo) -> Collection {
    let mut newest = env.collections
                        .clone()
                        .into_iter()
                        .last()
                        .expect("No collection found");
    for col in env.collections.clone() {
        if col.created > newest.created {
            newest = col
        }
    }
    newest
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
    for env in info.environments {
        let status = match env.environment.status {
            Status::Pending => " - pending",
            _ => {
                if env.environment.read_only {
                    " - read only"
                } else {
                    ""
                }
            }
        };
        let capacity = env.environment.index_capacity.as_ref();
        match capacity {
            Some(index_capacity) => {
                println!("Environment: {}, {} disk, {} memory{}",
                         env.environment.name,
                         index_capacity.disk_usage.total,
                         index_capacity.memory_usage.total,
                         status)
            }
            None => {
                println!("Environment: {}{}", env.environment.name, status);
            }
        }
        print_env_children(&env)
    }
}


// I suppose there is a standard library way to do this...
fn optional_string(s: &Option<&str>) -> Option<String> {
    match *s {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

fn cenv(matches: &clap::ArgMatches) {
    let creds_file = matches.value_of("credentials").unwrap();
    let creds = credentials_from_file(creds_file).unwrap(); // FIXME

    let env_options = NewEnvironment {
        name: matches.value_of("name").unwrap().to_string(),
        description: optional_string(&matches.value_of("description")),
        size: matches.value_of("size").unwrap_or("0").parse::<u64>().unwrap(),
    };
    match create_environment(&creds, &env_options) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  create_environment response"))
        }
        Err(e) => println!("Failed to create environment {}", e),
    }
}

fn denv(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_id = writable_environment(&info).environment.environment_id;

    match delete_environment(&info.creds, &env_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  delete_environment response"))
        }
        Err(e) => println!("Failed to delete environment {}", e),
    }
}

fn ccol(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env = writable_environment(&info);
    let env_id = env.environment.environment_id.clone();

    let col_options = NewCollection {
        name: matches.value_of("name").unwrap().to_string(),
        description: optional_string(&matches.value_of("description")),
        configuration_id: match matches.value_of("configuration_id") {
            Some(s) => Some(s.to_string()),
            None => newest_configuration(&env).configuration_id,
        },
    };

    match create_collection(&info.creds, &env_id, &col_options) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  create_collection response"))
        }
        Err(e) => println!("Failed to create collection {}", e),
    }
}

fn cconfig(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env = writable_environment(&info);
    let env_id = env.environment.environment_id.clone();
    let config_filename =
        matches.value_of("configuration").unwrap().to_string();
    let config_file = std::fs::File::open(config_filename)
        .expect("Failed to read configuration JSON file");
    let config: Configuration = from_reader(config_file)
        .expect("Failed to parse configuration JSON");

    match create_configuration(&info.creds, &env_id, &config) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  create_collection response"))
        }
        Err(e) => println!("Failed to create collection {}", e),
    }
}

fn crawler(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env = writable_environment(&info);

    let collection = newest_collection(&env);
    let config = configuration(&env, &collection.configuration_id);

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
    concurrent_upload_connection_limit = 2
    http_timeout = 125
    send_stats {{
            jvm = true
            os  = true
    }}
}}
",
             env.environment.environment_id,
             env.environment.name,
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

    let matches = App::new("wdscli")
        .about("Basic administration for Watson Discovery Service.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("show")
            .about("Displays information about existing resources.")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials.")))
            // .arg(Arg::with_name("guid")
            //     .short("g")
            //     .help("Displays the GUID for each resource.")))
        .subcommand(SubCommand::with_name("create-environment")
            .visible_alias("ce")
            .about("Create a writable environment")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials."))
            .arg(Arg::with_name("name")
                .required(true)
                .help("The name of the environment."))
            .arg(Arg::with_name("size")
                .short("s")
                .takes_value(true)
                .help("The size environment to create."))
            .arg(Arg::with_name("description")
                .short("d")
                .takes_value(true)
                .help("Description text for the environment.")))
        .subcommand(SubCommand::with_name("delete-environment")
            .visible_alias("de")
            .about("Delete the writable environment")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials.")))
        .subcommand(SubCommand::with_name("create-collection")
            .visible_alias("cl")
            .about("Create a new collection using the most recently created \
                    configuration")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials."))
            .arg(Arg::with_name("name")
                .required(true)
                .help("The name of the collection."))
            .arg(Arg::with_name("description")
                .short("d")
                .short("desc")
                .takes_value(true)
                .help("Description text for the collection.")))
        .subcommand(SubCommand::with_name("crawler-configuration")
            .visible_alias("cc")
            .about("Print out crawler configuration.")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials.")))
        .subcommand(SubCommand::with_name("create-configuration")
            .visible_alias("cn")
            .about("Create a new configuration, or replace existing \
                    configuration with the same name.")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials."))
            .arg(Arg::with_name("configuration")
                .required(true)
                .help("File containing the configuration JSON.")))
        .subcommand(SubCommand::with_name("query")
            .visible_alias("q")
            .about("Issue a query.")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials."))
            .arg(Arg::with_name("query").help("The query string."))
            .arg(Arg::with_name("count")
                .short("n")
                .takes_value(true)
                .help("The number of results to return. Defaults to 2."))
            .arg(Arg::with_name("collection")
                .short("c")
                .takes_value(true)
                .help("The name of the collection to query. \
                       Defaults to every collection.")))
        .get_matches();

    match matches.subcommand() {
        ("show", Some(m)) => show(m),
        ("create-environment", Some(m)) => cenv(m),
        ("delete-environment", Some(m)) => denv(m),
        ("create-collection", Some(m)) => ccol(m),
        ("create-configuration", Some(m)) => cconfig(m),
        ("crawler-configuration", Some(m)) => crawler(m),
        _ => println!("Not implemented yet; sorry!"),
    }
}
