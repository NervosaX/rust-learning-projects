struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current + self.next;
        self.current = self.next;
        self.next = next;
        Some(self.current)
    }
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

fn main() {
    let fib_num: usize;

    loop {
        println!("Pick a number!");

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Expected a result");

        match buffer.trim().parse::<usize>() {
            Ok(num) => {
                fib_num = num;
                break;
            }
            Err(_) => {
                println!("\nNot a number!\n");
                continue;
            }
        }
    }

    let fib = Fibonacci::new();

    let result = fib
        .take(fib_num)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", result);
}
