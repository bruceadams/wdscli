use rayon::prelude::*;
use serde_json::Value;
use std::cmp::Ordering;
use std::thread::{JoinHandle, spawn};
use wdsapi::collection;
use wdsapi::common::Credentials;
use wdsapi::configuration;
use wdsapi::environment;

#[derive(Clone, Debug)]
pub struct EnvironmentInfo {
    pub environment_id: String,
    pub environment: Value,
    pub configurations: Vec<Value>,
    pub collections: Vec<Value>,
}

#[derive(Clone, Debug)]
pub struct DiscoveryServiceInfo {
    pub creds: Credentials,
    pub environments: Vec<EnvironmentInfo>,
}

fn created_ordering(av: &Value, bv: &Value) -> Ordering {
    let astr = av["created"].as_str().unwrap_or("");
    let bstr = bv["created"].as_str().unwrap_or("");
    astr.cmp(bstr)
}

fn configuration_array(creds: &Credentials, env_id: &str) -> Vec<Value> {
    let mut confs =
        configuration::list(creds, env_id)
            .expect("Failed to get configuration list")["configurations"]
            .as_array()
            .expect("Internal error: configurations is not a list?")
            .clone();
    confs.sort_by(created_ordering);
    confs
}


pub fn get_configurations_thread(creds: &Credentials,
                                 env_id: &str)
                                 -> JoinHandle<Vec<Value>> {
    let creds = creds.clone();
    let env_id = env_id.to_string();
    spawn(move || configuration_array(&creds, &env_id))
}

fn collection_array(creds: &Credentials, env_id: &str) -> Vec<Value> {
    let mut cols = collection::list(creds, env_id)
                       .expect("Failed to get collection list")["collections"]
                       .as_array()
                       .expect("Internal error: collections is not a list?")
                       .clone();
    cols.sort_by(created_ordering);
    cols
}

pub fn get_collections_thread(creds: &Credentials,
                              env_id: &str)
                              -> JoinHandle<Vec<Value>> {
    let creds = creds.clone();
    let env_id = env_id.to_string();
    spawn(move || {
        collection_array(&creds, &env_id)
            .par_iter()
            .map({
                |col| {
                    collection::detail(&creds,
                                       &env_id,
                                       col["collection_id"]
                                           .as_str()
                                           .expect("Internal error: missing \
                                                    collection_id"))
                        .expect("Failed to get collection detail, in thread")
                }
            })
            .collect()
    })
}

pub fn environment_info(creds: &Credentials, env: &Value) -> EnvironmentInfo {
    let env_id = env["environment_id"]
        .as_str()
        .expect("Internal error: missing environment_id");
    // launch threads for these API calls
    let conf_thread = get_configurations_thread(creds, env_id);
    let col_thread = get_collections_thread(creds, env_id);
    // Gather the results from the threads
    let configurations =
        conf_thread.join().expect("Failed to get configuration information");
    let collections = col_thread.join()
                                .expect("Failed to get collection information");
    EnvironmentInfo {
        environment_id: env_id.to_string(),
        environment: env.clone(),
        configurations: configurations,
        collections: collections,
    }
}

fn environment_array(creds: &Credentials) -> Vec<Value> {
    environment::list(creds)
        .expect("Failed to get environment list")["environments"]
        .as_array()
        .expect("Internal error: environments is not a list?")
        .clone()
}

pub fn discovery_service_info(creds: Credentials) -> DiscoveryServiceInfo {
    let environments = environment_array(&creds)
        .par_iter()
        .map({
            |env| environment_info(&creds, env)
        })
        .collect();
    DiscoveryServiceInfo {
        creds: creds,
        environments: environments,
    }
}
