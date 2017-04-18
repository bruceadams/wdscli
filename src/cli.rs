use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!(", "))
        .version(crate_version!())
        .arg(Arg::with_name("credentials")
            .short("c")
            .long("creds")
            .long("credentials")
            .takes_value(true)
            .help("A JSON file containing service credentials. Default is \
                   the value of the environment variable \
                   WDSCLI_CREDENTIALS_FILE or 'credentials.json' when \
                   WDSCLI_CREDENTIALS_FILE is not set."))
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("generate-completions")
            .about("Generate a shell command completion script.")
            .arg(Arg::with_name("shell")
                .required(true)
                .help("Which shell to generate completions for. One of: \
                       bash, fish, powershell or zsh")))
        .subcommand(SubCommand::with_name("crawler-configuration")
            .visible_alias("cc")
            .about("Print out crawler configuration.")
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Use most recently created collection, default if no \
                       other selection is made"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Use the collection created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Use the collection with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Use the collection with this id"))
            .group(ArgGroup::with_name("selector")
                .args(&["newest", "oldest", "name", "id"])))
        .subcommand(SubCommand::with_name("create-collection")
            .visible_alias("cl")
            .about("Create a new collection using the most recently created \
                    configuration")
            .arg(Arg::with_name("name")
                .required(true)
                .help("The name of the collection."))
            .arg(Arg::with_name("description")
                .short("d")
                .long("desc")
                .long("description")
                .takes_value(true)
                .help("Description text for the collection.")))
        .subcommand(SubCommand::with_name("create-configuration")
            .visible_alias("cn")
            .about("Create a new configuration.")
            .arg(Arg::with_name("configuration")
                .required(true)
                .help("File containing the configuration JSON.")))
        .subcommand(SubCommand::with_name("create-environment")
            .visible_alias("ce")
            .about("Create a writable environment")
            .arg(Arg::with_name("name")
                .required(true)
                .help("The name of the environment."))
            .arg(Arg::with_name("size")
                .short("s")
                .long("size")
                .takes_value(true)
                .help("The size environment to create."))
            .arg(Arg::with_name("description")
                .short("d")
                .long("desc")
                .long("description")
                .takes_value(true)
                .help("Description text for the environment."))
            .arg(Arg::with_name("wait")
                .long("wait")
                .help("Wait for the new environment to become active.")))
        .subcommand(SubCommand::with_name("delete-collection")
            .visible_alias("dl")
            .about("Delete a collection.")
            .arg(Arg::with_name("all")
                .long("all")
                .help("Delete all existing collections"))
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Delete the most recently created collection"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Delete the collection created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Delete the collection with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Delete the collection with this id"))
            .group(ArgGroup::with_name("selector")
                .required(true)
                .args(&["all", "newest", "oldest", "name", "id"])))
        .subcommand(SubCommand::with_name("delete-configuration")
            .visible_alias("dn")
            .about("Delete a configuration.")
            .arg(Arg::with_name("all")
                .long("all")
                .help("Delete all existing configurations, except for the \
                       built-in configuration."))
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Delete the most recently created configuration"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Delete the configuration with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Delete the configuration with this id"))
            .group(ArgGroup::with_name("selector")
                .required(true)
                .args(&["all", "newest", "name", "id"])))
        .subcommand(SubCommand::with_name("delete-environment")
            .visible_alias("de")
            .about("Delete the writable environment"))
        .subcommand(SubCommand::with_name("overview")
            .visible_alias("o")
            .about("Displays information about existing resources.")
            .arg(Arg::with_name("guid")
                .short("g")
                .long("guid")
                .help("Display the GUID for each item")))
        .subcommand(SubCommand::with_name("query")
            .visible_alias("q")
            .about("Query a collection.")
            .arg(Arg::with_name("aggregation")
                .short("a")
                .long("aggregation")
                .takes_value(true)
                .help("The aggregation string for the query"))
            .arg(Arg::with_name("count")
                .short("C")
                .long("count")
                .takes_value(true)
                .help("The number of results to return for the query"))
            .arg(Arg::with_name("offset")
                .long("offset")
                .takes_value(true)
                .help("The offset of the first result returned by the query"))
            .arg(Arg::with_name("filter")
                .short("f")
                .long("filter")
                .takes_value(true)
                .help("The filter string for the query"))
            .arg(Arg::with_name("query")
                .short("q")
                .long("query")
                .takes_value(true)
                .help("The query string for the query"))
            .arg(Arg::with_name("natural_language_query")
                .short("l")
                .long("natural_language_query")
                .takes_value(true)
                .help("The natural language query string for the query"))
            .arg(Arg::with_name("sort")
                .short("s")
                .long("sort")
                .takes_value(true)
                .help("The sort string for the query"))
            .arg(Arg::with_name("passages")
                .short("p")
                .long("passages")
                .help("Return passages"))
            .arg(Arg::with_name("return_hierarchy")
                .short("h")
                .long("return_hierarchy")
                .takes_value(true)
                .help("The return hierarchy string for the query"))
            .arg(Arg::with_name("read-only")
                .long("read-only")
                .short("r")
                .help("Use the read only environment"))
            .arg(Arg::with_name("writable")
                .long("writable")
                .short("w")
                .help("Use the writable environment, default if no other \
                       environment selection is made"))
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Use most recently created collection, default if no \
                       other collection selection is made"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Use the collection created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Use the collection with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Use the collection with this id"))
            .group(ArgGroup::with_name("environment")
                .args(&["read-only", "writable"]))
            .group(ArgGroup::with_name("collection")
                .args(&["newest", "oldest", "name", "id"])))
        .subcommand(SubCommand::with_name("show-environment")
            .visible_alias("se")
            .about("Displays detailed information about an environment.")
            .arg(Arg::with_name("read-only")
                .long("read-only")
                .short("r")
                .help("Show the read only environment"))
            .arg(Arg::with_name("writable")
                .long("writable")
                .short("w")
                .help("Show the writable environment, default if no other \
                       environment selection is made"))
            .group(ArgGroup::with_name("environment")
                .args(&["read-only", "writable"])))
        .subcommand(SubCommand::with_name("show-collection")
            .visible_alias("sl")
            .about("Displays detailed information about a collection.")
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Use most recently created collection, default if no \
                       other selection is made"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Use the collection created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Use the collection with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Use the collection with this id"))
            .group(ArgGroup::with_name("selector")
                .args(&["newest", "oldest", "name", "id"])))
        .subcommand(SubCommand::with_name("show-document")
            .visible_alias("sd")
            .about("Displays status information about a document.")
            .arg(Arg::with_name("document_id")
                .required(true)
                .multiple(true)
                .help("The document_id(s) to lookup."))
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Use most recently created collection, default if no \
                       other selection is made"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Use the collection created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Use the collection with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Use the collection with this id"))
            .group(ArgGroup::with_name("selector")
                .args(&["newest", "oldest", "name", "id"])))
        .subcommand(SubCommand::with_name("add-document")
            .visible_alias("ad")
            .about("Add a document to a collection.")
            .arg(Arg::with_name("filenames")
                .required(true)
                .multiple(true)
                .help("File paths for documents to add."))
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Use most recently created collection, default if no \
                       other selection is made"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Use the collection created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Use the collection with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Use the collection with this id"))
            .group(ArgGroup::with_name("selector")
                .args(&["newest", "oldest", "name", "id"])))
        .subcommand(SubCommand::with_name("show-configuration")
            .visible_alias("sn")
            .about("Displays detailed information about a configuration.")
            .arg(Arg::with_name("newest")
                .short("n")
                .long("newest")
                .help("Use most recently created configuration, default if \
                       no other selection is made"))
            .arg(Arg::with_name("oldest")
                .short("o")
                .long("oldest")
                .help("Use the configuration created the longest time ago"))
            .arg(Arg::with_name("name")
                .short("m")
                .long("named")
                .takes_value(true)
                .help("Use the configuration with a name matching this"))
            .arg(Arg::with_name("id")
                .short("i")
                .long("with-id")
                .takes_value(true)
                .help("Use the configuration with this id"))
            .group(ArgGroup::with_name("selector")
                .args(&["newest", "oldest", "name", "id"])))
}
