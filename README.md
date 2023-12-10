# telraam-rs

[Telraam](https://telraam.net/en/what-is-telraam) is a system for collecting data from streets by way of people placing IOT devices in their homes to track the usage of their street. It provides crucial data on various modes of transport, including motor vehicles, cyclists, pedestrians, and more. Telraam networks can help create the opportunity for dialogue between traffic planners, local authorities and their most affected communities: the citizens who live on - and use - these streets, by turning traffic counting into an open and accessible citizen science project.

This project is a library and CLI for accessing the data captured by the Telraam network. It is intended for collecting data which can then be stored in another system for analysis. Feel free to make suggestions on any improvements that can be made, or contribute to the project.

## Getting Started

This project requires the Rust toolchain, use [rustup](https://rustup.rs) to install, or a system package manager.

### Installing from the current published version

This will install the currently published crate from the Crate.io public Rust package manager

```shell
> cargo install --features=clap --bin telraam telraam-rs
```

### Working from source

Clone this repo:

```shell
> git clone https://github.com/radical-bike-lobby/telraam-rs.git
```

*note*, the next series of commands assume your current working directory is in `telraam-rs`, i.e. `cd telraam-rs`

Running tests:

```shell
> cargo test --all-features
```

Run directly from the project with the `cargo run` command:

```shell
> cargo run --features=clap -- --help
```

This can be installed from source as well:

```shell
> cargo install --features=clap --path . --bin telraam
```

## Running the commands

All of the commands are derived from the [Telraam documentation](https://documenter.getpostman.com/view/8210376/TWDRqyaV#intro). You will need to login into your Telraam account and get an API token: https://telraam.net/en/admin/mijn-eigen-telraam/tokens

The telraam CLI will read this from the environment `TELRAAM_TOKEN`, or from the parameter `-t ${TELRAAM_TOKEN}`. It's recommended to use the environment variable as this won't end up with the token in logs or shell history.

Test that your token and the CLI work with the `welcome` command,

```shell
> telraam welcome
msg = hello! Telraam server 2.0 is up and running
```

When running with `cargo run`, all options to telraam must follow `--`,

```shell
> cargo run --features=clap -- welcome
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/telraam welcome`
msg = hello! Telraam server 2.0 is up and running
```

For the other options, use `help`:

```
> telraam --help
```

Some commands have their own parameters:

```shell
> telraam traffic --help
```

## Output

All output is currently in json, the formats are the inner types defined in the API, like the list of reports in `traffic`, see the [documentation for traffic](https://documenter.getpostman.com/view/8210376/TWDRqyaV#3bb3c6bd-ea23-4329-b885-0d142403ecbb).


## Contributing

This project is licensed under the MIT license. Please feel free to contribute.

Let's all work to make our streets safer!
