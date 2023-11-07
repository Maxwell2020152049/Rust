use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("猜数游戏!");

    let secret_number = rand::thread_rng().gen_range(1..=101);

    loop {
        println!("请输入一个1-100内的数:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取错误。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
