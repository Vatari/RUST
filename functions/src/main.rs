fn main() {
    apply_to_jobs(35, "Rust Developer jobs");
    println!("{}", is_even(9));
    println!("{:?}", alphabets("aarasdsadasddvark"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs.");
}

fn is_even(number: i32) -> bool {
    number % 2  == 0
}

fn alphabets(text: &str) -> (bool, bool) {
   (text.contains("a"), text.contains("z"))
}