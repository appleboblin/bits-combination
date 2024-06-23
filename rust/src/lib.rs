pub fn generate_bits(bit_length: usize) -> Vec<String> {
    // Return empty if bit length is 0
    if bit_length == 0 {
        return vec![String::from("")];
    }

    // Recursive case to generate combo for n-1 bits
    let new_combinations = generate_bits(bit_length - 1);
    let mut combinations = Vec::new();

    // Add 0 and 1 to each combination
    for bits in new_combinations {
        combinations.push(format!("0{}", bits));
        combinations.push(format!("1{}", bits));
    }

    combinations
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // 0 bit
    #[test]
    fn generate_0_bits() {
        assert_eq!(generate_bits(0), vec![
            String::from("")
        ]);
    }

    // 1 bit
    #[test]
    fn generate_1_bits() {
        assert_eq!(generate_bits(1), vec![
            String::from("0"), String::from("1")
        ]);
    }

    // 2 bits
    #[test]
    fn generate_2_bits() {
        assert_eq!(generate_bits(2), vec![
            String::from("00"), String::from("10"),
            String::from("01"), String::from("11")
        ]);
    }

    // 3 bits
    #[test]
    fn generate_3_bits() {
        assert_eq!(generate_bits(3), vec![
            String::from("000"), String::from("100"),
            String::from("010"), String::from("110"),
            String::from("001"), String::from("101"),
            String::from("011"), String::from("111")
        ]);
    }

    // test 20 bits generate time
    #[test]
    fn generate_20_bits() {
        generate_bits(22);
    }
}