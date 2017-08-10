use std::str::FromStr;
use std::string::ParseError;

#[derive(StructOpt, Debug)]
#[structopt]
pub struct WatsonDiscoveryTool {
    #[structopt(long = "credentials", short = "c")]
    /// A JSON file containing service credentials.
    // Default is the value of the environment variable
    // WDSCLI_CREDENTIALS_FILE or 'credentials.json' when
    // WDSCLI_CREDENTIALS_FILE is not set.
    credentials: Option<String>,
    #[structopt(subcommand)]
    verb: Verb,
}

#[derive(StructOpt, Debug)]
#[structopt]
enum Verb {
    #[structopt(name = "add-document", visible_alias = "ad")]
    /// Add one or more documents to a collection.
    AddDocument {
        /// Directory or file paths for documents to add.
        paths: Vec<String>,
        #[structopt(long = "youngest", short = "y")]
        /// Add document to the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Add document to the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
    },
    #[structopt(name = "crawler-configuration", visible_alias = "cc")]
    /// Print out crawler configuration.
    CrawlerConfiguration {
        #[structopt(long = "youngest", short = "y")]
        /// Use the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Use the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
    },
    #[structopt(name = "create-collection", visible_alias = "cl")]
    /// Create a new collection.
    CreateCollection {
        /// The name for the new collection
        collection_name: String,
        #[structopt(long = "youngest", short = "y")]
        /// Using the youngest configuration; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Using the oldest configuration.
        oldest: bool,
        #[structopt(long = "configuration", short = "n")]
        /// Choose configuration by id or name.
        configuration: Option<String>,
    },
    #[structopt(name = "create-configuration", visible_alias = "cn")]
    /// Create a new configuration.
    CreateConfiguration {
        /// File containing the configuration JSON, "-" for stdin.
        configuration: String,
    },
    #[structopt(name = "create-environment", visible_alias = "ce")]
    /// Create a writable environment.
    CreateEnvironment {
        /// The name for the new environment.
        enviroment_name: String,
        #[structopt(long = "description", short = "d")]
        /// A description for the new environment.
        description: Option<String>,
        #[structopt(long = "wait")]
        /// Wait for the new environment to become active.
        wait: bool,
    },
    #[structopt(name = "delete-collection", visible_alias = "dl")]
    /// Delete a collection.
    DeleteCollection {
        #[structopt(long = "all")]
        /// Delete all collections.
        all: bool,
        #[structopt(long = "youngest", short = "y")]
        /// Delete the youngest collection.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Delete the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Delete collection by id or name.
        collection: Option<String>,
    },
    #[structopt(name = "delete-configuration", visible_alias = "dn")]
    /// Delete a configuration.
    DeleteConfiguration {
        #[structopt(long = "all")]
        /// Delete all non-system configurations.
        all: bool,
        #[structopt(long = "youngest", short = "y")]
        /// Delete the youngest configuration.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Delete the oldest configuration.
        oldest: bool,
        #[structopt(long = "configuration", short = "n")]
        /// Delete configuration by id or name.
        configuration: Option<String>,
    },
    #[structopt(name = "delete-document", visible_alias = "dd")]
    /// Delete a document.
    DeleteDocument {
        #[structopt(long = "youngest", short = "y")]
        /// From the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// From the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
        /// One or more document ids to delete.
        document_id: Vec<String>,
    },
    #[structopt(name = "delete-environment", visible_alias = "de")]
    /// Delete an environment.
    DeleteEnvironment {
        /// The name of the environment to delete (for safety).
        enviroment_name: String,
    },
    #[structopt(name = "generate-completions")]
    /// Generate a shell command completion script.
    GenerateCompletions {
        /// One of: "bash", "fish", "powershell" or "zsh".
        shell: String,
    },
    #[structopt(name = "notices", visible_alias = "n")]
    /// Query ingestion notices for a collection.
    Notices {
        #[structopt(long = "aggregation", short = "a")]
        aggregation: Option<String>,
        #[structopt(long = "count", short = "c", default_value = "1")]
        count: u32,
        #[structopt(long = "filter", short = "f")]
        filter: Option<String>,
        #[structopt(long = "query", short = "q")]
        query: Option<String>,
        #[structopt(long = "return", short = "r")]
        return_hierarchy: Option<String>,
        #[structopt(long = "sort", short = "s")]
        sort: Option<String>,
        #[structopt(long = "youngest", short = "y")]
        /// Use the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Use the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
    },
    #[structopt(name = "overview", visible_alias = "o")]
    /// Displays information about existing resources.
    Overview {
        #[structopt(long = "guid", short = "g")]
        /// Also display the GUID for each item.
        guid: bool,
    },
    #[structopt(name = "preview", visible_alias = "p")]
    /// Preview conversion and enrichment for a document.
    Preview {
        /// File path for document to preview.
        filename: String,
        #[structopt(long = "youngest", short = "y")]
        /// Use the youngest configuration; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Use the oldest configuration.
        oldest: bool,
        #[structopt(long = "configuration", short = "n")]
        /// Select configuration by id or name.
        configuration: Option<String>,
    },
    #[structopt(name = "query", visible_alias = "q")]
    /// Query documents in a collection.
    Query {
        #[structopt(long = "aggregation", short = "a")]
        aggregation: Option<String>,
        #[structopt(long = "count", short = "c", default_value = "1")]
        count: u32,
        #[structopt(long = "filter", short = "f")]
        filter: Option<String>,
        #[structopt(long = "query", short = "q")]
        query: Option<String>,
        #[structopt(long = "return", short = "r")]
        return_hierarchy: Option<String>,
        #[structopt(long = "sort", short = "s")]
        sort: Option<String>,
        #[structopt(long = "youngest", short = "y")]
        /// Show the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Show the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
        #[structopt(long = "system", short = "S")]
        /// In the system environment.
        system: bool,
        #[structopt(long = "legacy", short = "L")]
        /// In the legacy read-only environment, if it exists.
        legacy: bool,
        #[structopt(long = "writable", short = "W")]
        /// In the writable environment; default.
        writable: bool,
    },
    #[structopt(name = "show-collection", visible_alias = "sl")]
    /// Display detailed information about a collection.
    ShowCollection {
        #[structopt(long = "youngest", short = "y")]
        /// Show the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Show the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
        #[structopt(long = "system", short = "S")]
        /// In the system environment.
        system: bool,
        #[structopt(long = "legacy", short = "L")]
        /// In the legacy read-only environment, if it exists.
        legacy: bool,
        #[structopt(long = "writable", short = "W")]
        /// In the writable environment; default.
        writable: bool,
    },
    #[structopt(name = "show-configuration", visible_alias = "cn")]
    /// Display detailed information about a configuration.
    ShowConfiguration {
        #[structopt(long = "youngest", short = "y")]
        /// Show the youngest configuration; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// Show the oldest configuration.
        oldest: bool,
        #[structopt(long = "configuration", short = "n")]
        /// Select configuration by id or name.
        configuration: Option<String>,
        #[structopt(long = "system", short = "S")]
        /// In the system environment.
        system: bool,
        #[structopt(long = "legacy", short = "L")]
        /// In the legacy read-only environment, if it exists.
        legacy: bool,
        #[structopt(long = "writable", short = "W")]
        /// In the writable environment; default.
        writable: bool,
    },
    #[structopt(name = "show-document", visible_alias = "sd")]
    /// Displays status information about one or more documents.
    ShowDocument {
        #[structopt(long = "youngest", short = "y")]
        /// In the youngest collection; default.
        youngest: bool,
        #[structopt(long = "oldest", short = "o")]
        /// In the oldest collection.
        oldest: bool,
        #[structopt(long = "collection", short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
        /// One or more document ids to lookup.
        document_id: Vec<String>,
    },
    #[structopt(name = "show-environment", visible_alias = "se")]
    /// Display detailed information about an environment.
    ShowEnvironment {
        #[structopt(long = "system", short = "S")]
        /// Show the system environment.
        system: bool,
        #[structopt(long = "legacy", short = "L")]
        /// Show the legacy read-only environment, if it exists.
        legacy: bool,
        #[structopt(long = "writable", short = "W")]
        /// Show the writable environment; default.
        writable: bool,
    },
}
