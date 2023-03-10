fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("{x}");

}

fn another_function(x: i32) {
    println!("Another function. {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Measurement is {value}{unit_label}");
}

fn five() -> i32 {
    5
}