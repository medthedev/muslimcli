# muslimcli

A small command-line companion that keeps a few Islamic utilities a keystroke away:
prayer times for your city and a random dua, right in the terminal.

> **Status:** early development — expect rough edges and breaking changes.

## Features

- **Prayer times** — print Fajr, Dhuhr, Asr, Maghrib and Isha for your configured city
  using the [AlAdhan prayer times API](https://aladhan.com/prayer-times-api).
- **Random dua** — print a randomly chosen dua in Arabic from the
  [Ummah API](https://ummahapi.com/).

## Requirements

- Rust 1.85 or newer (the project uses edition 2024)
- An internet connection for `list` and `dua`

## Installation

Build from source:

```sh
git clone https://github.com/medthedev/muslimcli.git
cd muslimcli
cargo build --release
```

The binary is written to `target/release/muslimcli`. To install it into your Cargo
bin directory instead:

```sh
cargo install --path .
```

## Usage

```console
$ muslimcli --help
A small CLI for prayer times and duas

Usage: muslimcli <ACTION>

Arguments:
  <ACTION>
          Action to perform

          Possible values:
          - list:  Show today's prayer times
          - setup: Set your city and country in the config
          - reset: Reset the config to its defaults
          - dua:   Show a random dua

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Set your location once:

```console
$ muslimcli setup
What is your city name?
Algiers
What is your country name?
Algeria
Config updated successfully.
```

Then print the prayer times:

```console
$ muslimcli list
Fajr: 05:52
Dhuhr: 12:31
Asr: 15:24
Maghrib: 17:53
Isha: 19:11
```

Or fetch a random dua:

```console
$ muslimcli dua
اللهم اجعل القرآن ربيع قلبي
```

| Action  | Description                                                   |
| ------- | ------------------------------------------------------------- |
| `list`  | Fetch and print the five daily prayer times for your city.    |
| `dua`   | Fetch and print a random dua in Arabic.                       |
| `setup` | Prompt for your city and country and save them to the config. |
| `reset` | Restore the config to its default (`unset`) values.           |

Use `--help` for full usage information and `--version` to print the CLI version
(`-h` and `-V` are short aliases).

> `list` reads your city and country from the config, so run `setup` first.

## Configuration

Settings live in a `config.toml` file in your platform's config directory, resolved
with the [`directories`](https://crates.io/crates/directories) crate:

| Platform | Location                                              |
| -------- | ----------------------------------------------------- |
| Linux    | `~/.config/muslimcli/config.toml`                     |
| macOS    | `~/Library/Application Support/muslimcli/config.toml` |
| Windows  | `%APPDATA%\medthedev\muslimcli\config\config.toml`    |

The file is written when you run `setup` (or `reset`) and looks like this:

```toml
[info]
city = "Algiers"
country = "Algeria"
```

You can edit it by hand, rerun `muslimcli setup`, or run `muslimcli reset` to start
over.

## License

Distributed under the ["Waqf" General Public License 2.0](LICENSE).
