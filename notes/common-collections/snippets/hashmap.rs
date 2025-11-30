use std::collections::HashMap;

fn main() {
    let mut ratings = HashMap::new();

    ratings.insert("Milton", 4.5);
    ratings.insert("Pexpo", 4.5);

    let milton_rating = ratings.get("Milton").copied().unwrap_or(0f64);

    println!("{ratings:#?}");
    println!("Rating for Milton: {milton_rating}");
}
