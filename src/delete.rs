use clap;
use info::discovery_service_info;
use select::{select_collection, select_configuration, writable_environment};
use serde_json::ser::to_string_pretty;
use wdsapi::collection;
use wdsapi::common::Credentials;
use wdsapi::configuration;
use wdsapi::environment;

pub fn delete_environment(matches: &clap::ArgMatches) {
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

pub fn delete_one_collection(creds: &Credentials,
                             env_id: &str,
                             collection_id: &str) {
    match collection::delete(creds, env_id, collection_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  delete_collection response"))
        }
        Err(e) => println!("Failed to delete collection {}", e),
    }
}

pub fn delete_collection(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    if matches.is_present("all") {
        for collection in env_info.collections {
            delete_one_collection(&info.creds,
                                  &env_id,
                                  &collection.collection_id)
        }
    } else {
        let collection = select_collection(&env_info, matches);
        delete_one_collection(&info.creds, &env_id, &collection.collection_id)
    }
}

pub fn delete_configuration(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let configuration = select_configuration(&env_info, matches);
    let configuration_id = configuration.configuration_id
                                        .expect("Internal error: missing \
                                                 configuration_id");
    match configuration::delete(&info.creds, &env_id, &configuration_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  delete_configuration response"))
        }
        Err(e) => println!("Failed to delete configuration {}", e),
    }
}
