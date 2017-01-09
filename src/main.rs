extern crate clap;
extern crate serde_json;
extern crate wdsapi;


use clap::{App, AppSettings, Arg, SubCommand};
use serde_json::ser::to_string_pretty;
use wdsapi::{Credentials, NewCollection, NewEnvironment, create_collection,
             create_environment, credentials_from_file, get_collection_detail,
             get_collections, get_configurations, get_environments};

fn get_and_print_env_children(creds: &Credentials, env_id: &str) {
    match get_configurations(creds, env_id) {
        Ok(configs) => {
            let mut first = true;
            for conf in configs.configurations {
                if first {
                    first = false;
                    println!("   Configurations: {}", conf.name)
                } else {
                    println!("                   {}", conf.name)
                }
            }
        }
        Err(e) => println!("   Failed to get configurations {}", e),
    }

    match get_collections(creds, env_id) {
        Ok(cols) => {
            let mut first = true;
            for col in cols.collections {
                match get_collection_detail(creds, env_id, &col.collection_id) {
                    Ok(detail) => {
                        let counts = detail.document_counts.unwrap();
                        let formatted_counts = if counts.failed > 0 {
                            format!("{} available, {} processing, {} failed",
                                    counts.available,
                                    counts.processing,
                                    counts.failed)
                        } else if counts.processing >
                                                         0 {
                            format!("{} available, {} processing",
                                    counts.available,
                                    counts.processing)
                        } else {
                            format!("{} available", counts.available)
                        };

                        if first {
                            first = false;
                            println!("   Collections: {}, {}",
                                     detail.name,
                                     formatted_counts)
                        } else {
                            println!("                {}, {}",
                                     detail.name,
                                     formatted_counts)
                        }
                    }
                    Err(e) => {
                        println!("   Failed to get collection detail {}", e)
                    }
                }
            }
        }
        Err(e) => println!("   Failed to get collections {}", e),
    }
}

fn show(matches: &clap::ArgMatches) {
    let creds_file = matches.value_of("credentials").unwrap();
    let creds = credentials_from_file(creds_file).unwrap(); // FIXME

    match get_environments(&creds) {
        Ok(envs) => {
            for env in envs.environments {
                match env.index_capacity {
                    Some(index_capacity) => {
                        println!("Environment: {}, {} disk, {} memory",
                                 env.name,
                                 index_capacity.disk_usage.total,
                                 index_capacity.memory_usage.total)
                    }
                    None => {
                        println!("Environment: {}", env.name);
                    }
                }
                get_and_print_env_children(&creds, &env.environment_id)
            }
        }
        Err(e) => println!("Failed to get environments {}", e),
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

fn ccol(matches: &clap::ArgMatches) {
    let creds_file = matches.value_of("credentials").unwrap();
    let creds = credentials_from_file(creds_file).unwrap(); // FIXME
    let col_options = NewCollection {
        name: matches.value_of("name").unwrap().to_string(),
        description: optional_string(&matches.value_of("description")),
        configuration_id:
            optional_string(&matches.value_of("configuration_id")),
    };
    let envs = get_environments(&creds)
        .expect("Failed to get environment information")
        .environments;
    // The following code should be refactored into helper functions
    let env_id = envs.into_iter()
                     .filter({
                         |env| !env.read_only
                     })
                     .last()
                     .expect("No writable environment found")
                     .environment_id;
    match create_collection(&creds, &env_id, &col_options) {
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
    let creds_file = matches.value_of("credentials").unwrap();
    let creds = credentials_from_file(creds_file).unwrap(); // FIXME
    let envs = get_environments(&creds)
        .expect("Failed to get environment information")
        .environments;
    // The following code should be refactored into helper functions
    let environment = envs.into_iter()
                          .filter({
                              |env| !env.read_only
                          })
                          .last()
                          .expect("No writable environment found");
    let collections = get_collections(&creds, &environment.environment_id)
        .expect("Failed to get collection information")
        .collections;
    let collection = collections.into_iter()
                                .last()
                                .expect("No collection found");
    println!("# discovery_service.conf \
              generated by https://github.com/bruceadams/wdsapi
{{
    environment_id   = \
              {:?} # {}
    collection_id    = {:?} # {}
    \
              configuration_id = {:?}
    base_url = {:?}
    credentials {{
            \
              username = {:?}
            password = {:?}
    }}

    \
              api_version = \"2016-11-07\"
    \
              concurrent_upload_connection_limit = 2
    http_timeout = 125
    \
              send_stats {{
            jvm = true
            os  = true
    \
              }}
}}
",
             environment.environment_id,
             environment.name,
             collection.collection_id,
             collection.name,
             collection.configuration_id,
             creds.url,
             creds.username,
             creds.password);
}

fn main() {
    let matches = App::new("wdscli")
        .about("Basic administration for Watson Discovery Service.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("show")
            .about("Displays information about existing resources.")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials."))
            .arg(Arg::with_name("guid")
                .short("g")
                .help("Displays the GUID for each resource.")))
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
            .visible_alias("ce")
            .about("Delete an environment")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials.")))
        .subcommand(SubCommand::with_name("create-collection")
            .visible_alias("cl")
            .about("Create a new collection")
            .arg(Arg::with_name("credentials")
                .required(true)
                .help("A JSON file containing service credentials."))
            .arg(Arg::with_name("name")
                .required(true)
                .help("The name of the collection."))
            .arg(Arg::with_name("purge")
                .short("p")
                .help("Remove all existing collections before creating the \
                       new one."))
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
        ("create-collection", Some(m)) => ccol(m),
        ("crawler-configuration", Some(m)) => crawler(m),
        _ => println!("Not implemented yet; sorry!"),
    }

    // let serialized =
    // serde_json::to_string_pretty(&deserialized).unwrap();
}
