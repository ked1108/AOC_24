use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    row: isize,
    col: isize
}

impl Position {
    fn in_bounds(self: &Position, size: (isize, isize))  -> bool {
        if self.row < 0 || self.col < 0 || self.row >= size.0 || self.col >= size.1 {
            return false;
        }

        true
    }

}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn generate_hashmap(grid: Vec<Vec<char>>) -> HashMap<char, Vec<Position>> {
    let mut map: HashMap<char, Vec<Position>> = HashMap::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch != '.' {
                let map_pos = Position { row: r as isize, col: c as isize };
                map.entry(ch)
                    .or_insert(Vec::new()) 
                    .push(map_pos);
            }
        }
    }

    map
}

fn get_antinodes(char_hash: HashMap<char, Vec<Position>>, size: (isize, isize)) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions) in char_hash {
        let n = positions.len();

        for i in 0..n {
            for j in i + 1..n {
                let pos1 = positions[i];
                let pos2 = positions[j];

                let dr = pos2.row - pos1.row;
                let dc = pos2.col - pos1.col;

                let next_antinode = Position {
                    row: pos2.row + dr,
                    col: pos2.col + dc,
                };

                let prev_antinode = Position {
                    row: pos1.row - dr,
                    col: pos1.col - dc,
                };

                if next_antinode.in_bounds(size) {
                    antinodes.insert(next_antinode);
                }
                if prev_antinode.in_bounds(size) {
                    antinodes.insert(prev_antinode);
                }
            }
        }
    }

    antinodes.len()
}


fn get_antinodes_in_line(char_hash: HashMap<char, Vec<Position>>, size: (isize, isize)) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions) in char_hash {
        let n = positions.len();

        for i in 0..n {
            for j in i + 1..n {
                let pos1 = positions[i];
                let pos2 = positions[j];

                antinodes.insert(pos1);
                antinodes.insert(pos2);

                let dr = pos2.row - pos1.row;
                let dc = pos2.col - pos1.col;

                let mut next_antinode = Position {
                    row: pos2.row + dr,
                    col: pos2.col + dc,
                };

                while next_antinode.in_bounds(size) {
                    antinodes.insert(next_antinode);
                    next_antinode = Position {
                        row: next_antinode.row + dr,
                        col: next_antinode.col + dc
                    }
                }

                let mut prev_antinode = Position {
                    row: pos1.row - dr,
                    col: pos1.col - dc,
                };

                while prev_antinode.in_bounds(size) {
                    antinodes.insert(prev_antinode);
                    prev_antinode = Position {
                        row: prev_antinode.row - dr,
                        col: prev_antinode.col - dc
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub mod solution {
    pub fn part1(input: &str) -> i32 {
        let antena_map = super::parse_input(input);
        let size = (antena_map.len() as isize, antena_map[0].len() as isize);
        let char_hash = super::generate_hashmap(antena_map);
        super::get_antinodes(char_hash, size) as i32
    }

    pub fn part2(input: &str) -> i32 {
        let antena_map = super::parse_input(input);
        let size = (antena_map.len() as isize, antena_map[0].len() as isize);
        let char_hash = super::generate_hashmap(antena_map);
        super::get_antinodes_in_line(char_hash, size) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 14);
    }

    #[test]
    fn test_part2() {
        let sol2 = solution::part2(TEST);
        assert_eq!(sol2, 34);
    }

}
