# wdscli
Command line interface to the IBM Watson Discovery Service API
[![Travis build status](https://travis-ci.org/bruceadams/wdscli.svg?branch=master)](https://travis-ci.org/bruceadams/wdscli)
[![AppVeyor build status](https://ci.appveyor.com/api/projects/status/4toqd1lqbrkwtj17/branch/master?svg=true)](https://ci.appveyor.com/project/bruceadams/wdscli)

## Installing
Binaries (built for 64 bit x86) are published on our
[Github releases](https://github.com/bruceadams/wdscli/releases) page.
- `wdscli.exe` Microsoft Windows binary
- `wdscli.linux` statically linked Linux binary
- `wdscli.macos` macOS binary

Also, the Linux binary is available packaged in a small
Docker image based on `busybox` and published on
[Docker Hub](https://hub.docker.com/r/bruceadams/wdscli/).

Grab the binary that works for machine and get it onto your `PATH`.

## Running
`wdscli`'s user documentation is builtin to its help texts.

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
    crawler-configuration
            Print out crawler configuration. [aliases: cc]
    create-collection
            Create a new collection using the most
            recently created configuration [aliases: cl]
    create-configuration
            Create a new configuration. [aliases: cn]
    create-environment
            Create a writable environment [aliases: ce]
    delete-collection
            Delete a collection. Default: delete the
            oldest collection [aliases: dl]
    delete-environment
            Delete the writable environment [aliases: de]
    help
            Prints this message or the help of the given
            subcommand(s)
    show
            Displays information about existing resources.
$ wdscli help show
wdscli-show
Displays information about existing resources.

USAGE:
    wdscli show [FLAGS] <credentials>

FLAGS:
    -g               Display the GUID for each item
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <credentials>
            A JSON file containing service credentials.
$ wdscli show ba-crawler-testing.json
Environment: ba-crawler-testing-3, 2 GB disk, 1.55 GB memory
   Configurations: Default Configuration
                   Empty Configuration
                   No Enrichments Configuration
   Collections: newer-than-empty-config, 0 available
                newer-than-no-enrichments, 0 available
Environment: Watson News Environment - read only
   Configurations: Default Configuration
   Collections: watson_news, 22642128 available
```

## Building
I highly recommend installing Rust itself using https://rustup.rs.
With `rustup` installed, `rustup default nightly` will set you up for building
with the Rust's _nightly_ toolchain.
This project depends on [wdsapi](https://github.com/bruceadams/wdsapi)
which uses the [serde](https://serde.rs)'s code generation features,
which currently requires the _nightly_ toolchain.

`cargo build`
