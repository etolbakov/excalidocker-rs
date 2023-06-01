# excalidocker-rs
Rust-based utility to convert docker-compose.yaml files into [excalidraw](https://excalidraw.com/) files.
![excalidocker](./data/img/excalidocker.png)


## Installation

1. Install Rust and Cargo if you haven't already. Refer to the official Rust documentation for [installation instructions](https://www.rust-lang.org/tools/install):
2. Clone this repository:
```shell
git clone https://github.com/etolbakov/excalidocker-rs.git
```
3. Build the project using Cargo:
```shell
cd excalidocker-rs && cargo build --release
```
There is the `make r` command available in the [Makefile](/Makefile) 

## Usage
0. Download the artifact from [releases](https://github.com/etolbakov/excalidocker-rs/releases) and ungzip it. 
Use the following command to get the help menu:
```sh
./excalidocker -h
```
![release-artifact-output](./data/img/release-artifact-output.png)

The following command can be used to see the tool in action:
```sh
./excalidocker --input-path /your/path/docker-compose.yaml --output-path /your/path/result.excalidraw
```

> **Warning**
> On the first launch the ungzipped artifact I saw the following pop up
> "Mac cannot be opened because it is from an unidentified developer" 
> If you are fine with that you can `Control-click` the artifact, then choose `Open` from the shortcut menu. 
> Click `Open`. The utility will be saved as an exception to your security settings, 
> and you can open it in the future by double-clicking it just as you can any registered app.
![mac-warning](./data/img/mac-warning.png)


> **Note**
> At the moment only darwin is supported.

1. The application supports two input parameters. Run
```sh
./target/release/excalidocker -h
```
or `make s` to check the output which should look like:

```sh
./target/release/excalidocker -h
Utility to convert docker-compose into excalidraw

Usage: excalidocker [OPTIONS] --input-path <INPUT_PATH>

Options:
  -i, --input-path <INPUT_PATH>    file path to the docker-compose.yaml
  -o, --output-path <OUTPUT_PATH>  file path for the output excalidraw file. By default a file is be stored under "/tmp/<docker-compose-file-name>.excalidraw"
  -h, --help                       Print help
  -V, --version                    Print version
```

2. To see how this tool converts a `docker-compose.yaml` file into `excalidraw`, run the following command:
```sh
./target/release/excalidocker --input-path ./data/compose/docker-compose.yaml --output-path /your/path/result.excalidraw
```
or it's shortcut `make e1` as an alternative. This will generate the `result.excalidraw` file under the provided path.

`--output-path` could be omitted, in which case the file would inherit the name of the original yaml and will be placed under `/tmp` folder.

The produced file could be opened in [excalidraw](https://excalidraw.com/) and.... hopefully it won't be too shocking 👻 😅.

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](./LICENSE).
Feel free to customize the text based on your project's specific details, such as repository URLs, installation instructions, and contribution guidelines.
