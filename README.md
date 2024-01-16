# Kaelix

[crates.io](https://crates.io/crates/kaelix)

Kaelix is a text editor written in Rust for use in the command line interface (CLI). It is designed to be lightweight, fast, and customizable.

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
