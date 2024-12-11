fn split_lines(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn differences_within_range(report: &[i32]) -> bool {
    let row: Vec<i32> = report.windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();
    let is_in_range = row.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3);
    let is_increasing = row.iter().all(|&diff| diff > 0);
    let is_decreasing = row.iter().all(|&diff| diff < 0);

    is_in_range && (is_increasing || is_decreasing)
}

fn is_safe(report: &[i32]) -> bool{
    if differences_within_range(report) {
        return true;
    } else {
        for i in 0..report.len() {
            let mut modified = report.to_vec();
            modified.remove(i);

            if differences_within_range(&modified) {
                return true;
            }
        }
    }

    false
}


pub mod solution {
    pub fn part1(input: &str) -> i32 {
        let grid = super::split_lines(input.trim());

        grid.iter()
            .filter(|&row| super::differences_within_range(row))
            .count().try_into().unwrap()
    }

    pub fn part2(input: &str) -> i32 {
        let grid = super::split_lines(input.trim());

        grid.iter()
            .filter(|&row| super::is_safe(row))
            .count().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STR: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        let result = solution::part1(TEST_STR);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = solution::part2(TEST_STR);
        assert_eq!(result, 4);
    }

}
