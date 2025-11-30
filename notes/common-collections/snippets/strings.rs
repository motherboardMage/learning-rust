fn main() {
    let mut str1 = String::from("Somebody");
    let str2 = String::from(" was here");
    let str3 = ", I guess...";

    str1.push_str(&str2);
    str1.push_str(str3);

    str1.push('\n');
    str1.push('W');
    str1.push('e');
    str1.push(' ');
    str1.push('c');
    str1.push('a');
    str1.push('n');
    str1.push(' ');
    str1.push('p');
    str1.push('u');
    str1.push('s');
    str1.push('h');
    str1.push(' ');
    str1.push('c');
    str1.push('h');
    str1.push('a');
    str1.push('r');
    str1.push('s');
    str1.push('!');

    println!("\nstr1 is {str1}");
    println!("\nstr2 is {str2}");
    println!("\nstr3 is {str3}");
}
