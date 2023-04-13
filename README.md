# Kaelix
> While Kaelix is a fun project to work on, it is important to note that it is not intended to replace your current text editor.
> This is just a POC project

Kaelix is a text editor written in Rust for use in the command line interface (CLI). It is designed to be lightweight, fast, and customizable.

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

## Usage

To open a file in Kaelix, simply run the following command:

```shell
kaelix filename.txt
```


Kaelix provides a number of keyboard shortcuts for navigating and editing files. For a full list of shortcuts, run the following command:

```shell
kaelix --help
```

## Contributing

Contributions to Kaelix are welcome! If you find a bug or have an idea for a new feature, please open an issue on the GitHub repository.

## License

Kaelix is released under MIT license.
