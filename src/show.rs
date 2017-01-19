use clap;
use info::discovery_service_info;
use select::{read_only_environment, select_collection, select_configuration,
             writable_environment};
use serde_json::ser::to_string_pretty;
use wdsapi::collection;
use wdsapi::configuration;
use wdsapi::document;
use wdsapi::environment;

pub fn show_environment(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_id = if matches.is_present("read-only") {
        read_only_environment(&info).environment.environment_id
    } else {
        writable_environment(&info).environment.environment_id
    };

    match environment::detail(&info.creds, &env_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  environment::detail response"))
        }
        Err(e) => println!("Failed to lookup environment {}", e),
    }
}

pub fn show_collection(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let collection = select_collection(&env_info, matches);

    match collection::detail(&info.creds, &env_id, &collection.collection_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  collection::detail response"))
        }
        Err(e) => println!("Failed to lookup collection {}", e),
    }
}

pub fn show_configuration(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let configuration = select_configuration(&env_info, matches);

    match configuration::detail(&info.creds,
                                &env_id,
                                &configuration.configuration_id
                                .expect("Internal error: missing configuration_id")) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  configuration::detail response"))
        }
        Err(e) => println!("Failed to lookup configuration {}", e),
    }
}

pub fn show_document(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let collection = select_collection(&env_info, matches);
    let document_id = matches.value_of("document_id")
                             .expect("Internal error: missing document_id");

    match document::detail(&info.creds,
                           &env_id,
                           &collection.collection_id,
                           document_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  document::detail response"))
        }
        Err(e) => println!("Failed to lookup document {}", e),
    }
}
