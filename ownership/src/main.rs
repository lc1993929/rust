fn main() {
    let mut a = vec![1, 2];
    println!("a:{:?}", a);
    let b = a;
    // println!("{:?}", a);
    println!("b:{:?}", b);
    a = move_fun(b);
    println!("a;{:?}", a);
    // println!("b:{:?}", b);
    borr1(&a);
    println!("a;{:?}", a);
    borr2(&mut a);
    println!("a;{:?}", a);

    let x = 32;
    let y = x;
    println!("x:{:?}", x);
    println!("y:{:?}", y);
}

fn move_fun(a: Vec<i32>) -> Vec<i32> {
    println!("a:{:?}", a);
    return a;
}

//指针传递，不会影响调用方的所有权
fn borr1(ab1: &Vec<i32>) {
    println!("ab1:{:?}", ab1);
}

// 可变变量指针传递，不影响调用方所有权
fn borr2(ab2: &mut Vec<i32>) {
    ab2.push(3);
    println!("ab2:{:?}", ab2);
}
