use chrono::Local;
use clap;
use info::discovery_service_info;
use select::{newest_configuration, select_collection, writable_environment};

use serde_json::de::from_reader;
use serde_json::ser::{to_string, to_string_pretty};
use std;

use wdsapi::collection;
use wdsapi::collection::NewCollection;
use wdsapi::common::credentials_from_file;
use wdsapi::configuration;
use wdsapi::configuration::Configuration;
use wdsapi::document;
use wdsapi::environment;
use wdsapi::environment::NewEnvironment;

// I suppose there is a standard library way to do this...
fn optional_string(s: &Option<&str>) -> Option<String> {
    match *s {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

pub fn create_environment(matches: &clap::ArgMatches) {
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

pub fn create_collection(matches: &clap::ArgMatches) {
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

pub fn create_configuration(matches: &clap::ArgMatches) {
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

pub fn add_document(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let collection = select_collection(&env_info, matches);

    for filename in matches.values_of("filenames").unwrap() {
        println!("{} -> {}", Local::now().format("%T%.3f"), filename);
        match document::create(&info.creds,
                               &env_id,
                               &collection.collection_id,
                               filename) {
            Ok(response) => {
                println!("{}",
                         to_string(&response)
                             .expect("Internal error: failed to format \
                                      document::create response"))
            }
            Err(e) => println!("Failed to create document {}", e),
        }
    }
}
