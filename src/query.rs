use clap;
use info::discovery_service_info;
use select::{read_only_environment, select_collection, writable_environment};
use serde_json::ser::to_string_pretty;
use wdsapi::common::{Credentials, QueryParams};
use wdsapi::query;

fn query_params(
    matches: &clap::ArgMatches,
    default_count: &str,
) -> QueryParams {
    QueryParams {
        filter: matches.value_of("filter").map(|s| s.to_string()),
        query: matches.value_of("query").map(|s| s.to_string()),
        natural_language_query:
            matches.value_of("natural_language_query").map(
                |s| s.to_string(),
            ),
        passages: Some(matches.is_present("passages")),
        aggregation: matches.value_of("aggregation").map(|s| s.to_string()),
        count: matches.value_of("count")
                      .unwrap_or(default_count)
                      .parse::<u64>()
                      .unwrap(),
        return_hierarchy: matches.value_of("return_hierarchy").map(
            |s| s.to_string(),
        ),
        offset: matches.value_of("offset").map(
            |s| s.parse::<u64>().unwrap(),
        ),
        sort: matches.value_of("sort").map(|s| s.to_string()),
    }
}

pub fn query(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(&creds);
    let env_info = if matches.is_present("read-only") {
        read_only_environment(&info)
    } else {
        writable_environment(&info)
    };
    let collection = select_collection(&env_info, matches);
    let env_id = env_info.environment_id;
    let params = query_params(matches, "1");

    match query::query(
        &info.creds,
        &env_id,
        collection["collection_id"].as_str().unwrap(),
        params,
    ) {
        Ok(response) => {
            println!(
                "{}",
                to_string_pretty(&response).expect(
                    "Internal error: failed to format \
                                  query::query response",
                )
            )
        }
        Err(e) => println!("Failed to lookup collection {}", e),
    }
}

pub fn notices(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(&creds);
    let env_info = writable_environment(&info);
    let collection = select_collection(&env_info, matches);
    let env_id = env_info.environment_id;
    let params = query_params(matches, "10");

    match query::notices(
        &info.creds,
        &env_id,
        collection["collection_id"].as_str().unwrap(),
        params,
    ) {
        Ok(response) => {
            println!(
                "{}",
                to_string_pretty(&response).expect(
                    "Internal error: failed to format \
                                  query::notices response",
                )
            )
        }
        Err(e) => println!("Failed to lookup collection {}", e),
    }
}
