fn main() {
    //
    let c = f_to_c(32);
    println!("{c}");
    let f = c_to_f(0);
    println!("{f}");

}

fn c_to_f (x: i32) -> i32 {
    ((9/5) * x) + 32
}

fn f_to_c (x: i32) -> i32 {
    (x - 32) * (5/9)
}