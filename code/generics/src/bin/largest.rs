fn largest<T: std::cmp::PartialOrd>(num1: T, num2: T) -> (T, T) {
    if num1 > num2 {
        (num1, num2)
    } else {
        (num2, num1)
    }
}

fn main() {
    let n1 = 7.523;
    let n2 = 6.89;

    let lgst = largest(n1, n2);
    println!("{} >= {}", lgst.0, lgst.1);
}
