fn main() {
    let mut student = Student {
        name: String::from("张三"),
        age: 33,
    };
    println!("student: {:?}", student);
    println!("name:{},age:{}", student.name, student.age);
    student.age = 34;
    println!("name:{},age:{}", student.name, student.age);
    let student1 = Student::new(String::from("李四"), 44);
    println!("student1: {:?}", student1);
    println!("{}", student1.name());

    //     enum
    let x = Type::Teacher;
    let y = Type::Student;
    println!("x:{:?}", x);
    println!("y:{:?}", y);
    //     Option
    let option = is_student(10);
    println!("option:{:?}", option);
    match option {
        None => {
            println!("没找到")
        }
        Some(value) => {
            println!("value:{}", value);
        }
    }

    let name = Name::Name(String::from("test"));
    match name {
        Name::Name(value) => {
            println!("name: {:?}", value);
        }
    }

    let c1 = Color(1, 2);
    println!("{}", c1.0);
}

struct Color(i32, i32);

#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
}

impl Student {
    fn new(name: String, age: i32) -> Self {
        Student { name, age }
    }

    fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
enum Type {
    Student,
    Teacher,
}

// Option<bool> represents
fn is_student(age: i32) -> Option<bool> {
    if age < 18 {
        Some(true)
    } else {
        None
    }
}

enum Name {
    Name(String),
}
