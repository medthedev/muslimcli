# muslimcli

A command line utility that has various islamic tools in the terminal to help you get closer to Allah.

> **Status:** early development. The CLI still not mature enough

## Features

- **Prayer times** — fetch today's Fajr, Dhuhr, Asr, Maghrib and Isha times for your city from the
  [AlAdhan API](https://aladhan.com/prayer-times-api).
- **Setup** — store your city and country in a local config file so you don't re-enter them.

## Requirements

- Rust 1.85+ (edition 2024)
- An internet connection to fetch prayer times

## Installation

Build from source:

```sh
git clone https://github.com/medthedev/muslimcli.git
cd muslimcli
cargo build --release
```

The binary is at `target/release/muslimcli`. To install it on your `PATH`:

```sh
cargo install --path .
```

## Usage

Run `setup` once and enter your city and country:

```sh
muslimcli setup
# What is your city name?
# Algiers
# What is your country name?
# Algeria
```

Then display the prayer times:

```sh
muslimcli list
# Fajr: 05:52
# Dhuhr: 12:31
# Asr: 15:24
# Maghrib: 17:53
# Isha: 19:11
```

Other actions:

```sh
muslimcli help      # list available actions
muslimcli version   # print the version
```

## Configuration

Settings live in a `config.toml` file inside your platform's config directory:

Linux : `~/.config/muslimcli/config.toml` 

The file is created automatically on first run and has this shape:

```toml
[info]
city = "Casablanca"
country = "Morocco"
```

You can edit it by hand or rerun `muslimcli setup`.

## License

Distributed under the ["Waqf" General Public License 2.0](LICENSE).
