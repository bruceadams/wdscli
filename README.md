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
  (based on [scratch](https://hub.docker.com/_/scratch/))
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
wdscli
Basic administration for Watson Discovery Service.

USAGE:
    wdscli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    crawler-configuration    Print out crawler configuration. [aliases: cc]
    create-collection        Create a new collection using the most recently
                             created configuration [aliases: cl]
    create-configuration     Create a new configuration. [aliases: cn]
    create-environment       Create a writable environment [aliases: ce]
    delete-collection        Delete a collection. [aliases: dl]
    delete-environment       Delete the writable environment [aliases: de]
    help                     Prints this message or the help of the given
                             subcommand(s)
    overview                 Displays information about existing resources.
                             [aliases: o]
    show-collection          Displays detailed information about a collection.
                             [aliases: sl]
    show-configuration       Displays detailed information about a configuration.
                             [aliases: sn]
    show-environment         Displays detailed information about an environment.
                             [aliases: se]
$ wdscli help overview
wdscli-overview
Displays information about existing resources.

USAGE:
    wdscli overview [FLAGS] <credentials>

FLAGS:
    -g, --guid       Display the GUID for each item
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <credentials>    A JSON file containing service credentials.
$ wdscli overview ba-crawler-testing.json

Environment: ba-crawler-testing-6, 2 GB disk, 1.55 GB memory
   Configurations: Default Configuration

Environment: Watson News Environment - read only
   Configurations: Default Configuration
   Collections: watson_news, 23428185 available
```
## Running
### Credentials
Every `wdscli` command (except `help`) requires credentials for
a Watson Discovery Service instance.


## Building
I highly recommend installing Rust using https://rustup.rs
(which boils down to `curl -sSf https://sh.rustup.rs | sh`).
This project depends on several packages from https://crates.io
and [wdsapi](https://github.com/bruceadams/wdsapi).

`cargo build`
