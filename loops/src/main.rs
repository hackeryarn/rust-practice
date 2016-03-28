fn main() {
    while_loop();
    for_loop();
    just_loop();
    odd_combo();
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

    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}

fn just_loop() {
    let mut x = 5;
    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }
}

fn odd_combo() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            }
            if y % 2 == 0 {
                continue 'inner;
            }
            println!("x: {}, y: {}", x, y);
        }
    }
}
