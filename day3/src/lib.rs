use regex::Regex;

fn calculate_mul_sum(corrupted: &str) -> i32 {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    mul_pattern
        .captures_iter(corrupted)
        .filter_map(|captures| {
            let x = captures.get(1).and_then(|m| m.as_str().parse::<i32>().ok());
            let y = captures.get(2).and_then(|m| m.as_str().parse::<i32>().ok());
            match (x, y) {
                (Some(x), Some(y)) => Some(x * y), // Compute the product if both numbers are valid
                _ => None, // Skip invalid entries
            }
        })
        .sum()
}

fn calculate_enabled_mul_sum(corrupted_string: &str) -> i32 {
    let token_pattern = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let tokens: Vec<_> = token_pattern
        .find_iter(corrupted_string)
        .map(|m| m.as_str())
        .collect();

    tokens
        .iter()
        .scan(true, |enabled, &token| {
            if token == "do()" {
                *enabled = true;
                Some(0) 
            } else if token == "don't()" {
                *enabled = false;
                Some(0)
            } else if let Some(captures) = mul_pattern.captures(token) {
                if *enabled {
                    let x = captures.get(1).and_then(|m| m.as_str().parse::<i32>().ok());
                    let y = captures.get(2).and_then(|m| m.as_str().parse::<i32>().ok());

                    match (x, y) {
                        (Some(x), Some(y)) => Some(x * y), // Compute the product if both numbers are valid
                        _ => None, // Skip invalid entries
                    }

                } else {
                    Some(0) 
                }
            } else {
                Some(0) 
            }
        })
        .sum() 
}

pub mod solution {
    pub fn part1(input: &str) -> i32 {
        super::calculate_mul_sum(input)
    }

    pub fn part2(input: &str) -> i32 {
        super::calculate_enabled_mul_sum(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mul_sum() {
        let corrupted_memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_sum = 161; // 2*4 + 5*5 + 32*64 + 11*8 + 8*5
        assert_eq!(calculate_mul_sum(corrupted_memory), expected_sum);
    }

    #[test]
    fn test_empty_string() {
        let corrupted_memory = "";
        let expected_sum = 0;
        assert_eq!(calculate_mul_sum(corrupted_memory), expected_sum);
    }

    #[test]
    fn test_no_mul() {
        let corrupted_memory = "do()don't()randomtext";
        let expected_sum = 0;
        assert_eq!(calculate_mul_sum(corrupted_memory), expected_sum);
    }

    #[test]
    fn test_only_mul() {
        let corrupted_memory = "mul(2,3)mul(4,5)";
        let expected_sum = 2 * 3 + 4 * 5; // 6 + 20 = 26
        assert_eq!(calculate_mul_sum(corrupted_memory), expected_sum);
    }
}
