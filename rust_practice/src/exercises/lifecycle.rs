#![allow(dead_code, coherence_leak_check)]
use std::{fmt, slice};

#[derive(Clone, Copy)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释掉，看看会出什么问题
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn use_buffer(buf: RawBuffer) {
        println!("buf to die: {:?}", buf);

        // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
        // drop(buf)
    }

    #[test]
    fn main() {
        let data = vec![1, 2, 3, 4];

        let buf: RawBuffer = data.into();

        // 因为 buf 允许 Copy，所以这里 Copy 了一份
        use_buffer(buf);

        // buf 还能用
        println!("buf: {:?}", buf);
    }
}

// Use Early Bound Lifetimes: When you need the lifetime to be tied directly to the type or function definition and you want stricter, compile-time checks.
// Use Late Bound Lifetimes: When you need flexibility and want the lifetimes to be inferred or determined at the point of function call or object creation.
#[cfg(test)]
mod lifetimes {

    // early bound
    struct MyStruct<'a> {
        value: &'a str,
    }

    // Early bound lifetimes are lifetimes that are specified as part of the type or trait definition and are explicitly bound to the lifetime of the references when the type or trait is instantiated.
    impl<'a> MyStruct<'a> {
        fn get_value(&self) -> &'a str {
            self.value
        }
    }

    #[test]
    fn test_my_struct() {
        let s = String::from("welcome");
        let my_str = MyStruct { value: &s };
        println!("{}", my_str.get_value());
    }

    #[test]
    fn early_bound_lifetime() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut buf = Buffer::new(&v);
        let b1 = buf.read_bytes();
        // let b1 = &buf.read_bytes().to_owned();
        let b2 = buf.read_bytes();
        print(b1, b2);
    }

    fn print(b1: &[u8], b2: &[u8]) {
        println!("{:#?} {:#?}", b1, b2)
    }

    struct Buffer<'a> {
        buf: &'a [u8],
        pos: usize,
    }

    impl<'b, 'a: 'b> Buffer<'a> {
        fn new(b: &'a [u8]) -> Buffer<'a> {
            Buffer { buf: b, pos: 0 }
        }

        fn read_bytes(&'b mut self) -> &'a [u8] {
            self.pos += 3;
            &self.buf[self.pos - 3..self.pos]
        }
    }
}

mod late_bound {

    #[test]
    fn main() {
        let mut buf = Buffer::new();
        let _ = buf.read_bytes(); // don't work
        let b1 = &(buf.read_bytes().to_owned());
        let b2 = buf.read_bytes();
        print(b1, b2)
    }

    struct Buffer {
        buf: Vec<u8>,
        pos: usize,
    }

    impl Buffer {
        fn new() -> Buffer {
            Buffer {
                buf: vec![1, 2, 3, 4, 5, 6],
                pos: 0,
            }
        }

        fn read_bytes<'a>(&'a mut self) -> &'a [u8] {
            self.pos += 1;
            &self.buf[self.pos - 1..self.pos]
        }
    }

    fn print(b1: &[u8], b2: &[u8]) {
        println!("{:#?} {:#?}", b1, b2)
    }
}

// Higher Rank trait Bounds
// This usage of for occurs primarily with trait bounds and function pointers, and it specifies that the lifetime is generic and will be determined when the function is actually called.
mod hrtb {

    // fn main() {
    //     let f = |x: &i32| x; // error
    //                          // 假如支持下面的语法就方便多了，目前还未支持
    //                          // let f: for<'a> Fn(&'a i32) -> &'a i32 = |x| x;
    //     let i = &3;
    //     let j = f(i);
    // }
    fn annotate<T, F>(f: F) -> F
    where
        for<'a> F: Fn(&'a T) -> &'a T,
    {
        f
    }

    #[test]
    fn main() {
        let f = annotate(|x| x);
        let i = &3;
        let j = f(i);
        assert_eq!(*j, 3);
    }

    use std::fmt::Debug;
    trait DoSomething<T> {
        fn do_sth(&self, value: T);
    }
    impl<'a, T: Debug> DoSomething<T> for &'a usize {
        fn do_sth(&self, value: T) {
            println!("{:?}", value);
        }
    }
    fn bar(b: Box<dyn for<'f> DoSomething<&'f usize>>) {
        let s: usize = 10;
        b.do_sth(&s);
    }

    #[test]
    fn main2() {
        let x = Box::new(&2usize);
        bar(x);
    }

    use std::io::Read;

    trait Checksum<R: Read> {
        fn calc(&mut self, r: R) -> Vec<u8>;
    }

    struct Xor;

    impl<R: Read> Checksum<R> for Xor {
        fn calc(&mut self, mut r: R) -> Vec<u8> {
            let mut res: u8 = 0;
            let mut buf = [0u8; 8];
            loop {
                let read = r.read(&mut buf).unwrap();
                if read == 0 {
                    break;
                }
                for b in &buf[..read] {
                    res ^= b;
                }
            }

            vec![res]
        }
    }

    struct Add;

    impl<R: Read> Checksum<R> for Add {
        fn calc(&mut self, mut r: R) -> Vec<u8> {
            let mut res: u8 = 0;
            let mut buf = [0u8; 8];
            loop {
                let read = r.read(&mut buf).unwrap();
                if read == 0 {
                    break;
                }
                for b in &buf[..read] {
                    let tmp = res as u16 + *b as u16;
                    res = tmp as u8;
                }
            }

            vec![res]
        }
    }

    #[test]
    fn main3() {
        let mut buf = [0u8; 8];
        // error[E0308]: `if` and `else` have incompatible types
        // 修正：
        // step 1: Box<dyn Checksum<&[u8]>> 转为 trait 对象
        // step 2: Box<dyn for<'a> Checksum<&'a [u8]>> 使用 for<'a> 转为 late bound
        let mut checker: Box<dyn for<'a> Checksum<&'a [u8]>> = if rand::random() {
            println!("Initializing Xor Checksum");
            Box::new(Xor)
        } else {
            println!("Initializing Add Checksum");
            Box::new(Add)
        };

        let mut data = "Sedm lumpu slohlo pumpu za uplnku".as_bytes();
        let mut i = 0;

        loop {
            let chunk_size = data.read(&mut buf).unwrap();
            if chunk_size == 0 {
                break;
            }
            let cs = checker.calc(&buf[..chunk_size]);
            println!("Checksum {} is {:?}", i, cs);
            i += 1;
        }
    }
}

mod generic {
    trait Trait {
        fn f(self);
    }

    impl<T> Trait for fn(T) {
        fn f(self) {
            print!("1");
        }
    }

    impl<T> Trait for fn(&T) {
        fn f(self) {
            print!("2");
        }
    }

    #[test]
    fn main() {
        // 112
        let a: fn(_) = |_: u8| {};
        let b: fn(_) = |_: &u8| {};
        let c: fn(&_) = |_: &u8| {};
        a.f();
        b.f();
        c.f();
    }
}
