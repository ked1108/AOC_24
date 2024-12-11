use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_forward(pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::Up if pos.0 > 0 => Some((pos.0 - 1, pos.1)),
        Direction::Right => Some((pos.0, pos.1 + 1)),
        Direction::Down => Some((pos.0 + 1, pos.1)),
        Direction::Left if pos.1 > 0 => Some((pos.0, pos.1 - 1)),
        _ => None,
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut grid = Vec::new();
    let mut marker_position = None;

    for (row_idx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_idx, ch) in line.chars().enumerate() {
            if ch == '^' {
                marker_position = Some((row_idx, col_idx));
            }
            row.push(ch);
        }
        grid.push(row);
    }

    (grid, marker_position.expect("Marker '^' not found in the grid"))
}


fn march(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> i32 {
    let mut grid = map.clone();
    let mut dir = Direction::Up;
    let mut count = 1;
    let n = grid.len();
    let m = grid[0].len();

    let mut pos = start_pos;

    loop {
        if let Some(next_pos) = move_forward(pos, dir) {
            if next_pos.0 >= n || next_pos.1 >= m {
                break; 
            }

            if grid[next_pos.0][next_pos.1] == '.' {
                grid[next_pos.0][next_pos.1] = 'X';
                count += 1;
                pos = next_pos; 
            } else if grid[next_pos.0][next_pos.1] == '#' {
                dir = turn_right(dir);
            } else {
                pos = next_pos;
            }
        } else {
            break;
        }
    }

    count
}


fn block(map: &Vec<Vec<char>>, start_pos: (usize, usize)) -> i32 {
    let mut loop_count = 0;

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '.' && (r, c) != start_pos {
                let mut grid = map.clone();
                grid[r][c] = '#';
                if simulate(&grid, start_pos, Direction::Up).is_some() {
                    loop_count += 1;
                }
            }
        }
    }

    loop_count
}

fn simulate(
    map: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    start_dir: Direction,
) -> Option<HashSet<(usize, usize, Direction)>> {
    let mut visited_states = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;

    loop {
        let state = (pos.0, pos.1, dir);
        visited_states.insert(state);

        if let Some(next_pos) = move_forward(pos, dir) {
            if next_pos.0 >= map.len() || next_pos.1 >= map[0].len() {
                break;
            }
            else if map[next_pos.0][next_pos.1] == '#'
            {
                dir = turn_right(dir);
                let turn_pos = move_forward(pos, dir)?;
                if visited_states.contains(&(turn_pos.0, turn_pos.1, dir)) {
                    return Some(visited_states);
                }
            } else {
                pos = next_pos; 
            }
        } else {
            break; 
        }
    }

    None 
}


pub mod solution {
    pub fn part1(input: &str) -> i32 {
        let (grid, pos) = super::parse_input(input.trim());

        super::march(&grid, pos)
    }


    pub fn part2(input: &str) -> i32 {
        let (grid, pos) = super::parse_input(input.trim());

        super::block(&grid, pos)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    const TEST: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_part1() {
        let sol = solution::part1(TEST);
        assert_eq!(sol, 41)
    }

    #[test]
    fn test_part2() {
        let sol = solution::part2(TEST);
        assert_eq!(sol, 6)
    }

}
