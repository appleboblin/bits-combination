use std::io;
use bits::generate_bits;

fn main() {
    // Prompt for user input
    println!("Number of bits: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let bit_length: usize = input.trim().parse().expect("Please enter a valid number");

    // generate all combinatinos for n bit
    let combinations = generate_bits(bit_length);

    // print number of combinations
    println!("Number of {}-bit combinations: {}", bit_length, combinations.len());

    // Print all combinations
    for bit in combinations {
        println!("{}", bit);
    }
}
