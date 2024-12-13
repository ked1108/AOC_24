use regex::Regex;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(r"\d+").unwrap();

    let mut parsed = Vec::new();

    for line in input.split("\n\n") {
        let nums: Vec<i64> = re.find_iter(line)
            .filter_map(|mat| mat.as_str().parse::<i64>().ok())
            .collect();

        if nums.len() == 6 {
            parsed.push(nums);
        }
    }

    parsed
}

fn solve(machines: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for machine in machines {
        let (ax, ay, bx, by, px, py)  = (
            machine[0],
            machine[1],
            machine[2],
            machine[3],
            machine[4],
            machine[5],
        );

        let ca = (px * by - py * bx) / (ax*by - ay*bx);
        let ca_check = (px * by - py * bx) % (ax*by - ay*bx);

        let cb = (px - ax * ca) / bx;
        let cb_check = (px - ax * ca) % bx;

        println!("{ca} {cb}");

        if ca_check == 0 && cb_check == 0 {
            total += ca*3 + cb;
        }

    }

    total
}

fn solve_2(machines: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for machine in machines {
        let (ax, ay, bx, by, px, py)  = (
            machine[0],
            machine[1],
            machine[2],
            machine[3],
            machine[4] + 10000000000000,
            machine[5] + 10000000000000,
        );

        let ca = (px * by - py * bx) / (ax*by - ay*bx);
        let ca_check = (px * by - py * bx) % (ax*by - ay*bx);

        let cb = (px - ax * ca) / bx;
        let cb_check = (px - ax * ca) % bx;

        println!("{ca} {cb}");

        if ca_check == 0 && cb_check == 0 {
            total += ca*3 + cb;
        }

    }

    total
}

pub mod solution {
    pub fn part1(input: &str) -> i64 {
        let machines = super::parse_input(input.trim());
        super::solve(machines)
    }

    pub fn part2(input: &str) -> i64 {
        let machines = super::parse_input(input.trim());
        super::solve_2(machines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;


    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 480);
    }

    #[test]
    fn test_part2() {
        let sol2 = solution::part2(TEST);
        assert_eq!(sol2, 0);
    }

}
