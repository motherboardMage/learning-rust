#[derive(Debug)]
struct AB<T, U> {
    a: T,
    b: U,
}

fn main() {
    let obj = AB { a: 'S', b: 3.2 };

    println!("a = {}\nb = {}", obj.a, obj.b);
}
