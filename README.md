# Kaelix

[crates.io](https://crates.io/crates/kaelix)

Kaelix is a text editor written in Rust for use in the command line interface (CLI). It is designed to be lightweight, fast, and customizable.

## Screenshots
<div style="display: flex; justify-content: space-between;">
  <img alt="Start page" src="https://github.com/Diwakar-Gupta/kaelix/assets/39624018/e44c6def-d0ac-4ff0-a7e9-f6da4ec4ce44" width="45%">
  <img alt="Won" src="https://github.com/Diwakar-Gupta/kaelix/assets/39624018/7cb795df-6a8a-4d18-8a2b-a9e4d48fee61" width="45%">
</div>

## Usage

To open a file in Kaelix, simply run the following command:

```shell
kaelix filename.txt
```

### Controls

* `Ctrl + n` - create new document
* `Ctrl + o` - open file
* `Ctrl + w` - close current document
* `Ctrl + s` - save current document
* `Ctrl + k` - move to previous document
* `Ctrl + l` - move to next document
* `Ctrl + q` - quit program

## Installation

### Install using cargo

```shell
cargo install kaelix
```

### Build from source
To install Kaelix, clone the project and run this command from source directory

```shell
cargo build --release
```
this will generate the binary at `target/release/kaelix`

## Contributing

Contributions to Kaelix are welcome! If you find a bug or have an idea for a new feature, please open an issue on the GitHub repository.

## License

Kaelix is released under MIT license.
