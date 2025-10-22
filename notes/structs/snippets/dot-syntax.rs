struct Point(i32, i32, i32);

fn main() {
    let somePoint = Point(12, 43, 89);

    let Point(x, y, z) = somePoint;

    somePoint.1 = 32;
}
