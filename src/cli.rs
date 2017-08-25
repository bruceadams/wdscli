
const BASE_URL: &str = "https://gateway.watsonplatform.net/discovery/api";

#[derive(StompCommand, Debug)]
#[stomp(name = "wdt")]
pub struct WatsonDiscoveryTool {
    /// A JSON file containing service credentials.
    /// Default is the value of the environment variable WDT_CREDENTIALS_FILE
    /// or "credentials.json" when WDT_CREDENTIALS_FILE is empty.
    #[stomp(long = "credentials", short = "c")]
    pub credentials: Option<String>,
    /// Username of service credentials
    #[stomp(long = "username", short = "u")]
    pub username: Option<String>,
    /// Password of service credentials
    #[stomp(long = "password", short = "p")]
    pub password: Option<String>,
    /// Base API URL
    #[stomp(long = "url", short = "a", default_value = "https://gateway.watsonplatform.net/discovery/api")]
    pub url: Option<String>,
    /// Debug: show the API request
    #[stomp(long = "debug", short = "D")]
    pub debug: bool,
    #[stomp(subcommand)]
    pub verb: Verb,
}

/// Add one or more documents to a collection.
#[derive(StompCommand, Debug)]
#[stomp(name = "add-document")]
pub struct AddDocumentOptions {
    #[stomp(index = 1)]
    /// Directory or file paths for documents to add.
    paths: Vec<String>,
    #[stomp(long = "youngest", short = "y")]
    /// Add document to the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Add document to the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "crawler-configuration")]
/// Print out crawler configuration.
pub struct CrawlerConfigurationOptions {
    #[stomp(long = "youngest", short = "y")]
    /// Use the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Use the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "create-collection")]
/// Create a new collection.
pub struct CreateCollectionOptions {
    /// The name for the new collection
    collection_name: String,
    #[stomp(long = "youngest", short = "y")]
    /// Using the youngest configuration; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Using the oldest configuration.
    oldest: bool,
    #[stomp(long = "configuration", short = "n")]
    /// Choose configuration by id or name.
    configuration: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "create-configuration")]
/// Create a new configuration.
pub struct CreateConfigurationOptions {
    /// File containing the configuration JSON, "-" for stdin.
    configuration: String,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "create-environment")]
/// Create a writable environment.
pub struct CreateEnvironmentOptions {
    /// The name for the new environment.
    enviroment_name: String,
    #[stomp(long = "description", short = "d")]
    /// A description for the new environment.
    description: Option<String>,
    #[stomp(long = "wait")]
    /// Wait for the new environment to become active.
    wait: bool,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "delete-collection")]
/// Delete a collection.
pub struct DeleteCollectionOptions {
    #[stomp(long = "all")]
    /// Delete all collections.
    all: bool,
    #[stomp(long = "youngest", short = "y")]
    /// Delete the youngest collection.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Delete the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Delete collection by id or name.
    collection: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "delete-configuration")]
/// Delete a configuration.
pub struct DeleteConfigurationOptions {
    #[stomp(long = "all")]
    /// Delete all non-system configurations.
    all: bool,
    #[stomp(long = "youngest", short = "y")]
    /// Delete the youngest configuration.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Delete the oldest configuration.
    oldest: bool,
    #[stomp(long = "configuration", short = "n")]
    /// Delete configuration by id or name.
    configuration: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "delete-document")]
/// Delete a document.
pub struct DeleteDocumentOptions {
    #[stomp(long = "youngest", short = "y")]
    /// From the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// From the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
    /// One or more document ids to delete.
    document_id: Vec<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "delete-environment")]
/// Delete an environment.
pub struct DeleteEnvironmentOptions {
    /// The name of the environment to delete (for safety).
    enviroment_name: String,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "generate-completions")]
/// Generate a shell command completion script.
pub struct GenerateCompletionsOptions {
    /// One of: "bash", "fish", "powershell" or "zsh".
    shell: String,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "notices")]
/// Query ingestion notices for a collection.
pub struct NoticesOptions {
    #[stomp(long = "aggregation", short = "a")]
    aggregation: Option<String>,
    #[stomp(long = "count", short = "c", default_value = "1")]
    count: u32,
    #[stomp(long = "filter", short = "f")]
    filter: Option<String>,
    #[stomp(long = "query", short = "q")]
    query: Option<String>,
    #[stomp(long = "return", short = "r")]
    return_hierarchy: Option<String>,
    #[stomp(long = "sort", short = "s")]
    sort: Option<String>,
    #[stomp(long = "youngest", short = "y")]
    /// Use the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Use the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "overview")]
/// Displays information about existing resources.
pub struct OverviewOptions {
    #[stomp(long = "guid", short = "g")]
    /// Also display the GUID for each item.
    guid: bool,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "preview")]
/// Preview conversion and enrichment for a document.
pub struct PreviewOptions {
    /// File path for document to preview.
    filename: String,
    #[stomp(long = "youngest", short = "y")]
    /// Use the youngest configuration; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Use the oldest configuration.
    oldest: bool,
    #[stomp(long = "configuration", short = "n")]
    /// Select configuration by id or name.
    configuration: Option<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "query")]
/// Query documents in a collection.
pub struct QueryOptions {
    #[stomp(long = "aggregation", short = "a")]
    aggregation: Option<String>,
    #[stomp(long = "count", short = "c", default_value = "1")]
    count: u32,
    #[stomp(long = "filter", short = "f")]
    filter: Option<String>,
    #[stomp(long = "query", short = "q")]
    query: Option<String>,
    #[stomp(long = "return", short = "r")]
    return_hierarchy: Option<String>,
    #[stomp(long = "sort", short = "s")]
    sort: Option<String>,
    #[stomp(long = "youngest", short = "y")]
    /// Show the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Show the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
    #[stomp(long = "system", short = "S")]
    /// In the system environment.
    system: bool,
    #[stomp(long = "legacy", short = "L")]
    /// In the legacy read-only environment, if it exists.
    legacy: bool,
    #[stomp(long = "writable", short = "W")]
    /// In the writable environment; default.
    writable: bool,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "show-collection")]
/// Display detailed information about a collection.
pub struct ShowCollectionOptions {
    #[stomp(long = "youngest", short = "y")]
    /// Show the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Show the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
    #[stomp(long = "system", short = "S")]
    /// In the system environment.
    system: bool,
    #[stomp(long = "legacy", short = "L")]
    /// In the legacy read-only environment, if it exists.
    legacy: bool,
    #[stomp(long = "writable", short = "W")]
    /// In the writable environment; default.
    writable: bool,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "show-configuration")]
/// Display detailed information about a configuration.
pub struct ShowConfigurationOptions {
    #[stomp(long = "youngest", short = "y")]
    /// Show the youngest configuration; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// Show the oldest configuration.
    oldest: bool,
    #[stomp(long = "configuration", short = "n")]
    /// Select configuration by id or name.
    configuration: Option<String>,
    #[stomp(long = "system", short = "S")]
    /// In the system environment.
    system: bool,
    #[stomp(long = "legacy", short = "L")]
    /// In the legacy read-only environment, if it exists.
    legacy: bool,
    #[stomp(long = "writable", short = "W")]
    /// In the writable environment; default.
    writable: bool,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "show-document")]
/// Displays status information about one or more documents.
pub struct ShowDocumentOptions {
    #[stomp(long = "youngest", short = "y")]
    /// In the youngest collection; default.
    youngest: bool,
    #[stomp(long = "oldest", short = "o")]
    /// In the oldest collection.
    oldest: bool,
    #[stomp(long = "collection", short = "l")]
    /// Select collection by id or name.
    collection: Option<String>,
    /// One or more document ids to lookup.
    document_id: Vec<String>,
}

#[derive(StompCommand, Debug)]
#[stomp(name = "show-environment")]
/// Display detailed information about an environment.
pub struct ShowEnvironmentOptions {
    #[stomp(long = "system", short = "S")]
    /// Show the system environment.
    system: bool,
    #[stomp(long = "legacy", short = "L")]
    /// Show the legacy read-only environment, if it exists.
    legacy: bool,
    #[stomp(long = "writable", short = "W")]
    /// Show the writable environment; default.
    writable: bool,
}


#[derive(StompCommands, Debug)]
#[stomp]
pub enum Verb {
    #[stomp(name = "add-document")]
    AddDocument(AddDocumentOptions),
    #[stomp(name = "crawler-configuration")]
    /// Print out crawler configuration.
    CrawlerConfiguration(CrawlerConfigurationOptions),
    #[stomp(name = "create-collection")]
    /// Create a new collection.
    CreateCollection(CreateCollectionOptions),
    #[stomp(name = "create-configuration")]
    /// Create a new configuration.
    CreateConfiguration(CreateConfigurationOptions),
    #[stomp(name = "create-environment")]
    /// Create a writable environment.
    CreateEnvironment(CreateEnvironmentOptions),
    #[stomp(name = "delete-collection")]
    /// Delete a collection.
    DeleteCollection(DeleteCollectionOptions),
    #[stomp(name = "delete-configuration")]
    /// Delete a configuration.
    DeleteConfiguration(DeleteConfigurationOptions),
    #[stomp(name = "delete-document")]
    /// Delete a document.
    DeleteDocument(DeleteDocumentOptions),
    #[stomp(name = "delete-environment")]
    /// Delete an environment.
    DeleteEnvironment(DeleteEnvironmentOptions),
    #[stomp(name = "generate-completions")]
    /// Generate a shell command completion script.
    GenerateCompletions(GenerateCompletionsOptions),
    #[stomp(name = "notices")]
    /// Query ingestion notices for a collection.
    Notices(NoticesOptions),
    #[stomp(name = "overview")]
    /// Displays information about existing resources.
    Overview(OverviewOptions),
    #[stomp(name = "preview")]
    /// Preview conversion and enrichment for a document.
    Preview(PreviewOptions),
    #[stomp(name = "query")]
    /// Query documents in a collection.
    Query(QueryOptions),
    #[stomp(name = "show-collection")]
    /// Display detailed information about a collection.
    ShowCollection(ShowCollectionOptions),
    #[stomp(name = "show-configuration")]
    /// Display detailed information about a configuration.
    ShowConfiguration(ShowConfigurationOptions),
    #[stomp(name = "show-document")]
    /// Displays status information about one or more documents.
    ShowDocument(ShowDocumentOptions),
    #[stomp(name = "show-environment")]
    /// Display detailed information about an environment.
    ShowEnvironment(ShowEnvironmentOptions),
}
