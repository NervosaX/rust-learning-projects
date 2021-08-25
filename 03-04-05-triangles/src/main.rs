fn main() {
    let mut buffer;
    let num_of_triangles: usize;

    loop {
        println!("How many rows in your triangle?");

        buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Expected a value");

        match buffer.trim().parse::<usize>() {
            Ok(num) => {
                num_of_triangles = num;
                break;
            }
            Err(_) => {
                println!("Error, not a number.");
                continue;
            }
        }
    }

    // * Remove the filter to get rid of the odd/even part (question 5)
    // * Remove the reverse to make it question 3
    (1..=num_of_triangles)
        .filter(|x| x % 2 != 0)
        .rev()
        .for_each(|x| println!("{}", "*".repeat(x)));
}
