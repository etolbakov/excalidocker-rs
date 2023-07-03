
current_dir = $(shell pwd)
docker_image_tag = latest

##########################################################################################
## Development commands
########################################################################################## 

r:
	cargo build --release
s:
	./target/release/excalidocker -h

check:
	cargo check --all-features

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings	

##########################################################################################
## Sample execution commands
########################################################################################## 
## currently the Makefile commands follow the convention below:
## <execute><id><arguments>
## 'e' - execute with full option for command line arguments
## 'x' - execute with short option for command line arguments
## 'd' - execute with docker image
## 'i' - provided  '--input-path' argument
## 'o' - provided  '--output-path' argument
## 's' - provided  '--skip-dependencies' argument
## 'c' - provided  '--config-path' argument
## 'r' - provided  '--input-path' argument has a link to an external (github) file
## 'cfg' - 		   '--show-config' argument	

ecfg:
	./target/release/excalidocker -C

e1i:
	./target/release/excalidocker --input-path ./data/compose/docker-compose.yaml
e1io:
	./target/release/excalidocker --input-path ./data/compose/docker-compose.yaml --output-path /tmp/result.excalidraw

e2i:
	./target/release/excalidocker --input-path ./data/compose/docker-compose-large.yaml

e3o:
	./target/release/excalidocker --output-path /tmp/no.excalidraw

e4i:
	./target/release/excalidocker --input-path ./data/compose/docker-compose-very-large.yaml
e4io:
	./target/release/excalidocker --input-path ./data/compose/docker-compose-very-large.yaml --output-path $(shell pwd)/docker-compose-very-large.excalidraw
x4io:
	./target/release/excalidocker -i ./data/compose/docker-compose-very-large.yaml -o $(current_dir)/docker-compose-very-large.excalidraw	
e4ios:
	./target/release/excalidocker --skip-dependencies --input-path ./data/compose/docker-compose-very-large.yaml --output-path $(shell pwd)/docker-compose-very-large.excalidraw

d5i:
	docker run --rm -v "$(current_dir)/data/compose/:/tmp/" -e INPUT_PATH=/tmp/docker-compose.yaml etolbakov/excalidocker:$(docker_image_tag) > produced-by-image.excalidraw
d5ir:
	docker run --rm -v "$(current_dir)/data/compose/:/tmp/" -e INPUT_PATH=https://github.com/apache/pinot/blob/master/docker/images/pinot/docker-compose.yml etolbakov/excalidocker:$(docker_image_tag) > produced-by-image-remote.excalidraw
d5is:
	docker run --rm -v "$(current_dir)/data/compose/:/tmp/" -e INPUT_PATH=/tmp/docker-compose.yaml -e SKIP_DEPS=true etolbakov/excalidocker:$(docker_image_tag) > produced-by-image-no-deps.excalidraw
d5ic:
	docker run --rm -v "$(current_dir)/data/compose/:/tmp/" -v "$(current_dir)/excalidocker-config.yaml:/tmp/excalidocker-config.yaml" -e INPUT_PATH=/tmp/docker-compose.yaml -e CONFIG_PATH=/tmp/excalidocker-config.yaml etolbakov/excalidocker:$(docker_image_tag) > produced-by-image-config-deps.excalidraw
