use std::io;

fn main() {
    let x = 2.0; // default f64

    let y: f32 = 3.0;

    let f: bool = false;

    let z: char = 'Z';

    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    println!("The value of y is: {five_hundred}");

    let c: [i32; 5] = [1,2,3,4,5];

    let d = [3; 5];

    let a = [1,2,3,4,5];

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

    println!("The value of the element at index {index} is: {element}");

}