use std::collections::{VecDeque, HashSet};

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<(usize, usize)>) {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut trail_heads: Vec<(usize, usize)> = Vec::new();

    for (r, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        } else {
            let mut row: Vec<i32> = Vec::new();
            for (c, char) in line.chars().enumerate() {
                row.push(char.to_digit(10).unwrap_or(0) as i32);
                if char.to_digit(10) == Some(0) {
                    trail_heads.push((r , c ));
                }
            }
            grid.push(row);
        }
    }

    (grid, trail_heads)
}

fn bfs_scores(grid: Vec<Vec<i32>>, start: Vec<(usize, usize)>) -> i32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = 0;

    let n = grid.len();
    let m = grid[0].len();

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    for head in start {
        queue.push_back(head);
        visited.insert(head);

        while let Some(pos) = queue.pop_front() {
            for (dx, dy) in &directions {
                let nx = pos.0 as isize + dx;
                let ny = pos.1 as isize + dy;

                if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if !visited.contains(&(nx, ny)) && grid[nx][ny] - grid[pos.0][pos.1] == 1 {
                        visited.insert((nx, ny));
                        if grid[nx][ny] == 9 {
                            paths += 1;
                            continue;
                        }
                        queue.push_back((nx, ny));
                    } 
                }
            }
        }

        visited.clear();
    }

    paths
}


fn bfs_ratings(grid: Vec<Vec<i32>>, start: Vec<(usize, usize)>) -> i32 {
    //let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut paths = 0;

    let n = grid.len();
    let m = grid[0].len();

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    for head in start {
        queue.push_back(head);

        while let Some(pos) = queue.pop_front() {
            for (dx, dy) in &directions {
                let nx = pos.0 as isize + dx;
                let ny = pos.1 as isize + dy;

                if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if grid[nx][ny] - grid[pos.0][pos.1] == 1 {
                        if grid[nx][ny] == 9 {
                            paths += 1;
                            continue;
                        }
                        queue.push_back((nx, ny));
                    } 
                }
            }
        }

    }

    paths
}

pub mod solution {
    pub fn part1(input: &str) -> i32 {
        let (grid, trail_heads) = super::parse_input(input.trim());
        super::bfs_scores(grid, trail_heads)
    }

    pub fn part2(input: &str) -> i32 {
        let (grid, trail_heads) = super::parse_input(input.trim());
        super::bfs_ratings(grid, trail_heads)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 36);
    }

    #[test]
    fn test_part2() {
        let sol2 = solution::part2(TEST);
        assert_eq!(sol2, 81);
    }

}
