# MakeMKV CLI

## ✨ Features

- Wrapper around the MakeMKV command line interface to provide a more user-friendly experience
- Better main feature detection for series and movies using TMDb

## 💾 Installation

At the moment, you can only install the toolkit from source while building it on your own. This requires [git](https://git-scm.com/) and [Cargo](https://doc.rust-lang.org/stable/cargo/).

```bash
$ git clone https://github.com/DevTobias/autoripper
$ cd autoripper/services/makemkv-cli

# Either just build it (will be available in ./target/release/makemkv-cli)
$ cargo build --release

# Or install it to your system
$ cargo install --force --path .
```

## 🖥️ Usage

> All shown commands are examples 🧑🏼‍🍳

### Devices

The `device` command will list all available devices detected by MakeMKV. This will save all
devices to the specified json File, removing this option will print the results to the console.

```bash
$ makemkv-cli devices --output devices.json
```

When using a mac, you can use the `--location` flag to specify the location of the MakeMKV command
line executable.

```bash
$ makemkv-cli devices --location /Applications/MakeMKV.app/Contents/MacOS/makemkvcon --output devices.json
```
