fill:
	cargo run -p server --bin fixtures

gen:
	cargo run -p shared generate-types

run:
	cargo run -p server

sse:
	curl -N http://localhost:3000/sse
