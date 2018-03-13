extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("猜数字游戏");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("生成的秘密数字是：{}", secret_number);

    loop {
        println!("请输入你猜的数字：");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("读取输入失败！");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("恭喜你猜中了！");
                break;
            }
        }
    }
}
