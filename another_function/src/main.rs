fn main() {
    println!("Hello, world!");
    another_function();
    let mut x = 5;
    x = plus_one(x);
    println!("The value of x is {}", x);

    // loop
    let y = loop {
        x = plus_one(x);
        if x == 10 {
            break x * 2;
        }
    };
    println!("The value of y is {}", y);

    // for
    let a = [1, 2, 3, 4, 5];
    let mut index = 1;
    while index < a.len() {
        println!("The value of a[{}] is {}", index, a[index]);
        index += 1;
    }

    for b in a.iter() {
        println!("{}", b);
    }

    for c in 1..4 {
        println!("{}", c);
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function() {
    println!("Another function");
}
