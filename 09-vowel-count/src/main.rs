fn main() {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];

    let mut str_to_test = String::new();

    println!("Input a string:");

    std::io::stdin()
        .read_line(&mut str_to_test)
        .expect("Expected a string");

    let alpha_str: String = str_to_test
        .chars()
        .take_while(|x| x.is_alphabetic())
        .collect();

    let total_count = alpha_str.chars().count();
    let vowel_count = alpha_str
        .chars()
        .filter(|x| vowels.contains(&x.to_lowercase().to_string().parse::<char>().unwrap()))
        .count();
    let consonant_count = total_count - vowel_count;

    println!("Total vowel characters: {}", vowel_count);
    println!("Total consonant characters: {}", consonant_count);
}
