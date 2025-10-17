fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    let s1 = String::from("Nooooo");

    println!("{s2}");
    println!("{s1}");

    takes_ownership(s1);
    // s1 does not exist anymore

    does_not(&s2);

    println!("{s1}");
    println!("{s2}");
}

fn takes_ownership(something: String) {
    println!("Took ownership of {something}");
} // Passed String is dropped here

fn does_not(something: &String) {
    println!("Did not take ownership of {something}");
} // String was only borrowed here so nothing is dropped
