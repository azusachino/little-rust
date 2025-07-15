#![allow(unused)]
#[cfg(test)]
mod tests {
    use std::{process, rc::Rc};

    const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
    const I_AM_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
    const CONTEXT: usize = 2;

    fn sum(data: Vec<u32>) -> u32 {
        data.iter().sum()
    }

    fn sum_ref(data: &Vec<u32>) -> u32 {
        // 值的地址会改变么？引用的地址会改变么？
        println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
        data.iter().sum()
    }

    // generate a temporary file name that is hopefully unique
    #[inline]
    fn temp_file() -> String {
        let thread_id: String = format!("{:?}", std::thread::current().id())
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        format!("./temp_{}_{}", process::id(), thread_id)
    }

    #[test]
    fn run_sum() {
        let data = vec![1, 2, 3, 4];
        sum(data.clone());
        let data1 = &data;

        // 值的地址是什么？引用的地址又是什么？
        println!(
            "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
            &data, data1, &&data, &data1
        );
        println!("sum of data1: {}", sum_ref(data1));
        // 堆上数据的地址是什么？
        println!(
            "addr of items: [{:p}, {:p}, {:p}, {:p}]",
            &data[0], &data[1], &data[2], &data[3]
        );
    }

    #[test]
    fn print() {
        println!("{:?} {:?} {:?}", RUSTC_COLOR_ARGS, I_AM_DONE_REGEX, CONTEXT);
        temp_file();
        let _data = include_bytes!("lib.rs");
        let rc = Rc::new(1);
        let _ = rc.clone();
    }

    #[test]
    fn fizz_buzz() {
        for i in 1..102 {
            match (i % 3, i % 5) {
                (0, 0) => println!("FizzBuzz"),
                (0, _) => println!("Fizz"),
                (_, 0) => println!("Buzz"),
                _ => println!("{}", i),
            }
        }
    }

    fn counter(i: i32) -> impl FnMut(i32) -> i32 {
        move |n| n + i
    }

    #[test]
    fn test_counter() {
        let mut f = counter(2);
        assert_eq!(3, f(1));
    }

    type RGB = (i16, i16, i16);

    fn color(_: &str) -> RGB {
        (1, 1, 1)
    }

    fn show(c: fn(&str) -> RGB) {
        println!("{:?}", c("black"))
    }

    #[test]
    fn test_color() {
        let rgb = color;
        println!("size of {:?}", std::mem::size_of_val(&rgb));
        // 函数项隐式转换成函数指针
        show(rgb);

        let c = |_: &str| (1, 2, 3);
        println!("size of {:?}", std::mem::size_of_val(&c));
        show(c);
    }

    fn foo<F: Fn() + Copy>(f: F) {
        f()
    }

    #[test]
    fn test_closure() {
        let c1 = || "c1";
        let c2 = || "c2";
        let v = [c1, c2];

        let i = 1;
        let c3 = || i;

        let mut j = 1i32;
        let c4 = || {
            j = 3;
            j
        };

        let s = "hello".to_owned();
        let f = || {
            println!("{}", s);
        };

        foo(f);
    }
}

fn main() {
    let tao = '道';
    let _1 = tao as u32;
    assert_eq!(36947, _1);

    println!("U+{:x}", _1);
    println!("{}", tao.escape_unicode());

    assert_eq!(char::from(65), 'A');
    assert_eq!(std::char::from_u32(0x9053), Some('道'));
    assert_eq!(std::char::from_u32(36947), Some('道'));
    // not valid unicode
    // assert_eq!(std::char::from_u32(12909090909), None);
}

fn main2() {
    let v = vec![1, 2, 3, 4, 5];
    {
        // for scope
        let mut _itr = v.into_iter();
        loop {
            match _itr.next() {
                Some(i) => {
                    println!("{}", i)
                }
                None => break,
            }
        }
    }
}

pub mod outer_mod {
    pub(self) fn outer_mod_fn() {}

    pub mod inner_mod {
        pub(in super::super::outer_mod) fn aaa() {}

        pub(crate) fn crate_visible_fn() {}
        pub(super) fn super_mod_visible_fn() {
            super::outer_mod_fn();
            inner_mod_visible_fn();
        }

        pub(self) fn inner_mod_visible_fn() {}
    }
}
