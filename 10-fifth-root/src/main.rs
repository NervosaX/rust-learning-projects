fn main() {
    let total = (0u32..100)
        .filter(|x| x % 2 != 0)
        .map(|x| x.pow(2))
        .fold(0, |sum, i| sum + i) as f32;

    let result = total.powf(1f32 / 5f32);

    println!("Result is: {}", result);
}
