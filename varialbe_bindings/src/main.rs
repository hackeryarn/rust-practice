fn main() {
    let x: i32 = 8;

    {
        println!("{}", x);
        let x = 12;
        println!("{}", x);
    }

    println!("{}", x);
    let x = 42;
    println!("{}", x);

    let y: i32 = 4;
    println!("{}", y);
    let y = "text";
    println!("{}", y);
}
