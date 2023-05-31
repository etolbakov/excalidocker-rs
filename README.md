# excalidocker-rs
Convert your docker-compose into excalidraw
Rust-based utility to convert docker-compose.yaml files into [excalidraw](https://excalidraw.com/) files.

## Installation

1. Install Rust and Cargo if you haven't already. Refer to the official Rust documentation for [installation instructions](https://www.rust-lang.org/tools/install):
2. Clone this repository:
```shell
git clone https://github.com/etolbakov/excalidocker-rs.git
```
3. Build the project using Cargo:
```shell
cd excalidocker-rs
cargo build --release
```

## Usage

To convert a docker-compose.yaml file into excalidraw, run the following command:

```shell
cargo run --release -- path/to/docker-compose.yaml
```

This will generate an excalidraw file named `docker-compose.excalidraw`.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](./LICENSE).
Feel free to customize the text based on your project's specific details, such as repository URLs, installation instructions, and contribution guidelines.
