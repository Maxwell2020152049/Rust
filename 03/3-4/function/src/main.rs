fn main() {
    println!("Hello, world!");
    hello_rex();

    let x: i32 = 5;
    println!("value of x is {}", plus_five(x));
}

fn hello_rex() {
    println!("Hello, Rex!");
}

fn plus_five(x: i32) -> i32 {
    x + 5
}