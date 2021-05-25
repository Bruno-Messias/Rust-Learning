
const MAX_SCORES: u32 = 100_000;
use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("This is a const: {}", MAX_SCORES);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {}", y);
    println!("First Element of tuple: {}", tup.0);

    let a = [3; 5]; //Same as [3;3;3;3;3]

    println!("First Element of an array: {}",a[0]);

    // A bad array indexing by user --Panic
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
