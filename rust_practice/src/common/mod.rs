#![allow(dead_code)]
mod alloca;
mod atomic_action;
// channel practice
mod ch;
mod container;
mod copy_clone;
mod enums;
mod extender;

mod inner_mutability;

mod leak;
mod network;
mod path_ruler;
mod smart_pointer;

#[cfg(test)]
mod test_reader {
    use std::fs::File;
    use std::io::{BufReader, Read, Result};

    const fn init_len() -> usize {
        5
    }

    struct MyReader<R> {
        reader: R,
        buf: String,
    }

    impl<R> MyReader<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                buf: String::with_capacity(64),
            }
        }
    }

    impl<R> MyReader<R>
    where
        R: Read,
    {
        pub fn process(&mut self) -> Result<usize> {
            self.reader.read_to_string(&mut self.buf)
        }
    }

    #[test]
    fn main() {
        let f = File::open("./mod.rs").unwrap();
        let mut reader = MyReader::new(BufReader::new(f));

        let size = reader.process().unwrap();
        let sz = init_len();
        println!("total size read: {}", size);
        println!("init size: {}", sz);
    }

    fn two_times_impl() -> impl Fn(i32) -> i32 {
        let i = 2;
        move |j| j * i
    }

    #[test]
    fn main_fn() {
        let r = two_times_impl();
        assert_eq!(r(2), 4);
    }

    #[test]
    fn main_match() {
        let n = 42;
        match n {
            0 => println!("zero"),
            // 1...3 => println!("all"),
            5 | 7 | 13 => println!("bad luck"),
            // 使用操作符@可以将模式中的值绑定给一个变量，供分支右侧的代码使用
            n @ 42 => println!("answer is {}", n),
            _ => println!("common"),
        }

        let mut v = vec![1, 2, 3, 4, 5];
        let mut vc = v.clone();
        while let Some(x) = v.pop() {
            println!("{}", x);
        }
        // while let is efficient than loop
        // loop {
        //     match v.pop() {
        //         Some(x) => println!("{}", x),
        //         None => break,
        //     }
        // }

        while let Some(x) = vc.pop() {
            println!("{}", x);
        }
    }

    #[test]
    fn main_range() {
        assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
        assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    }

    #[test]
    fn main_slice() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(&arr, &[1, 2, 3, 4, 5]);
        assert_eq!(&arr[1..], [2, 3, 4, 5]);

        assert_eq!(&arr.len(), &5);
        assert_eq!(&arr.is_empty(), &false);

        let arr = &mut [1, 2, 3];
        arr[1] = 7;

        assert_eq!(arr, &[1, 7, 3]);

        let vec = vec![1, 2, 3];
        assert_eq!(&vec[..], [1, 2, 3]);
    }

    #[test]
    fn main_str() {
        let truth: &'static str = "Rust";
        let ptr = truth.as_ptr();
        let len = truth.len();

        assert_eq!(4, len);

        let s = unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8(slice)
        };
        assert_eq!(s, Ok(truth));
    }

    #[test]
    fn main_pointer() {
        let mut x = 50;
        let ptr_x = &mut x as *mut i32;
        let y = Box::new(20);
        let ptr_y = &*y as *const i32;

        unsafe {
            *ptr_x += *ptr_y;
        }
        assert_eq!(x, 70);
    }
}

#[cfg(test)]
mod test_writer {

    use std::fs::File;
    use std::io::BufWriter;
    use std::net::TcpStream;

    #[allow(dead_code)]
    #[derive(Debug)]
    struct MyWriter<W> {
        writer: W,
    }

    impl MyWriter<BufWriter<TcpStream>> {
        pub fn new(addr: &str) -> Self {
            let stream = TcpStream::connect(addr).unwrap();
            Self {
                writer: BufWriter::new(stream),
            }
        }
    }

    impl MyWriter<File> {
        pub fn new(addr: &str) -> Self {
            let file = File::open(addr).unwrap();
            Self { writer: file }
        }
    }

    impl<W> MyWriter<W> {
        pub fn write(&self, msg: &str) {
            println!("msg: {:?}", msg)
        }
    }

    #[test]
    fn main() {
        let writer = MyWriter::<BufWriter<TcpStream>>::new("127.0.0.1:8080");
        writer.write("hello world!");

        let writer = MyWriter::<File>::new("/etc/hosts");
        writer.write("127.0.0.1 localhost");
    }
}

#[cfg(test)]
mod test_vec {

    use rand::Rng;

    #[test]
    fn main() {
        let mut data: Vec<*const [u8]> = Vec::new();
        let mut rng: rand::prelude::ThreadRng = rand::rng();

        for _i in 0..5 {
            let mut num: Vec<u8> = Vec::new();
            for _j in 0..16 {
                let rand_num: u8 = rng.random();
                num.push(rand_num);
            }
            println!("num({:p}) is : {:?}", &*num, num);
            let boxed = num.into_boxed_slice();
            data.push(Box::into_raw(boxed) as _);
        }
        println!("data is: {:?}", data);
    }
}

#[cfg(test)]
mod test_trait {
    use regex::Regex;
    use std::fmt;
    use std::io::Write;
    use std::str::FromStr;

    pub trait Parse {
        type Error;
        fn parse(s: &str) -> Result<Self, Self::Error>
        where
            Self: Sized;
    }

    impl<T> Parse for T
    where
        T: FromStr + Default,
    {
        type Error = String;
        fn parse(s: &str) -> Result<Self, Self::Error> {
            let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
            if let Some(captures) = re.captures(s) {
                // 当出错时我们返回 Err(String)
                captures
                    .get(0)
                    .map_or(Err("failed to capture".to_string()), |s| {
                        s.as_str()
                            .parse()
                            .map_err(|_err| "failed to parse captured string".to_string())
                    })
            } else {
                Err("failed to parse string".to_string())
            }
        }
    }

    struct BufBuilder {
        buf: Vec<u8>,
    }

    impl BufBuilder {
        pub fn new() -> Self {
            Self {
                buf: Vec::with_capacity(1024),
            }
        }
    }

    impl fmt::Debug for BufBuilder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", String::from_utf8_lossy(&self.buf))
        }
    }

    impl Write for BufBuilder {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn main() {
        let mut buf = BufBuilder::new();
        buf.write_all(b"HelloWorld").unwrap();
        println!("{:?}", buf);
    }

    #[test]
    fn parse_should_work() {
        assert_eq!(u32::parse("123abcd"), Ok(123));
        assert_eq!(
            u32::parse("123.45abcd"),
            Err("failed to parse captured string".into())
        );
        assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
        assert!(f64::parse("abcd").is_err());
    }
}

#[cfg(test)]
mod test_traitor {

    struct SentenceIter<'a> {
        s: &'a mut &'a str,
        delimiter: char,
    }
    impl<'a> SentenceIter<'a> {
        pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
            Self { s, delimiter }
        }
    }
    impl<'a> Iterator for SentenceIter<'a> {
        type Item = &'a str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.s.is_empty() {
                return None;
            }
            match self.s.find(self.delimiter) {
                Some(pos) => {
                    let len = self.delimiter.len_utf8();
                    let s = &self.s[..pos + len];
                    let suffix = &self.s[pos + len..];
                    *self.s = suffix;
                    Some(s.trim())
                }
                None => {
                    let s = (*self.s).trim();
                    *self.s = "";
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                }
            }
        }
    }

    #[test]
    fn it_works() {
        let mut s = "This is the 1st sentence. This is the 2nd sentence.";
        let mut iter = SentenceIter::new(&mut s, '.');
        assert_eq!(iter.next(), Some("This is the 1st sentence."));
        assert_eq!(iter.next(), Some("This is the 2nd sentence."));
        assert_eq!(iter.next(), None);
    }

    pub trait HList: Sized {}

    pub struct HNil;

    impl HList for HNil {}

    pub struct HCons<H, T> {
        pub head: H,
        pub tail: T,
    }

    impl<H, T: HList> HList for HCons<H, T> {}
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;
    use std::mem::size_of_val;

    #[test]
    fn main_read_file() {
        read_file("./mod.rs").unwrap();
    }

    fn read_file(name: &str) -> Result<String, std::io::Error> {
        let mut f = File::open(name)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[test]
    fn main_closure() {
        let c1 = || println!("hello world!");
        let c2 = |i: i32| println!("hello {}", i);

        let name = String::from("az");
        let name_cln = name.clone();

        let mut table = HashMap::new();
        table.insert("hello", "world");

        // 捕获一个引用，长度为 8
        let c3 = || println!("hello: {}", name);

        let c4 = move || println!("hello: {}, {:?}", name_cln, table);

        let name_cln2 = name.clone();

        let c5 = move || {
            let x = 1;
            let name3 = String::from("lindy");
            println!("hello {}, {:?}, {:?}", x, name_cln2, name3);
        };

        println!(
            "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
            size_of_val(&c1),
            size_of_val(&c2),
            size_of_val(&c3),
            size_of_val(&c4),
            size_of_val(&c5),
            size_of_val(&main_closure),
        )
    }
}
