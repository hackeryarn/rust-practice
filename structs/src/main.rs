struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32
}

fn main() {
    let origin = Point { x: 0, y: 0 };

    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 6;
        *r.y = 7;
    }

    println!("The new point is at ({}, {})", point.x, point.y);
}
