# Makefile for building my solutions for Category Theory for Programmers

identity_function_in_rust: identity_function.rs
	rustc identity_function.rs -o identity_function_in_rust

clean:
	rm identity_function_in_rust
