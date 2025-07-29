#![allow(unused)]
struct ZeroSized {}

#[cfg(test)]
mod test {

    use super::ZeroSized;

    #[test]
    fn main() {
        let mut v1 = Vec::<i32>::new();
        println!("Start: length {} capacity {}", v1.len(), v1.capacity());

        for i in 1..10 {
            v1.push(i);
            println!(
                "[Pushed {}] length {} capacity {}",
                i,
                v1.len(),
                v1.capacity()
            );
        }

        let mut v2 = Vec::<i32>::with_capacity(1);
        println!("Start: length {} capacity {}", v2.len(), v2.capacity());

        v2.reserve(10);
        for i in 1..10 {
            v2.push(i);
            println!(
                "[Pushed {}] length {} capacity {}",
                i,
                v2.len(),
                v2.capacity()
            );
        }
    }

    // 如果指定元素的类型大小为0，那么Vec无需在堆上分配任何空间
    #[test]
    fn zero_sized() {
        let mut v = Vec::<ZeroSized>::new();
        println!("capacity: {}, length: {}", v.capacity(), v.len());

        v.push(ZeroSized {});
        v.push(ZeroSized {});
        println!("capacity: {}, length: {}", v.capacity(), v.len());

        // p 永远指向 align_of::<ZeroSized>()，不需要调用 allocator
        let p = v.as_ptr();
        println!("ptr: {:p}", p);

        let size1 = std::mem::size_of::<Vec<i32>>();
        let size2 = std::mem::size_of::<Option<Vec<i32>>>();
        println!("size of vec: {}, size of option vec: {}", size1, size2);
    }

    fn fib_loop(n: u8) {
        let mut a = 1;
        let mut b = 1;
        let mut i = 2u8;

        loop {
            let c = a + b;
            a = b;
            b = c;
            i += 1;

            println!("next val is {}", b);

            if i >= n {
                break;
            }
        }
    }

    fn fib_while(n: u8) {
        let (mut a, mut b, mut i) = (1, 1, 2);

        while i < n {
            let c = a + b;
            a = b;
            b = c;
            i += 1;

            println!("next val is {}", b);
        }
    }

    fn fib_for(n: u8) {
        let (mut a, mut b) = (1, 1);

        for _i in 2..n {
            let c = a + b;
            a = b;
            b = c;
            println!("next val is {}", b);
        }
    }
}
