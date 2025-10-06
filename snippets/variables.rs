fn main() {
    let x = 5;
    const PI: f64 = 3.14;

    {
        let x = 7.6;
        let x = x * 2f64;
        println!("Value of x in inner scope: {x}");

        println!("Assume pi is 4");
        const PI: u32 = 4;
        println!("Value of pi in inner scope is: {PI}");
    }

    println!("Value of x: {x}");
    println!("Value of pi is: {PI}");
}
