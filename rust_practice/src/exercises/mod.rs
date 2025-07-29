pub mod closures;
pub mod collection;
pub mod common;
pub mod concurrent;
mod data_structure;
mod err;
mod futures;
pub mod io;
pub mod lifecycle;
mod not_safe;
mod ownership_;
mod para;
pub mod pkg;
pub mod req;
mod sede;
pub mod string;

mod thread_safe;

mod server {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::{Arc, Mutex};

    #[allow(unused)]
    fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = match stream.read(&mut buffer) {
                Ok(n) => n,
                Err(_) => {
                    println!("Error reading from stream");
                    break;
                }
            };
            if bytes_read == 0 {
                println!("Client disconnected, failed to read data");
                break;
            }
            let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("received msg: {}", msg);

            let mut clients = match clients.lock() {
                Ok(guard) => guard,
                Err(_) => {
                    println!("Error acquiring lock on clients");
                    break;
                }
            };

            for client in clients.iter_mut() {
                if client.write(&buffer[..bytes_read]).is_err() {
                    println!("Error writing to stream");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_collect() {
        let _a: Vec<_> = (b'A'..b'z' + 1)
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect();
        println!("{:?}", _a);
    }

    #[test]
    fn test_match() {
        // match guard
        enum Temp {
            C(i32),
            F(i32),
        }
        let t = Temp::C(35);
        let _t = Temp::F(34);
        match t {
            Temp::C(tt) if tt > 30 => println!("{}", tt),
            Temp::C(tt) => println!("{}", tt),
            Temp::F(ff) if ff > 86 => println!("{}", ff),
            Temp::F(ff) => println!("{}", ff),
        }

        fn age() -> u32 {
            15
        }
        match age() {
            0 => println!("oops"),
            // catch arm as variable
            n @ 1..=12 => println!("{}", n),
            n @ 13..=19 => println!("{}", n),
            n => println!("{}", n),
        }

        fn some_number() -> Option<i32> {
            Some(1)
        }

        match some_number() {
            Some(n @ 1) => println!("{}", n),
            Some(n) => println!("{},,", n),
            _ => (),
        }
    }

    extern crate rand;

    #[test]
    fn gen_rand() {
        use rand::Rng;

        let _ = rand::random::<i32>();
        let _: i32 = rand::random();

        let mut rng = rand::rng();
        let _ = rng.random_range(0..=10);
        println!("{:?}", rng.random_range('a'..='z'));
    }

    extern crate regex;

    #[test]
    fn regex() {
        use regex::Regex;
        let date_regex = Regex::new(r"^\d{2}.\d{2}.\d{4}").expect("fail to create regex");
        let date = "15.10.2023";

        // match
        println!("is '{}' a date? {}", date, date_regex.is_match(date));

        let text_with_date = "Alan turing was born on 23.06.1912 and died on 07.06.1954. \
                                    A movie about his life called 'the limitation game' came out on 14.11.2017";
        // capture
        for cap in date_regex.captures_iter(text_with_date) {
            println!("{}", &cap[0]);
        }

        // replace
        let _txt = date_regex.replace_all(text_with_date, "$1-S2-#3");

        use regex::RegexBuilder;
        let rust_regex = RegexBuilder::new(r"rust")
            .case_insensitive(true)
            .build()
            .expect("fail to create regex");

        println!("is match Rust and rust? {}", rust_regex.is_match("RuST"));
    }

    use std::env;

    #[test]
    fn cli() {
        if let Some(arg) = env::args().nth(1) {
            println!("the first param is {}", arg);
        }

        let args: Vec<_> = env::args().collect();
        println!("{:?}", args);
    }

    #[test]
    fn env() {
        // Returns an iterator of (variable, value) pairs of strings, for all the environment variables of the current process.
        for (key, val) in env::vars() {
            println!("{}: {}", key, val);
        }

        let key = "PORT";
        env::set_var(key, "8080");
        print_env_var(key);
        env::remove_var(key);
        print_env_var(key); // error, NotPresent

        // cwd
        let root = std::path::Path::new("/");
        env::set_current_dir(root).expect("fail to set cwd");
        println!("cwd is {:?}", env::current_dir().expect("fail to read cwd"));
    }

    fn print_env_var(key: &str) {
        match env::var(key) {
            Ok(k) => println!("{}: {}", key, k),
            Err(e) => println!("error, {:?}", e),
        }
    }

    #[test]
    fn io() {
        use std::io::{self, prelude::*};

        print_single_line("please enter your name: ");
        let name = read_line_iter();
        let _n = read_line_buffer();

        println!("your name is {}", name);

        fn print_single_line(txt: &str) {
            print!("{}", txt);
            // flush to guarantee display
            io::stdout().flush().expect("fail to flush output");
        }

        fn read_line_iter() -> String {
            io::stdin()
                .lock()
                // read one line of input iter-style
                .lines()
                .next()
                .expect("no lines in buffer")
                .expect("fail to read line")
                .trim()
                .to_owned()
        }

        fn read_line_buffer() -> String {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("fail to read line");
            input.trim().to_string()
        }
    }

    macro_rules! multiply {
        ($last:expr) => {
            $last
        };
        ($head:expr, $($tail:expr), +) => {
            $head * multiply!($($tail), +)
        }
    }

    #[test]
    fn mul() {
        let val = multiply!(2, 4, 5);
        println!("2*4*5 = {}", val);
    }
}
