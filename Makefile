.PHONY:
	clean build_webapp build_server build clippy fmt test check run all

all: clean performance-calc-server

clean:
	rm -rf target
	rm -rf dist
	rm -rf app/pkg app/target
	rm -rf server/target

build_webapp: app/src
	cd app && wasm-pack build --target web --no-pack

build_server: server/src
	cd server && cargo build -r

build: build_webapp build_server

performance-calc-server: build
	rm -rf dist
	mkdir -p dist/pkg
	mv app/pkg/*.wasm app/pkg/*.ts app/pkg/*.js dist/pkg
	mv target/release/performance-calc-server dist/performance-calc-server
	cp -r public dist/public

clippy:
	cargo clippy

fmt:
	cargo fmt

test:
	cargo test

check: clippy fmt test

run: performance-calc-server
	./dist/performance-calc-server
