struct Point<T> {
    // in a real app, use the num_traits crate
    x: T,
    y: T,
}

impl Point<f64> {
    fn dist_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 1.6, y: 7.2 };

    println!("{}", p.dist_from_origin());
}
