

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate
}

struct Expression {
    a: u64,
    b: u64,
    op: Operation
}

impl Expression {
    fn evaluate(&self) -> u64 {
        match self.op {
            Operation::Add => self.a + self.b,
            Operation::Multiply => self.a * self.b,
            Operation::Concatenate => {
                let concat = format!("{}{}", self.a, self.b);
                concat.parse::<u64>().unwrap_or(0)
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)>{
    let mut res: Vec<(u64, Vec<u64>)> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(":");
        let key_part = parts.next();
        let values_part = parts.next();

        if let (Some(key), Some(values)) = (key_part, values_part) {
            if let Ok(key_num) = key.trim().parse::<u64>() {
                let values_vec: Vec<u64> = values
                    .split_whitespace()
                    .filter_map(|v| v.parse::<u64>().ok())
                    .collect();

                res.push((key_num, values_vec));
            }
        }
    }

    res
}

fn check_combinations(res: u64, values: &[u64]) -> bool {
    let n = values.len();
    let num_operators = n - 1;
    let total_combinations = 1 << num_operators;

    for i in 0..total_combinations {
        let mut result = values[0];
        let mut current = values[0];

        for j in 0..num_operators {
            let op = if (i& (1 << j)) != 0 {
                Operation::Add
            } else {
                Operation::Multiply
            };
            let exp = Expression {
                a: current,
                b: values[j+1],
                op: op
            };

            current = exp.evaluate();
            result = current
        }

        if result == res {
            return true;
        }
    }

    false
}


fn check_combinations_3ops(res: u64, values: &[u64]) -> bool {
    fn recurse(current: u64, index: usize, res: u64, values: &[u64]) -> bool {
        if index == values.len() {
            return current == res;
        } else {

            let next_value = values[index];
            let exprs = [
                Expression{a: current, b: next_value, op: Operation::Add},
                Expression{a: current, b: next_value, op: Operation::Multiply},
                Expression{a: current, b: next_value, op: Operation::Concatenate}
            ];

            for exp in exprs {
                let new_res = exp.evaluate();
                if recurse(new_res, index+1, res, values) {
                    return true;
                }
            }


            return false;
        }

    }
    recurse(0, 0, res, values)
}


pub mod solution {
    pub fn part1(input: &str) -> u64 {
        let vec = super::parse_input(input.trim());

        let mut sum = 0;
        for row in vec {
            if super::check_combinations(row.0, &row.1) {
                sum += row.0;
            }
        }

        sum
    }


    pub fn part2(input: &str) -> u64 {
        let vec = super::parse_input(input.trim());

        let mut sum = 0;
        for row in vec {
            if super::check_combinations_3ops(row.0, &row.1) {
                sum += row.0;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    const TEST: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part1() {
        let sol = solution::part1(TEST);
        assert_eq!(sol, 3749)
    }

    #[test]
    fn test_part2() {
        let sol = solution::part2(TEST);
        assert_eq!(sol, 11387)
    }

    #[test]
    fn test_part3() {
        let exp = Expression{a:173138715990, b:3123213, op:Operation::Concatenate};
        let res = exp.evaluate();

        assert_eq!(res, 1731387159903123213);
    }
}
