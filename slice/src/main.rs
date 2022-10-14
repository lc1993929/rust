fn main() {
    let v = vec![1, 2, 3, 4];

    println!("v:{:?}", v);
    // 切片长度是未知的，所以必须使用指针来传递
    let v1 = &v[1..];
    println!("v1:{:?}", v1);
    borr1(v1);
    println!("v1:{:?}", v1);

    let mut w = vec![1, 2, 3, 4];
    // 可变指针切片，调用函数时不发生move
    let v2 = &mut w[1..];
    borr2(v2);
    println!("v2:{:?}", v2);
    println!("w:{:?}", w);
}

// 切片相当于指针，所以函数传递的时候属于借用
fn borr1(v1: &[i32]) {
    println!("v1:{:?}", v1);
}

fn borr2(v2: &mut [i32]) {
    v2[0] = 5;
    println!("v1:{:?}", v2);
}
