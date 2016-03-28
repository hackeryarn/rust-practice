struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

struct Inches(i32);

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

    let mut point3d = Point3d { x: 0, y: 0, z: 0 };

    point3d = Point3d { y: 1, .. point3d };

    println!("The 3d point is at ({}, {}, {})", point3d.x, point3d.y, point3d.z);

    let length = Inches (10);

    let Inches(integer_length) = length;

    println!("length is {} inches", integer_length);
}
