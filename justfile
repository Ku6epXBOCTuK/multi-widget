fill:
	cargo run -p server --bin fixtures

gen:
	cargo run -p shared generate-types

sse:
	curl -N http://localhost:3000/sse
