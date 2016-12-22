#[macro_use]
extern crate clap;
extern crate wdsapi;

use wdsapi::{Credentials, credentials_from_file, get_collection_detail,
             get_collections, get_configurations, get_envs};

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

    match get_envs(&creds) {
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

// The user wants to take an action.
// - get info: environments, collections, configurations, statuses,
// sizes
// - new environment: that is: new writable environment.
// - new configuration: create a new configuration, or replace an
// existing one with the same name.
// - new collection: removing any existing, writable collections.
// Will default to using the most recently updated configuration.
// - query: If there are any writable environments,
//          query each collection in each writable environment.
//          (Typically there is only one.)
//          If there is no writable environment,
//          query each collection in each read only environment.
//          (Again, typically there is only one.)

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Bruce Adams <ba@us.ibm.com>")
        (about: "Basic administration for Watson Discovery Service.")
        (@subcommand show =>
            (about: "Displays information about existing resources.")
            (@arg credentials: +required
                "A JSON file containing service credentials.")
            (@arg guid: -g --guid "Displays the GUID for each resource."))
        (@subcommand env =>
            (about: "Create a writable environment")
            (@arg credentials: +required
                "A JSON file containing service credentials.")
            (@arg name: +required "The name of the environment.")
            (@arg size: -s --size +takes_value
                "The size environment to create.")
            (@arg description: -d --desc --description +takes_value
                "Description text for the environment."))
        (@subcommand config =>
            (about: "Create a new configuration,
                     or replace existing configuration with the same name.")
            (@arg credentials: +required
                "A JSON file containing service credentials.")
            (@arg configuration: +required
                "File containing the configuration JSON."))
        (@subcommand col =>
            (about: "Create a new collection")
            (@arg credentials: +required
                "A JSON file containing service credentials.")
            (@arg name: +required "The name of the collection.")
            (@arg purge: -p --purge "Remove all existing collections.")
            (@arg description: -d --desc --description +takes_value
                "Description text for the collection."))
        (@subcommand query =>
            (about: "Issue a query.")
            (@arg credentials: +required
                "A JSON file containing service credentials.")
            (@arg query: "The query string.")
            (@arg count: -n --count +takes_value
                "The number of results to return. Defaults to 2.")
            (@arg collection: -c --collection +takes_value
                "The name of the collection to query.
                 Defaults to every writable collection.")))
                      .get_matches();

    match matches.subcommand() {
        ("show", Some(show_matches)) => show(show_matches),
        ("env", Some(env_matches)) => println!("env is not yet implemented."),
        ("config", Some(config_matches)) => {
            println!("config is not yet implemented.")
        }
        ("col", Some(col_matches)) => println!("col is not yet implemented."),
        ("query", Some(query_matches)) => {
            println!("query is not yet implemented.")
        }
        _ => println!("no command is not yet implemented."),
    }


    // let serialized =
    // serde_json::to_string_pretty(&deserialized).unwrap();
}
