use clap;
use info::discovery_service_info;
use select::{select_collection, writable_environment};
use serde_json::ser::to_string_pretty;
use wdsapi::common::{Credentials, QueryParams};
use wdsapi::query;

fn query_params(matches: &clap::ArgMatches) -> QueryParams {
    QueryParams {
        filter: matches.value_of("filter").map(|s| s.to_string()),
        query: matches.value_of("query").map(|s| s.to_string()),
        aggregation: matches.value_of("aggregation").map(|s| {
                                                             s.to_string()
                                                         }),
        count: matches.value_of("count")
                      .unwrap_or("1")
                      .parse::<u64>()
                      .unwrap(),
        return_hierarchy:
            matches.value_of("return_hierarchy").map(|s| s.to_string()),
        offset: matches.value_of("offset").map(|s| s.parse::<u64>().unwrap()),
    }
}

pub fn query(creds: Credentials, matches: &clap::ArgMatches) {
    let info = discovery_service_info(creds);
    let env_info = writable_environment(&info);
    let env_id = env_info.environment.environment_id.clone();
    let collection = select_collection(&env_info, matches);
    let params = query_params(matches);

    match query::query(&info.creds,
                       &env_id,
                       &collection.collection_id,
                       params) {
        Ok(response) => {
            println!("{}",
                     to_string_pretty(&response)
                         .expect("Internal error: failed to format \
                                  collection::detail response"))
        }
        Err(e) => println!("Failed to lookup collection {}", e),
    }
}
