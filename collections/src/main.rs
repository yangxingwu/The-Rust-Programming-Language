fn main() {
    // let v = vec![1, 2, 3, 4, 5];
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third = v[2];
    println!("The third value of vector is {}", third);
    println!("The third value of vector is {}", v[2]);

    match v.get(2) {
        Some(third) => println!("The third value of vector is {}", third),
        None => println!("There is no third argument"),
    }

    let hello = "Здравствуйте";
    let answer = &hello[0];
    println!("The answer is {}", answer);
}
