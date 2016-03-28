fn main() {
    while_loop();
    for_loop();
}

fn while_loop() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

fn for_loop() {
    for x in 0..10 {
        println!("{}", x);
    }

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}
