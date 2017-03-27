use clap;
use info::discovery_service_info;
use select::{select_collection, select_configuration, writable_environment};
use serde_json::ser::to_string_pretty;
use wdsapi::collection;
use wdsapi::common::Credentials;
use wdsapi::configuration;
use wdsapi::environment;

pub fn delete_environment(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
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

pub fn delete_collection(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
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

pub fn delete_one_configuration(creds: &Credentials,
                                env_id: &str,
                                configuration_id: &str) {
    match configuration::delete(creds, env_id, configuration_id) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  delete_configuration response"))
        }
        Err(e) => println!("Failed to delete configuration {}", e),
    }
}

pub fn delete_configuration(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    if matches.is_present("all") {
        for configuration in env_info.configurations {
            let configuration_id =
                configuration.configuration_id
                             .expect("Internal error: missing \
                                      configuration_id");
            delete_one_configuration(&info.creds, &env_id, &configuration_id)
        }
    } else {
        let configuration = select_configuration(&env_info, matches);
        let configuration_id = configuration.configuration_id
                                            .expect("Internal error: missing \
                                                     configuration_id");
        delete_one_configuration(&info.creds, &env_id, &configuration_id)
    }

}
