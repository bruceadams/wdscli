use chrono::Local;
use clap;
use info::discovery_service_info;
use rayon::prelude::*;
use select::{read_only_environment, select_collection, select_configuration,
             writable_environment};
use serde_json::{Value, to_string, to_string_pretty};
use wdsapi::collection;
use wdsapi::common::{ApiError, Credentials};
use wdsapi::configuration;
use wdsapi::document;
use wdsapi::environment;

pub fn show_environment(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
    let env_id = if matches.is_present("read-only") {
        read_only_environment(&info).environment_id
    } else {
        writable_environment(&info).environment_id
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
    let configuration = select_configuration(&env_info, matches);
    let env_id = env_info.environment_id;

    if let Some(filename) = matches.value_of("filename") {
        println!("{} -> {}", Local::now().format("%T%.3f"), filename);
        match environment::preview(&info.creds,
                                   &env_id,
                                   configuration["configuration_id"].as_str(),
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
    let collection = select_collection(&env_info, matches);
    let env_id = env_info.environment_id;

    match collection::detail(&info.creds,
                             &env_id,
                             collection["collection_id"]
                                 .as_str()
                                 .unwrap_or("")) {
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
    let configuration = select_configuration(&env_info, matches);
    let env_id = env_info.environment_id;

    match configuration::detail(&info.creds,
                                &env_id,
                                configuration["configuration_id"]
                                    .as_str()
                                    .expect("Internal error: missing \
                                             configuration_id")) {
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
    let collection = select_collection(&env_info, matches);
    let env_id = env_info.environment_id;

    // I didn't figure out how to use the matches directly...
    let document_ids: Vec<&str> = matches.values_of("document_id")
                                         .expect("Internal error: missing \
                                                  document_id")
                                         .collect();

    let document_statuses: Vec<Result<Value, ApiError>> =
        document_ids.par_iter()
                    .map({
                        |document_id| {
                document::detail(&info.creds,
                                 &env_id,
                                 collection["collection_id"]
                                     .as_str()
                                     .unwrap_or(""),
                                 document_id)
            }
                    })
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
