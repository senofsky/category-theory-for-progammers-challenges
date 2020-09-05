fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add_one = |x| x + 1;
    let times_two = |x| x * 2;
    println!("compose(add_one, add_one)(0) = {}", compose(add_one, add_one)(0));
    println!("compose(times_two, times_two)(1) = {}", compose(times_two, times_two)(1));
}
