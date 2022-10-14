use std::time::Duration;

use async_std::task;

#[async_std::main]
async fn main() {
    /*    thread::spawn(move || {
        for i in 0..10 {
            println!("{}", i);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for i in 0..5 {
        println!("main:{}", i);
        thread::sleep(std::time::Duration::from_secs(2));
    }*/

    let do3_async = task::spawn(do3());
    do4().await;
    do3_async.await;
}

async fn do3() {
    for i in 0..5 {
        println!("do3:{}", i);
        task::sleep(Duration::from_secs(1)).await;
    }
}

async fn do4() {
    for i in 0..5 {
        println!("do4:{}", i);
        task::sleep(Duration::from_secs(1)).await;
    }
}
