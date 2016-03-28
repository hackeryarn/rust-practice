struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let origin = Point { x: 0, y: 0 };

    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);
}
