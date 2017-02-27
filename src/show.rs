use chrono::Local;
use clap;
use info::discovery_service_info;
use rayon::prelude::*;
use select::{read_only_environment, select_collection, select_configuration,
             writable_environment};
use serde_json::ser::{to_string, to_string_pretty};
use wdsapi::collection;
use wdsapi::common::{ApiError, Credentials};
use wdsapi::configuration;
use wdsapi::document;
use wdsapi::document::DocumentStatus;
use wdsapi::environment;

pub fn show_environment(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
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

pub fn show_preview(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let configuration = select_configuration(&env_info, matches);

    if let Some(filename) = matches.value_of("filename") {
        println!("{} -> {}", Local::now().format("%T%.3f"), filename);
        match environment::preview(&info.creds,
                                   &env_id,
                                   &configuration.configuration_id.unwrap(),
                                   filename) {
            Ok(response) => {
                println!("{}",
                         to_string(&response)
                             .expect("Internal error: failed to format \
                                      environment::preview response"))
            }
            Err(e) => println!("Preview failed {}", e),
        }
    }
}

pub fn show_collection(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
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

pub fn show_configuration(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
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

pub fn show_document(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let collection = select_collection(&env_info, matches);

    // I didn't figure out how to use the matches directly...
    let document_ids: Vec<&str> = matches.values_of("document_id")
                                         .expect("Internal error: missing \
                                                  document_id")
                                         .collect();

    let document_statuses: Vec<Result<DocumentStatus, ApiError>> =
        document_ids.par_iter()
                    .map({
                        |document_id| {
                            document::detail(&info.creds,
                                             &env_id,
                                             &collection.collection_id,
                                             document_id)
                        }
                    })
                    .weight_max()
                    .collect();

    for doc in document_statuses {
        match doc {
            Ok(response) => {
                println!("{}",
                         to_string_pretty(&response)
                             .expect("Internal error: failed to format \
                                      document::detail response"))
            }
            Err(e) => println!("Failed to lookup document {}", e),
        }
    }
}
