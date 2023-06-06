
current_dir = $(shell pwd)

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
	docker run --rm -v "$(current_dir)/data/compose/:/tmp/" -e INPUT_PATH=/tmp/docker-compose.yaml etolbakov/excalidocker:0.1.3-20230606 > produced-by-image.excalidraw
