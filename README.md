# `wdscli`
Command line interface to the IBM Watson Discovery Service API
[![Travis build status](https://travis-ci.org/bruceadams/wdscli.svg?branch=master)](https://travis-ci.org/bruceadams/wdscli)
[![AppVeyor build status](https://ci.appveyor.com/api/projects/status/4toqd1lqbrkwtj17/branch/master?svg=true)](https://ci.appveyor.com/project/bruceadams/wdscli)

## Installing
Binaries (for 64bit x86) are published on our
[Github releases](https://github.com/bruceadams/wdscli/releases) page.
- `wdscli.exe` Microsoft Windows binary
- `wdscli.linux` Linux binary; statically linked.
  `wdscli` will run in an empty Docker image
  (that is: based on [scratch](https://hub.docker.com/_/scratch/))
  as well as in any Linux distribution.
- `wdscli.macos` macOS binary

Also, the Linux binary is available packaged in a small
Docker image based on [alpine](https://hub.docker.com/_/alpine/)
and published on
[Docker Hub](https://hub.docker.com/r/bruceadams/wdscli/).

Grab the binary for your machine and get it onto your `PATH`.

## Example
```
$ wdscli help
wdscli 2.2.0
Bruce Adams <bruce.adams@acm.org>
Basic administration for Watson Discovery Service.

USAGE:
    wdscli [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --credentials <credentials>    A JSON file containing service credentials. Default is the
                                       value of the environment variable WDSCLI_CREDENTIALS_FILE or
                                       'credentials.json' when WDSCLI_CREDENTIALS_FILE is not set.

SUBCOMMANDS:
    add-document             Add a document to a collection. [aliases: ad]
    crawler-configuration    Print out crawler configuration. [aliases: cc]
    create-collection        Create a new collection using the most recently created
                             configuration [aliases: cl]
    create-configuration     Create a new configuration. [aliases: cn]
    create-environment       Create a writable environment [aliases: ce]
    delete-collection        Delete a collection. [aliases: dl]
    delete-configuration     Delete a configuration. [aliases: dn]
    delete-document          Delete a document from a collection. [aliases: dd]
    delete-environment       Delete the writable environment [aliases: de]
    generate-completions     Generate a shell command completion script.
    help                     Prints this message or the help of the given subcommand(s)
    notices                  Query ingestion notices for a collection. [aliases: n]
    overview                 Displays information about existing resources. [aliases: o]
    preview                  Preview conversion and enrichment for a document. [aliases: p]
    query                    Query a collection. [aliases: q]
    show-collection          Displays detailed information about a collection. [aliases: sl]
    show-configuration       Displays detailed information about a configuration. [aliases: sn]
    show-document            Displays status information about a document. [aliases: sd]
    show-environment         Displays detailed information about an environment. [aliases: se]
$ wdscli help overview
wdscli-overview
Displays information about existing resources.

USAGE:
    wdscli overview [FLAGS]

FLAGS:
    -g, --guid       Display the GUID for each item
    -h, --help       Prints help information
    -V, --version    Prints version information
$ wdscli overview

Environment: "ba-cadence-testing", size=0, 4 GB disk, 1007.38 MB memory
   Configurations: "Default Configuration"
                   "extract all; english"
   Collections: "irs-pdf" ↳ "extract all; english", 2094 available, 12 failed

Environment: "Watson News Environment" - read only
   Configurations: "Default Configuration"
   Collections: "watson_news" ↳ "Default Configuration", 17271335 available
```
## Running
### Credentials
Every `wdscli` command (except `help`) requires credentials for
a Watson Discovery Service instance.

#### Credentials from Bluemix web console

On https://console.ng.bluemix.net/dashboard/apps/, find the Watson Discovery
instance you want to work with and select that service. The service instance
overview screen has three tabs:
- Manage — Service credentials — Connections

Select the _Service credentials_ tab, then select `View credentials` to see the
credentials in JSON format.

#### Credentials from command line `cf` tool

The `cf` command line tool (and the enhanced for Bluemix variant `bx`) is very
powerful. That power comes with a vast array of options that takes time to
learn. If you _already_ use `cf`, you can get the credentials for your service
instance with a few commands. First, you need to find the name(s) of the
credentials (there can be more than one) with the `cf service-keys` command
(notice _keys_ is plural here). For my service instance named `ba-demo`:

```
$ cf service-keys ba-demo
Getting keys for service instance ba-demo as ba@us.ibm.com...

name
Credentials-1
```

The credentials name shown here, `Credentials-1`, is what Bluemix usually
creates when you create a service a service instance using the Bluemix web
console. The `cf service-key` (singular _key_) will show the JSON formatted
credentials, like this:

```
$ cf service-key ba-demo Credentials-1
Getting key Credentials-1 for service instance ba-demo as ba@us.ibm.com...

{
 "password": "a password",
 "url": "https://gateway.watsonplatform.net/discovery/api",
 "username": "74c397c5-1ef1-4976-a0e8-89a0b831d679"
}
```

To directly write these credentials to a file, you can use `tail -5` to
only save the last five lines of the output, which is in JSON format.
```
$ cf service-key ba-demo Credentials-1 | tail -5 > credentials.json
```

## Customizing `X-Global-Transaction-ID`

There is a subtle feature for customizing the value of the HTTP header
`X-Global-Transaction-ID`. The value of the environment variable
`X_GLOBAL_TRANSACTION_ID` will be used as the beginning of the header value.

For example, running this command:

    X_GLOBAL_TRANSACTION_ID=demo-tx-header wdscli overview

will send `X-Global-Transaction-ID` headers with the values: `demo-tx-header-0`,
`demo-tx-header-1`, `demo-tx-header-2` … The sequence number is incremented for
each Discovery API call that is made.

## Building
I highly recommend installing Rust using https://rustup.rs
(which boils down to `curl -sSf https://sh.rustup.rs | sh`).
This project depends on several packages from https://crates.io
and [wdsapi](https://github.com/bruceadams/wdsapi).

`cargo build`
