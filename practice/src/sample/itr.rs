#[cfg(test)]
mod test {

    use std::iter::Iterator;

    struct Seq {
        current: i32,
    }

    impl Seq {
        fn new() -> Self {
            Self { current: 0 }
        }
    }

    impl Iterator for Seq {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            if self.current < 100 {
                self.current += 1;
                Some(self.current)
            } else {
                None
            }
        }
    }

    #[test]
    fn main() {
        let mut seq = Seq::new();
        while let Some(i) = seq.next() {
            println!("{}", i);
        }
    }

    #[test]
    fn m() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut iter = v
            .iter()
            .take(5)
            .filter(|&x| x % 2 == 0)
            .map(|&x| x * x)
            .enumerate();
        while let Some((i, v)) = iter.next() {
            println!("{}, {}", i, v);
        }
    }
}
