use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let number = &args[1].trim().parse::<u32>().expect("specify number.");

    let mut result = 0;
    let mut before_result = 1;
    let mut second_before_result = 1;

    for n in 0..*number {
        result = if n <= 1 {
            1
        } else {
            before_result + second_before_result
        };
        second_before_result = before_result;
        before_result = result;
    }

    println!("{}", result);
}
