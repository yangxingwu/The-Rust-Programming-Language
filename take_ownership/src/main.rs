fn main() {
    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("{}, world", s1);
    // println!("{}, world", s2);
    let s = String::from("Hello, world");
    take_ownership(s);

    let x = 5;
    make_copy(x);

    let s = String::from("hello, world");
    let (s2, len) = calculate_length_1(s);
    println!("The len of {} is {}", s2, len);

    let len2 = calculate_length_2(&s2);
    println!("The len of {} is {}", s2, len2);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length_1(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}

fn calculate_length_2(s: &String) -> usize {
    return s.len();
}
