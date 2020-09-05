// Challenge 1: Identity function in Rust

fn id<T>(x: T) -> T {
    x
}

fn main() {
    println!("id(42) = {}", id(42));
    println!("id(\"hello world!\") = {}", id(String::from("hello world!")));
}
