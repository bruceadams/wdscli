use rayon::prelude::*;
use std::thread::{JoinHandle, spawn};
use wdsapi::collection;
use wdsapi::collection::Collection;
use wdsapi::common::Credentials;
use wdsapi::configuration;
use wdsapi::configuration::Configuration;
use wdsapi::environment;
use wdsapi::environment::Environment;

#[derive(Clone, Debug)]
pub struct EnvironmentInfo {
    pub environment: Environment,
    pub configurations: Vec<Configuration>,
    pub collections: Vec<Collection>,
}

#[derive(Clone, Debug)]
pub struct DiscoveryServiceInfo {
    pub creds: Credentials,
    pub environments: Vec<EnvironmentInfo>,
}


pub fn get_configurations_thread(creds: &Credentials,
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

pub fn get_collections_thread(creds: &Credentials,
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

pub fn environment_info(creds: &Credentials,
                        env: &Environment)
                        -> EnvironmentInfo {
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

pub fn discovery_service_info(creds: Credentials) -> DiscoveryServiceInfo {
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
