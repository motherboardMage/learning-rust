fn main() {
    let mut count = 0;

    while count < 5 {
        let x = match count {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            _ => 5,
        };

        println!("Value of x is: {x}");
        count += 1;
    }
}
