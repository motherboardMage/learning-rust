struct Person {
    name: String,
    email: String,
    age: u8,
}

fn main() {
    let mut person1 = Person {
        name: String::from("some_offensive_name"),
        email: String::from("stupid@email.com"),
        age: 52,
    };
    person1.name = String::from("more_offensive_name");

    println!(
        "name: {}\ne-mail: {}\nage: {}\n",
        person1.name, person1.email, person1.age
    );

    let person2 = create_new_person(String::from("somebody"), String::from("iused@to.know"), 45);

    println!(
        "name: {}\ne-mail: {}\nage: {}\n",
        person2.name, person2.email, person2.age
    );

    let person3 = Person {
        name: String::from("Pandey"),
        ..person2
    };

    println!(
        "name: {}\ne-mail: {}\nage: {}",
        person3.name, person3.email, person3.age
    );
}

fn create_new_person(name: String, email: String, age: u8) -> Person {
    Person { name, email, age }
}
