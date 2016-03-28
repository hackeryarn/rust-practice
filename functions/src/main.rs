fn main() {
    print_number(5);
    print_sum(5, 6);
    print_add_one(5);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn print_add_one(x: i32) {
    println!("add one is: {}", add_one(x));
}

fn add_one(x: i32) -> i32 {
    x + 1
}
