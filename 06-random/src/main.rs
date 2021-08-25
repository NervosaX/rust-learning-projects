extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn cat() {
    println!("I'm a cat");
}

fn dog() {
    println!("I'm a dog");
}

fn orange() {
    println!("I'm an orange");
}

fn main() {
    let rng = thread_rng().gen_range(1..4);

    match rng {
        1 => cat(),
        2 => dog(),
        3 => orange(),
        _ => panic!("This shouldn't be possible!"),
    }
}
