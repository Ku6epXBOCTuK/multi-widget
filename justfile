fill:
	cargo run -p server --bin fixtures

gen:
	cargo run -p shared generate-types
	