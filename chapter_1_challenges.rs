// This module contains my solutions to the challenges in chapter 1 of Category
// Theory for Programmers

// Challenge 1
// Implement, as best as you can, the identity function in your favorite language (or the second favorite, if your favorite language happens to be Haskell).
fn id<T>(x: T) -> T {
    x
}

// Challenge 2
// Implement the composition function in your favorite language. It takes two functions as arguments and returns a function that is their composition.
fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// Challenge 3
// Write a program that tries to test that your composition function respects identity.
fn main() {
    println!("compose(id, id)(42) = {}", compose(id, id)(42));
}

// Challenge 4
// Is the world-wide web a category in any sense? Are links morphisms?
//
// Composition in a category must:
// - be associatevely composable: f . g . h == (f . g ) . h == f . (g . h)
// - have an identity arrow to point back at itself
//
// If every website is an object and:
//  - has a link that points to itself (identity)
//  - has composable links (ignoring authentication mechanisms)
// Then the world-wide web is technically a category.

// Challenge 5
// Is Facebook a category, with people as objects and friendships as morphisms?
// Given:
//   Person A is friends with Person B
//   Person B is friends with Person C
// But Person A is not necessarily friends with Person C
// Therefore, friendships fail to compose, and thus, Facebook is not a category.

// Challenge 6
// When is a directed graph a category?
// Each node in the graph has a single arrow that points to itself.
// With DAG, A -> B and B -> C implies A -> C
