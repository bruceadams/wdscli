use std::str::FromStr;
use std::string::ParseError;

#[derive(StructOpt, Debug)]
#[structopt]
pub struct WatsonDiscoveryTool {
    #[structopt(short = "c", long = "credentials")]
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
    #[structopt(name = "add-document")]
    /// Add a document to a collection.
    AddDocument,
    #[structopt(name = "create-collection")]
    /// Create a new collection.
    CreateCollection,
    #[structopt(name = "create-configuration")]
    /// Create a new configuration.
    CreateConfiguration,
    #[structopt(name = "create-environment")]
    /// Create a writable environment.
    CreateEnvironment,
    #[structopt(name = "delete-collection")]
    /// Delete a collection.
    DeleteCollection,
    #[structopt(name = "delete-configuration")]
    /// Delete a configuration.
    DeleteConfiguration,
    #[structopt(name = "delete-document")]
    /// Delete a document.
    DeleteDocument,
    #[structopt(name = "delete-environment")]
    /// Delete an environment.
    DeleteEnvironment,
    #[structopt(name = "crawler-configuration")]
    /// Print out crawler configuration.
    CrawlerConfiguration { collection: String },
    #[structopt(name = "generate-completions")]
    /// Generate a shell command completion script.
    GenerateCompletions {
        /// One of: bash, fish, powershell or zsh.
        shell: String,
    },
    #[structopt(name = "notices")]
    /// Query ingestion notices for a collection.
    #[structopt(name = "overview")]
    /// Displays information about existing resources.
    #[structopt(name = "preview")]
    /// Preview conversion and enrichment for a document.
    #[structopt(name = "query")]
    /// Query a collection.
    Query {
        #[structopt(short = "a")]
        aggregation: Option<String>,
        #[structopt(short = "c", default_value = "1")]
        count: u32,
        #[structopt(short = "f")]
        filter: Option<String>,
        #[structopt(short = "q")]
        query: Option<String>,
        #[structopt(short = "r")]
        return_hierarchy: Option<String>,
        #[structopt(short = "s")]
        sort: Option<String>,
    },
    #[structopt(name = "show-collection")]
    ShowCollection,
    #[structopt(name = "show-configuration")]
    ShowConfiguration,
    #[structopt(name = "show-document")]
    ShowDocument {
        #[structopt(short = "n")]
        /// Use the newest collection; default if no other selection.
        newest: bool,
        #[structopt(short = "o")]
        /// Use the oldest collection; default if no other selection.
        oldest: bool,
        #[structopt(short = "l")]
        /// Select collection by id or name.
        collection: Option<String>,
        /// One or more document ids to lookup.
        document_id: Vec<String>,
    },
    #[structopt(name = "show-environment")]
    ShowEnvironment {
        #[structopt(short = "s")]
        /// Show the system environment.
        system: bool,
        #[structopt(short = "l")]
        /// Show the legacy read-only environment.
        legacy: bool,
        #[structopt(short = "w")]
        /// Show the writable environment; default if no other
        /// environment selection is made.
        writable: bool,
    },
}
