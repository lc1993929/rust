fn main() {
    hello_world();
    let world2 = hello_world2();
    println!("world2: {}", world2);
    let mut price = 10;
    double(price);
    println!("price: {}", price);
    double2(&mut price);
    println!("price: {}", price);

    //     tuple
    let t = ("test1", "test2");
    println!("t: {:?}", t);
    println!("t: {:?}", t.0);
    let (t1, _t2) = t;
    println!("t1: {:?}", t1);
    //     array
    let a = [1, 2, 3];
    println!("a: {:?}", a);
    println!("a: {:?}", a[0]);
    let a2 = ["a2"; 3];
    println!("a2: {:?}", a2);
}

fn hello_world() {
    println!("Hello, world!");
}

fn hello_world2() -> String {
    return String::from("Hello, world!");
}

// 这里传递进来的参数与调用时传递的变量不是同一个，可以理解为将调用的变量赋值给了参数变量（值传递）
fn double(mut price: u32) {
    price = price * 2;
    println!("price: {}", price);
}

// 相当于传递进来一个可变变量的指针
// *price相当于转移了所有权
fn double2(price: &mut u32) {
    *price = *price * 2;
    println!("price: {}", price);
}
