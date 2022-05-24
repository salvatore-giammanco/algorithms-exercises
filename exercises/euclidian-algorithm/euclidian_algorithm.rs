use std::io;

fn insert_number(message: &String) -> u32{
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    
    let input: u32 = input.trim().parse().expect("Please insert an unsigned 32bit number!");
    input    
}

fn calculate_module(first_number: u32, second_number:u32) -> u32 {
    if first_number > second_number {
        return first_number % second_number
    }
    second_number % first_number
}

fn main() {
    println!("Great Common Divisor (GCD) Calculator!\n");
    let mut first_number: u32 = insert_number(&String::from("Insert first number: "));
    let mut second_number: u32 = insert_number(&String::from("Insert second number: "));

    let mut module = calculate_module(first_number, second_number);

    while module != 0 {
        first_number = second_number;
        second_number = module;
        module = calculate_module(first_number, second_number);
    }

    println!("GCD: {}", second_number);
}
