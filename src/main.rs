use rand::Rng;

fn main() {
    println!("Welcome to my guessing game!");

    let secret: u32 = rand::thread_rng().gen_rng(1..=100);
    println!("{}", secret);



}

