# excalidocker-rs
Rust-based utility to convert docker-compose.yaml files into [excalidraw](https://excalidraw.com/) files.
![excalidocker](./data/img/excalidocker.png)

# Table of Contents
1. [Usage](#usage) 
2. [Installation](#installation)
3. [Contributing](#Contributing)
4. [Roadmap](#roadmap)

## Usage
0. Download the artifact from [releases](https://github.com/etolbakov/excalidocker-rs/releases) and ungzip it. 
To get the `help` menu use:
```sh
./excalidocker -h
```
![release-artifact-output](./data/img/release-artifact-output.png)
The application should be provided with two parameters: 
 - `--input-path` :  `docker-compose.yaml` file path that you would like to convert 
 - `--output-path` : file path for the output excalidraw file. By default a file is be stored under "/tmp/<docker-compose-file-name>.excalidraw"


To see the tool in action use:
```sh
./excalidocker --input-path /your/path/docker-compose.yaml --output-path /your/path/result.excalidraw
```
The produced file could be opened in [excalidraw](https://excalidraw.com/) and.... hopefully it won't be too shocking 👻 😅.

> **Warning**
> On the first launch the ungzipped artifact I saw the following pop up
> "Mac cannot be opened because it is from an unidentified developer" 
> If you are fine with that you can `Control-click` the artifact, then choose `Open` from the shortcut menu. 
> Click `Open`. The utility will be saved as an exception to your security settings, 
> and you can open it in the future by double-clicking it just as you can any registered app.
![mac-warning](./data/img/mac-warning.png)


> **Note**
> At the moment only darwin is supported.

## Installation
If you'd like to build `excalidocker` locally follow these steps:

1. Install Rust and Cargo if you haven't already. Refer to the official Rust documentation for [installation instructions](https://www.rust-lang.org/tools/install):
2. Clone this repository:
```shell
git clone https://github.com/etolbakov/excalidocker-rs.git
```
3. Build the project using Cargo:
```shell
cd excalidocker-rs && cargo build --release
```
There is the `make r` command available in the [Makefile](/Makefile) along with other useful command shortcuts.


## Roadmap
These are the features that I would like to add
 - 📊 visualize more data from docker-compose (volumes, network, etc)
 - 🐳 docker artifact 
 - 🐧 linux artifact
 - 🎨 colour output
 - etc

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](./LICENSE).
Feel free to customize the text based on your project's specific details, such as repository URLs, installation instructions, and contribution guidelines.
