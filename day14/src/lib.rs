use regex::Regex;

struct Guard {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32
}

fn parse_input(input: &str) -> Vec<Guard> {
    let mut guards = Vec::new();
    let re = Regex::new(r"p=(-?\d+),(-?\d+)\s+v=(-?\d+),(-?\d+)").unwrap();

    for caps in re.captures_iter(input) {
        let px = caps[1].parse::<i32>().unwrap();
        let py = caps[2].parse::<i32>().unwrap();
        let vx = caps[3].parse::<i32>().unwrap();
        let vy = caps[4].parse::<i32>().unwrap();

        guards.push(Guard { px, py, vx, vy });
    }

    guards
}

fn solve1(guards: &Vec<Guard>, time: i32) -> u32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mid_x = 100/2;
    let mid_y = 102/2;

    for guard in guards {
        let nx = (guard.px + guard.vx * time).rem_euclid(101);
        let ny = (guard.py + guard.vy * time).rem_euclid(103);

        if nx < mid_x && ny < mid_y {
            q1 += 1;
        } else if nx < mid_x && ny > mid_y {
            q3 += 1;
        } else if nx > mid_x && ny < mid_y {
            q2 += 1;
        } else if nx > mid_x && ny > mid_y {
            q4 += 1;
        }
    }

    q1*q2*q3*q4
}

fn print_easter_egg(guards: &Vec<Guard>, t: i32) {
    let mut grid:Vec<Vec<char>> = vec![vec!['⬛'; 101]; 103];
    for guard in guards {
        let nx = (guard.px + guard.vx * t).rem_euclid(101);
        let ny = (guard.py + guard.vy * t).rem_euclid(103);

        grid[ny as usize][nx as usize] = '⭐';
    }

    for line in grid {
        println!("{}", line.iter().collect::<String>());
    }
}

pub mod solution {
    pub fn part1(input: &str) -> u32 {
        let guards = super::parse_input(input.trim());
        super::solve1(&guards, 100)
    }


    pub fn part2(input: &str) -> i32 {
        let guards = super::parse_input(input.trim());
        let mut min_t = u32::MAX;
        let mut best_it = 0;
        for t in 0..(101*103) {
            let count = super::solve1(&guards, t);

            if count < min_t {
                min_t = count;
                best_it = t;
            }
        }

        super::print_easter_egg(&guards, best_it);
        best_it
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 0);
    }

    #[test]
    fn test_part2() {
        let sol2 = solution::part2(TEST);
        assert_eq!(sol2, 0);
    }

}
