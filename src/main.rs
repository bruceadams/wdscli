#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate futures;
extern crate hyper;
extern crate rayon;
extern crate serde_json;
extern crate swagger;
extern crate walkdir;
extern crate watson_discovery_api;
extern crate wdsapi;

mod add;
mod cli;
mod create;
mod delete;
mod info;
mod query;
mod select;
mod show;

use add::add_document;
use create::{create_collection, create_configuration, create_environment};
use delete::{delete_collection, delete_configuration, delete_document,
             delete_environment};
use info::{EnvironmentInfo, discovery_service_info};
use query::{notices, query};
use select::{configuration_with_id, select_collection, writable_environment};
use serde_json::Value;
use show::{show_collection, show_configuration, show_document,
           show_environment, show_preview};
use std::io::stdout;

use wdsapi::common::{ApiError, Credentials, credentials_from_file};

fn print_env_children(env: &EnvironmentInfo, guid: bool) {
    let mut first = true;
    let configs: &Vec<Value> = &env.configurations;
    let collections: &Vec<Value> = &env.collections;
    for conf in configs {
        if first {
            first = false;
            println!("   Configurations: {}", conf["name"])
        } else {
            println!("                   {}", conf["name"])
        }
        if guid {
            println!(
                "                   {}\n",
                conf["configuration_id"].as_str().unwrap_or(
                    "missing configuration_id",
                )
            );
        };
    }
    first = true;
    for col in collections {
        let counts = &col["document_counts"];
        let config = configuration_with_id(
            env,
            col["configuration_id"].as_str().unwrap_or(""),
        );

        let formatted_counts =
            if counts["processing"].as_u64().unwrap_or(0) > 0 &&
                counts["failed"].as_u64().unwrap_or(9) > 0
            {
                format!(
                    "{} available, {} processing, {} failed",
                    counts["available"],
                    counts["processing"],
                    counts["failed"]
                )
            } else if counts["processing"].as_u64().unwrap_or(0) > 0 {
                format!(
                    "{} available, {} processing",
                    counts["available"],
                    counts["processing"]
                )
            } else if counts["failed"].as_u64().unwrap_or(9) > 0 {
                format!(
                    "{} available, {} failed",
                    counts["available"],
                    counts["failed"]
                )
            } else {
                format!("{} available", counts["available"])
            };

        if first {
            first = false;
            println!(
                "   Collections: {} ↳ {}, {}",
                col["name"],
                config["name"],
                formatted_counts
            )
        } else {
            println!(
                "                {} ↳ {}, {}",
                col["name"],
                config["name"],
                formatted_counts
            )
        }
        if guid {
            println!("                {}\n", col["collection_id"]);
        };
    }
}

fn show(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(&creds);
    let guid = matches.is_present("guid");

    for env_info in info.environments {
        let status = match env_info.environment["status"].as_str() {
            Some("pending") => " - pending",
            _ => {
                if env_info.environment["read_only"].as_bool() == Some(true) {
                    " - read only"
                } else {
                    ""
                }
            }
        };
        let capacity = env_info.environment["index_capacity"].as_object();
        match capacity {
            Some(index_capacity) => {
                println!(
                    "\nEnvironment: {}, size={}, {:.0}% of {} disk, \
                          {:.0}% of {} memory{}",
                    env_info.environment["name"],
                    env_info.environment["size"].as_u64().unwrap_or(999),
                    index_capacity["disk_usage"]["percent_used"]
                        .as_f64()
                        .unwrap_or(0.0),
                    index_capacity["disk_usage"]["total"].as_str().unwrap_or(
                        "?",
                    ),
                    index_capacity["memory_usage"]["percent_used"]
                        .as_f64()
                        .unwrap_or(0.0),
                    index_capacity["memory_usage"]["total"].as_str().unwrap_or(
                        "?",
                    ),
                    status
                )
            }
            None => {
                println!(
                    "\nEnvironment: {}{}",
                    env_info.environment["name"],
                    status
                );
            }
        }
        if guid {
            println!("             {}\n", env_info.environment_id);
        };
        print_env_children(&env_info, guid)
    }
}

fn crawler_configuration(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(&creds);
    let env_info = writable_environment(&info);

    let collection = select_collection(&env_info, matches);
    let config = configuration_with_id(
        &env_info,
        collection["configuration_id"].as_str().unwrap_or(""),
    );

    println!(
        "# discovery_service.conf \
              generated by https://github.com/bruceadams/wdsapi

environment_id   = {:?} # {}
collection_id    = {} # {}
                 # configuration name is {}

base_url = {:?}
credentials {{
        username = {:?}
        password = {:?}
}}

api_version = \"2017-02-01\"
check_for_completion = true
concurrent_upload_connection_limit = 10
http_timeout = 125
send_stats {{ jvm = true, os = true }}
uri_tracking {{ include \"uri_tracking_storage.conf\" }}",
        env_info.environment_id,
        env_info.environment["name"],
        collection["collection_id"],
        collection["name"],
        config["name"],
        info.creds.url,
        info.creds.username,
        info.creds.password
    );
}

fn generate_completions(matches: &clap::ArgMatches) {
    let shell_name = matches.value_of("shell").unwrap();

    let shell: clap::Shell = shell_name.parse().expect(&format!(
        "Generating completions for \"{}\" is \
                                    not supported.",
        shell_name
    ));

    // The completions generated here have some issues.
    // FIXME adjust the output: something like `sed 's/"<credentials>"//g'`
    let mut my_cli = cli::build_cli();
    let my_name = my_cli.get_name().to_string();
    my_cli.gen_completions_to(my_name, shell, &mut stdout());
}

fn main() {
    rayon::initialize(rayon::Configuration::new().num_threads(64))
        .expect("Failed to initialize thread pool");

    let matches = cli::build_cli().get_matches();
    // Just a few commands do not need credentials.
    match matches.subcommand() {
        ("generate-completions", Some(m)) => generate_completions(m),
        _ => subcommand_needing_credentials(&matches),
    }
}

fn subcommand_needing_credentials(matches: &clap::ArgMatches) {
    let default_file = match std::env::var("WDSCLI_CREDENTIALS_FILE") {
        Ok(filename) => filename,
        Err(_) => "credentials.json".to_string(),
    };
    let creds_file = match matches.value_of("credentials") {
        Some(filename) => filename,
        None => &default_file,
    };

    match credentials_from_file(creds_file) {
        Ok(creds) => {
            match matches.subcommand() {
                ("overview", Some(m)) => show(creds, m),
                ("query", Some(m)) => query(creds, m),
                ("notices", Some(m)) => notices(creds, m),
                ("preview", Some(m)) => show_preview(creds, m),
                ("create-environment", Some(m)) => {
                    create_environment(&creds, m)
                }
                ("create-collection", Some(m)) => create_collection(creds, m),
                ("create-configuration", Some(m)) => {
                    create_configuration(creds, m)
                }
                ("delete-environment", Some(m)) => delete_environment(creds, m),
                ("delete-collection", Some(m)) => delete_collection(creds, m),
                ("delete-document", Some(m)) => delete_document(creds, m),
                ("delete-configuration", Some(m)) => {
                    delete_configuration(creds, m)
                }
                ("show-environment", Some(m)) => show_environment(creds, m),
                ("show-collection", Some(m)) => show_collection(creds, m),
                ("show-configuration", Some(m)) => show_configuration(creds, m),
                ("show-document", Some(m)) => show_document(creds, m),
                ("add-document", Some(m)) => add_document(creds, m),
                ("crawler-configuration", Some(m)) => {
                    crawler_configuration(creds, m)
                }
                _ => {
                    println!("Not implemented yet; sorry!");
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            match e {
                ApiError::Io(e) => {
                    println!("Failed to read {}: {}", creds_file, e)
                }
                ApiError::SerdeJson(e) => {
                    println!("Invalid credentials in {}: {}", creds_file, e)
                }
                _ => println!("Unexpected error reading {}: {}", creds_file, e),
            };
            std::process::exit(1);
        }
    }
}
