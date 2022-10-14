fn main() {
    // let width = 30;
    // let height = 50;

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("总面积是:{}", area2(&rectangle));
    // 两种调用方法没有本质上的区别，方法的第一个参数决定了是否可以使用.的方式调用方法
    println!("总面积是:{}", rectangle.area());
    println!("总面积是:{}", Rectangle::area(&rectangle));
    println!("{:#?}", rectangle);
    dbg!(&rectangle);
    dbg!(rectangle);
    println!("{}", Rectangle::test());

    let dice_roll = 9;
    match dice_roll {
        3 => println!("{}", 3),
        7 => println!("{}", 7),
        test => println!("{}", test),
    }
}

/*fn area(width: u32, height: u32) -> u32 {
    width * height
}*/

// 注意这里参数只能传递引用，否则在返回时会释放掉参数，如果在调用处继续使用参数对象，就会编译不通过
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn test() -> u32 {
        6
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        _ => None,
    }
}
