Examples
=================
<!--ts-->
   * [Usage](#usage)
   * [Docker](#docker)
   * [Artifact](#artifact)
<!--te-->

## Usage 
`excalidocker` supports 2 running modes:
   - `show-config` - show configuration file content. This option could be handy when utility is running as a docker image, 
   as it generates the config template for further customization.
   - `input-path` - path for the `docker-compose.yaml` to perform the conversion.

To get the help menu execute:
 
 - for docker image
```sh
docker run --rm  etolbakov/excalidocker:latest
```
 - for artifacts
```sh
excalidocker
```

## Docker
1. Convert a local file
Specify the path to the `docker-compose.yaml` as a `INPUT_PATH` environment variable for the docker image.

 ```sh
docker run --rm --pull always \
           -v "$(pwd)/data/compose/:/tmp/" \
           -e INPUT_PATH=/tmp/docker-compose.yaml \
           etolbakov/excalidocker:latest \
           > produced-by-image.excalidraw
```

2. Convert a remote file (conversion via github link)
Specify the url to the `docker-compose.yaml` as a `INPUT_PATH` environment variable for the docker image.
Both the github link and the link to the raw file work.

```sh
docker run --rm --pull always \
           -e INPUT_PATH=https://github.com/apache/pinot/blob/master/docker/images/pinot/docker-compose.yml \
           etolbakov/excalidocker:latest \
           > produced-by-image-remote.excalidraw
```

```sh
docker run --rm --pull always \
           -e INPUT_PATH=https://raw.githubusercontent.com/apache/pinot/master/docker/images/pinot/docker-compose.yml \
           etolbakov/excalidocker:latest \
           > produced-by-image-remote.excalidraw
```

3. Convert a remote file and skip dependency links.
Set `SKIP_DEPS=true` to switch off dependency arrows

```sh
docker run --rm --pull always \
           -e INPUT_PATH=https://raw.githubusercontent.com/apache/pinot/master/docker/images/pinot/docker-compose.yml \
           -e SKIP_DEPS=true \
           etolbakov/excalidocker:latest \
           > produced-by-image-no-deps.excalidraw
```

4. Convert a local file with the provided config
Specify the path to the  configuration file `excalidocker-config.yaml` as a `CONFIG_PATH` environment variable for the docker image.
```sh
   docker run --rm --pull always \
             -v "$(pwd)/data/compose/:/tmp/" \
             -v "$(pwd)/excalidocker-config.yaml:/tmp/excalidocker-config.yaml" \
             -e INPUT_PATH=/tmp/docker-compose.yaml \
             -e CONFIG_PATH=/tmp/excalidocker-config.yaml \
             etolbakov/excalidocker:latest \
             > produced-by-image-config-deps.excalidraw
```

5. Show config
Specify the `SHOW_CONFIG` environment variable. This command can be handy to generate a config for further customization.
```sh
docker run --rm --pull always \
            -e SHOW_CONFIG=true \
            etolbakov/excalidocker:latest \
            > excalidocker-config.yaml
```

## Artifact
> **Warning**
>  The `excalidocker` config precedence:
> 
>   - the highest priority takes command line argument `--config-path`
> 
>   - otherwise `excalidocker` expects the `excalidocker-config.yaml` to be placed near the executable (as per the `DEFAULT_CONFIG_PATH` constant in [excalidraw_config.rs](./provide_link))
> 
>   - otherwise `excalidocker` uses the default configuration (as per the `DEFAULT_CONFIG` constant in [excalidraw_config.rs](./provide_link))

1. Convert a local file.
Specify the path to the `docker-compose.yaml` as a `--input-path` command line argument.

```sh
excalidocker --input-path ./data/compose/docker-compose.yaml
```

2. Convert a local file and produce the result into another file
Specify the `--input-path` argument pointing to the `docker-compose.yaml` and `--output-path` for the result.

```sh
excalidocker --input-path ./data/compose/docker-compose.yaml --output-path /tmp/result.excalidraw
```

3. Convert a remote file (conversion via github link)
Specify the url to the `docker-compose.yaml` as a `--input-path` command line argument.
Both the github link and the link to the raw file work.

```sh
excalidocker --input-path https://github.com/apache/pinot/blob/master/docker/images/pinot/docker-compose.yml
```
```sh
excalidocker --input-path https://raw.githubusercontent.com/apache/pinot/master/docker/images/pinot/docker-compose.yml
```

4. Convert a remote file and skip dependency links.
Specify `--skip-dependencies` command line argument.

```sh
excalidocker --skip-dependencies --input-path https://github.com/apache/pinot/blob/master/docker/images/pinot/docker-compose.yml
```

5. Convert a local file with the provided config

```sh
excalidocker --config-path /tmp/excalidocker-config.yaml --input-path https://github.com/apache/pinot/blob/master/docker/images/pinot/docker-compose.yml
```
	
