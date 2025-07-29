#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::io;
    use std::process;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn main() {
        guess();
        thread::sleep(Duration::from_millis(1000));
        process::exit(0);
    }

    fn guess() {
        use rand::Rng;
        println!("Guess the number!");

        let secret_number = rand::rng().random();

        loop {
            println!("Please input your guess.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
