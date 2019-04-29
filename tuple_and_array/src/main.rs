fn main() {
    // create a tuple
    let tuple: (u32, f32, bool) = (500, 3.14, true);
    // access each value of tuple
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;
    println!("The value of tuple is ({}, {}, {})", x, y, z);
    // destructure a tuple
    let (j, k, l) = tuple;
    println!("The value of tuple is ({}, {}, {})", j, k, l);

    // creat a array
    let a: [u32; 3] = [1, 2, 3];
    // `[{integer}; 3]` cannot be formatted with the default formatter
    // println!("The value of array a is {}", a);
    // let b = [5; 3];
    // `[{integer}; 3]` cannot be formatted with the default formatter
    // println!("The value of array b is {}", b);
    // access each value of array
    let x1 = a[0];
    let x2 = a[1];
    let x3 = a[2];
    println!("The value of each element of array is {}, {}, {}", x1, x2, x3);
}
