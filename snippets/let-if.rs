fn main() {
    let mut selector = 0;

    while selector <= 3 {
        let x = if selector == 0 {
            2
        } else if selector == 1 {
            3
        } else if selector == 2 {
            4
        } else {
            5
        };
        println!("Value of x is: {x}");
        selector += 1;
    }
}
