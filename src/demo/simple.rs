pub mod hosting {
    pub fn add_to_wait_list() {}
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(mut a_string: String) -> String {
    a_string.push_str(", world");
    return a_string;
}

pub enum Sex {
    MALE,
    FEMALE,
}

pub struct Person {
    pub name: String,
    pub age: i8,
    pub sex: Sex,
    pub email: String,
}

fn new_person() -> Person {
    let person = Person {
        name: String::from("Hao"),
        age: 12,
        sex: Sex::MALE,
        email: String::from("abc@email.com"),
    };
    return person;
}

mod foo {
    #[derive(Debug)]
    pub struct Foo {
        s: &'static str,
    }

    impl Foo {
        pub fn new(s: &'static str) -> Foo {
            Foo { s }
        }
    }
}

fn main() {
    let s1 = gives_ownership();
    takes_ownership(s1);

    // 如果编译下面的代码，会出现s1不可用的错误
    // println!("s1= {}", s1);
    //                    ^^ value borrowed here after move
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // 如果编译下面的代码，会出现可不可用的错误
    // println!("s2={}, s3={}", s2, s3);
    //                         ^^ value borrowed here after move
    println!("s3={}", s3);

    // new_person内部的person被move到main函数中
    let _p = new_person(); // with _ prefix, no warning of unused variable

    let f = foo::Foo::new("hello");
    println!("{:?}", f)
}

mod bm {
    fn borrow(s: &String) {
        println!("{}", s);
    }

    fn borrow_mut(s: &mut String) {
        s.push_str(" zzz");
        println!("{}", s)
    }

    fn main() {
        let s = String::from("123");
        borrow(&s);
        borrow(&s);

        let mut z = String::from("zzz");
        borrow_mut(&mut z);

    }
}