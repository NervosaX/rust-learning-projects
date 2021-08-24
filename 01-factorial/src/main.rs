fn get_number_from_buffer() -> u32 {
    let result: u32;

    loop {
        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Expected a value");

        match buffer.trim().parse::<u32>() {
            Ok(num) => {
                result = num;
                break;
            }
            Err(_) => {
                println!("Error, not a number");
                continue;
            }
        }
    }

    result
}

fn main() {
    println!("Get the factorial of which number?");

    let num = get_number_from_buffer();
    let factorial = (1..=num).fold(1, |res, n| res * n);

    println!("Factorial: {}", factorial);
}
