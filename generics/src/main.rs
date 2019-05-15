fn largest(list: &[i32]) -> i32 {
    let mut max = list[0];

    for number in list {
        if *number > max {
            max = *number;
        }
    }

    max
}

fn main() {
    let number_list = vec![1, 2, 3, 4, 5];
    println!("The largest is {}", largest(&number_list));
}
