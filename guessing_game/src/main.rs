use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("猜数字");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("神秘数字是：{secret_number}");
    loop {
        println!("请输入你的猜测的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("未检测到输入");
        let guess = match guess.trim().parse::<u32>() {
            Ok(guess_num) => guess_num,
            Err(_) => {
                println!("请输入数字");
                continue;
            }
        };

        println!("你猜测的数字是：{guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("小了")
            }
            Ordering::Equal => {
                println!("对了");
                break;
            }
            Ordering::Greater => {
                println!("大了")
            }
        }
    }
}
