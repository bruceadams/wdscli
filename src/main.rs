extern crate clap;
extern crate serde_json;
extern crate wdsapi;
extern crate rayon;

mod cli;
mod create;
mod delete;
mod info;
mod select;
mod show;

use create::{create_collection, create_configuration, create_environment};
use delete::{delete_collection, delete_environment};
use info::{EnvironmentInfo, discovery_service_info};
use select::{configuration_with_id, select_collection, writable_environment};
use show::{show_collection, show_configuration, show_environment};

use wdsapi::collection::Collection;
use wdsapi::common::Status;
use wdsapi::configuration::Configuration;

fn print_env_children(env: &EnvironmentInfo, guid: bool) {
    let mut first = true;
    let configs: &Vec<Configuration> = &env.configurations;
    let collections: &Vec<Collection> = &env.collections;
    for conf in configs {
        if first {
            first = false;
            println!("   Configurations: {}", conf.name)
        } else {
            println!("                   {}", conf.name)
        }
        if guid {
            println!("                   {}\n",
                     conf.configuration_id
                         .clone()
                         .unwrap_or("missing configuration_id".to_string()));
        };
    }
    first = true;
    for col in collections {
        let counts = col.document_counts.clone().unwrap();
        let formatted_counts = if counts.failed > 0 {
            format!("{} available, {} processing, {} failed",
                    counts.available,
                    counts.processing,
                    counts.failed)
        } else if counts.processing > 0 {
            format!("{} available, {} processing",
                    counts.available,
                    counts.processing)
        } else {
            format!("{} available", counts.available)
        };

        if first {
            first = false;
            println!("   Collections: {}, {}", col.name, formatted_counts)
        } else {
            println!("                {}, {}", col.name, formatted_counts)
        }
        if guid {
            println!("                {}\n", col.collection_id);
        };
    }
}

fn show(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let guid = matches.is_present("guid");

    for env_info in info.environments {
        let status = match env_info.environment.status {
            Status::Pending => " - pending",
            _ => {
                if env_info.environment.read_only {
                    " - read only"
                } else {
                    ""
                }
            }
        };
        let capacity = env_info.environment.index_capacity.as_ref();
        match capacity {
            Some(index_capacity) => {
                println!("\nEnvironment: {}, {} disk, {} memory{}",
                         env_info.environment.name,
                         index_capacity.disk_usage.total,
                         index_capacity.memory_usage.total,
                         status)
            }
            None => {
                println!("\nEnvironment: {}{}",
                         env_info.environment.name,
                         status);
            }
        }
        if guid {
            println!("             {}\n", env_info.environment.environment_id);
        };
        print_env_children(&env_info, guid)
    }
}

fn crawler_configuration(matches: &clap::ArgMatches) {
    let info = discovery_service_info(matches);
    let env_info = writable_environment(&info);

    let collection = select_collection(&env_info, matches);
    let config = configuration_with_id(&env_info, &collection.configuration_id);

    println!("# discovery_service.conf \
              generated by https://github.com/bruceadams/wdsapi
{{
    environment_id   = {:?} # {}
    collection_id    = {:?} # {}
    configuration_id = {:?} # {}

    base_url = {:?}
    credentials {{
            username = {:?}
            password = {:?}
    }}

    api_version = \"2016-11-07\"
    check_for_completion = true
    concurrent_upload_connection_limit = 10
    http_timeout = 125
    send_stats {{
            jvm = true
            os  = true
    }}
}}
",
             env_info.environment.environment_id,
             env_info.environment.name,
             collection.collection_id,
             collection.name,
             collection.configuration_id,
             config.name,
             info.creds.url,
             info.creds.username,
             info.creds.password);
}

fn main() {
    rayon::initialize(rayon::Configuration::new().set_num_threads(64))
        .expect("Failed to initialize thread pool");

    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("overview", Some(m)) => show(m),
        ("create-environment", Some(m)) => create_environment(m),
        ("create-collection", Some(m)) => create_collection(m),
        ("create-configuration", Some(m)) => create_configuration(m),
        ("delete-environment", Some(m)) => delete_environment(m),
        ("delete-collection", Some(m)) => delete_collection(m),
        ("show-environment", Some(m)) => show_environment(m),
        ("show-collection", Some(m)) => show_collection(m),
        ("show-configuration", Some(m)) => show_configuration(m),
        ("crawler-configuration", Some(m)) => crawler_configuration(m),
        _ => println!("Not implemented yet; sorry!"),
    }
}
