use std::collections::HashMap;

fn main() {
    let mut ratings = HashMap::new();

    let milton = String::from("Milton");
    let pexpo = String::from("Pexpo");

    ratings.insert(milton, 4.5);
    ratings.insert(pexpo, 4.2);

    let milton_rating = ratings.get("Milton").copied().unwrap_or(0f64);

    println!("{ratings:#?}");
    println!("Rating for Milton: {milton_rating}");

    println!("{}", milton); // error, value moved
}
