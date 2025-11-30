fn main() {
    let mut str1 = String::from("Is it");
    let str2 = String::from("too late ");

    str1 = str1 + " " + &str2;
    println!("{}", str1);

    let str3 = str1 + "to turn back now?";
    println!("{}", str3);
    // println!("{}", str1);
}
