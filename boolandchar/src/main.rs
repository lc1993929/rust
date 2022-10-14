fn main() {
    let b: bool = true;
    let c: char = 'c';
    println!("b :{}", b);
    println!("c :{}", c);

    static BOOK: &'static str = "test";
    println!("BOOK :{}", BOOK);
//     string
    let lesson: &str = "lesson";
    println!("lesson :{}", lesson);
    let mut str1 = String::new();
    println!("str1 :{}", str1);
    str1 = String::from(lesson);
    println!("str1 :{}", str1);
    str1.push_str("push1");
    println!("str1 :{}", str1);
    for x in str1.chars() {
        println!("x :{}", x);
    }

    let code = "2";
    let x1 = match code {
        "1" => { "test1" }
        _ => { "test2" }
    };
    println!("x1 :{}", x1);

    for i in 0..=5 {
        println!("i :{}", i);
    }

    let list = vec!["test1", "test2"];
    println!("list :{:?}", list);
    for x in list {
        println!("x :{}", x);
    }
}

