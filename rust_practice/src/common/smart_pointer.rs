//! 在 Rust 中，凡是需要做资源回收的数据结构，且实现了Deref/DerefMut/Drop，都是智能指针。
#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use std::{
        borrow::{Borrow, Cow},
        ops::Deref,
    };

    use serde::Deserialize;

    #[derive(Debug, Default)]
    struct UserSample {
        username: String,
        password: String,
    }

    #[test]
    fn boxi() {
        let mut boxed_obj = Box::<UserSample>::default();
        // auto deref mut
        boxed_obj.username = String::from("username");
        boxed_obj.password = "password".to_owned();
        drop(boxed_obj)
    }

    fn remove_whitespace(s: &str) -> String {
        s.to_string().replace(' ', "")
    }

    fn remove_ws_cow(s: &str) -> Cow<str> {
        if s.contains(' ') {
            Cow::Owned(s.to_string().replace(' ', ""))
        } else {
            Cow::Borrowed(s)
        }
    }
    // 包裹一个只读借用，但如果调用者需要所有权或者需要修改内容，那么它会 clone 借用的数据。
    #[test]
    fn call_cow() {
        // 如果参数不包含空格，会 Copy出一份多余的内存
        let v1 = remove_whitespace("HelloWorld");
        let v2 = remove_ws_cow("Hello World");

        println!("{}, {}", v1, v2);
    }

    struct User<'a> {
        first_name: Cow<'a, str>,
        last_name: Cow<'a, str>,
    }

    impl<'a> User<'a> {
        pub fn new_owned(first_name: String, last_name: String) -> User<'static> {
            User {
                first_name: Cow::Owned(first_name),
                last_name: Cow::Owned(last_name),
            }
        }

        pub fn new_borrowed(first_name: &'a str, last_name: &'a str) -> Self {
            Self {
                first_name: Cow::Borrowed(first_name),
                last_name: Cow::Borrowed(last_name),
            }
        }

        pub fn first_name(&self) -> &str {
            &self.first_name
        }
        pub fn last_name(&self) -> &str {
            &self.last_name
        }
    }

    // The true power of Cow comes with to_mut method. If the Cow is owned,
    // it simply returns the pointer to the underlying data,
    // however if it is borrowed, the data is first cloned to the owned from.
    #[test]
    fn main_cow() {
        // Static lifetime as it owns the data
        let user: User<'static> = User::new_owned("James".to_owned(), "Bond".to_owned());
        println!("Name: {} {}", user.first_name, user.last_name);

        // Static lifetime as it borrows 'static data
        let user: User<'static> = User::new_borrowed("Felix", "Leiter");
        println!("Name: {} {}", user.first_name, user.last_name);

        let first_name = "Eve".to_owned();
        let last_name = "Moneypenny".to_owned();

        // Non-static lifetime as it borrows the data
        let user = User::new_borrowed(&first_name, &last_name);
        println!("Name: {} {}", user.first_name, user.last_name);
    }

    struct LazyBuffer<'a> {
        data: Cow<'a, [u8]>,
    }

    impl<'a> LazyBuffer<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            Self {
                data: Cow::Borrowed(data),
            }
        }

        pub fn data(&self) -> &[u8] {
            &self.data
        }

        pub fn append(&mut self, data: &[u8]) {
            self.data.to_mut().extend(data)
        }
    }

    #[test]
    fn main_lazy() {
        let data = vec![0u8; 10];

        // No memory copied yet
        let mut buffer = LazyBuffer::new(&data);
        println!("{:?}", buffer.data());

        // The data is cloned
        buffer.append(&[1, 2, 3]);
        println!("{:?}", buffer.data());

        // The data is not cloned on further attempts
        let new_data = vec![4, 5, 6];
        buffer.append(&new_data);
        println!("{:?}", buffer.data());
    }

    #[derive(Debug)]
    struct MyString {
        data: String,
    }

    #[derive(Debug)]
    #[repr(transparent)]
    struct MyStr {
        data: str,
    }

    impl Borrow<MyStr> for MyString {
        fn borrow(&self) -> &MyStr {
            unsafe { &*(self.data.as_str() as *const str as *const MyStr) }
        }
    }

    impl ToOwned for MyStr {
        type Owned = MyString;

        fn to_owned(&self) -> MyString {
            MyString {
                data: self.data.to_owned(),
            }
        }
    }

    impl Deref for MyString {
        type Target = MyStr;

        fn deref(&self) -> &Self::Target {
            self.borrow()
        }
    }

    #[test]
    fn main_owned_obj() {
        let data = MyString {
            data: "Hello world".to_owned(),
        };

        let borrowed_cow: Cow<'_, MyStr> = Cow::Borrowed(&data);
        println!("{:?}", borrowed_cow);

        let owned_cow: Cow<'_, MyStr> = Cow::Owned(data);
        println!("{:?}", owned_cow);
    }

    #[derive(Debug, Deserialize)]
    struct UserObj<'a> {
        #[serde(borrow)]
        name: Cow<'a, str>,
        age: u8,
    }

    #[test]
    fn main_serde() {
        let input = r#"{ "name": "Tyr", "age": 18 }"#;
        let user: UserObj = serde_json::from_str(input).unwrap();
        match user.name {
            Cow::Borrowed(x) => println!("borrowed {}", x),
            Cow::Owned(x) => println!("owned {}", x),
        }
    }

    #[test]
    fn main_mutex_guard() {
        let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
            Arc::new(Mutex::new(HashMap::new()));
        for _ in 0..32 {
            let m = metrics.clone();
            thread::spawn(move || {
                let mut g = m.lock().unwrap();
                // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
                let data = &mut *g;
                // Cow 实现了很多数据结构的 From trait，
                // 所以我们可以用 "hello".into() 生成 Cow
                let entry = data.entry("hello".into()).or_insert(0);
                *entry += 1;
                // MutexGuard 被 Drop，锁被释放
            });
        }

        thread::sleep(Duration::from_secs(1));

        println!("metrics: {:?}", metrics.lock().unwrap());
    }

    fn print_slice<T: std::fmt::Debug>(s: &[T]) {
        println!("{:?}", s);
    }

    fn print_slice_ref<T, U>(s: T)
    where
        T: AsRef<U>,
        U: std::fmt::Debug + ?Sized,
    {
        println!("{:?}", s.as_ref());
    }

    #[test]
    fn main_vec_iter() {
        let r = vec![1, 2, 3, 4]
            .iter()
            .map(|v| v * v)
            .filter(|v| *v < 16)
            .take(1)
            // 直到运行到 collect 时才真正开始执行，之前的部分不过是在不断地生成新的结构
            .collect::<Vec<i32>>();
        println!("{:?}", r);
    }

    use itertools::Itertools;

    #[test]
    fn main_itertools() {
        let err_str = "bad result";
        let input = vec![Ok(21), Err(err_str), Ok(8)];

        let it = input
            .into_iter()
            .filter_map_ok(|i| if i > 10 { Some(i * 2) } else { None });

        // 结果应该是：vec![Ok(42), Err(err_str)]
        println!("{:?}", it.collect::<Vec<_>>());
    }

    use std::iter::FromIterator;

    #[test]
    fn main_iter_str() {
        let arr = ['h', 'e', 'l', 'l', 'o'];
        let vec = vec!['h', 'e', 'l', 'l', 'o'];
        let s = String::from("hello");

        let s1 = &arr[1..3];
        let s2 = &vec[1..3];
        let s3 = &s[1..3];

        println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
        // &[char] 和 &[char] 是否相等取决于长度和内容是否相等
        assert_eq!(s1, s2);
        // &[char] 和 &str 不能直接对比，我们把 s3 变成 Vec<char>
        assert_eq!(s2, s3.chars().collect::<Vec<_>>());
        // &[char] 可以通过迭代器转换成 String，String 和 &str 可以直接对比
        assert_eq!(String::from_iter(s2), s3);
    }

    #[test]
    fn test_slices() {
        let arr = [1, 2, 3, 4, 5];
        let vec = vec![1, 2, 3, 4, 5];

        let s1 = &arr[..2];
        let s2 = &vec[..2];

        // length & content
        assert_eq!(s1, s2);
    }

    use once_cell::sync::Lazy;

    static METRICS: Lazy<Mutex<HashMap<Cow<'static, str>, usize>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    #[test]
    fn test_arc() {
        let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
            Arc::new(Mutex::new(HashMap::new()));
        for _ in 0..32 {
            let m = metrics.clone();
            thread::spawn(move || {
                let mut g = m.lock().unwrap();
                // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
                let data = &mut *g;
                // Cow 实现了很多数据结构的 From trait，
                // 所以我们可以用 "hello".into() 生成 Cow
                let entry = data.entry("hello".into()).or_insert(0);
                *entry += 1;
                // MutexGuard 被 Drop，锁被释放
            });
        }

        thread::sleep(Duration::from_secs(1));

        println!("metrics: {:?}", metrics.lock().unwrap());
    }

    pub struct WindowCount<I> {
        pub(crate) iter: I,
        count: u32,
    }

    pub trait IteratorExt: Iterator {
        fn window_count(self, count: u32) -> WindowCount<Self>
        where
            Self: Sized,
        {
            WindowCount { iter: self, count }
        }
    }

    impl<T: ?Sized> IteratorExt for T where T: Iterator {}

    impl<I> Iterator for WindowCount<I>
    where
        I: Iterator,
    {
        type Item = Vec<I::Item>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut v = Vec::new();
            for _ in 0..self.count {
                match self.iter.next() {
                    Some(item) => v.push(item),
                    None => break,
                }
            }
            if v.is_empty() { None } else { Some(v) }
        }
    }
}

use std::{error::Error, process::Command};

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        if output.status.success() {
            Ok(None)
        } else {
            Ok(Some(output.status.code().unwrap_or(1)))
        }
    }
}

pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[test]
fn test_trait_object() {
    let cmd = Shell::new("ls", &["-l"]);
    let r = execute_generics(&cmd);
    println!("{:?}", r);
    let r = execute_trait_object(&cmd);
    println!("{:?}", r);
    let r = execute_boxed_trait_object(Box::new(cmd));
    println!("{:?}", r);
}
