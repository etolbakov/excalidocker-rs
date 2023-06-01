aa:
	pwd
r:
	cargo build --release
s:
	./target/release/excalidocker -h
e1:
	./target/release/excalidocker --input-path ./data/compose/docker-compose.yaml --output-path /tmp/result.excalidraw
e2:
	./target/release/excalidocker --input-path ./data/compose/docker-compose-large.yaml
e3:
	./target/release/excalidocker --output-path /tmp/no.excalidraw

check:
	cargo check --all-features

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings
