use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        grid.push(line.chars().collect::<Vec<_>>());
    }
    
    grid
}

fn flood_fill(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_r: usize,
    start_c: usize,
) -> (usize, usize) {

    let rows = map.len();
    let cols = map[0].len();
    let plant_type = map[start_r][start_c];

    let mut queue = VecDeque::new();
    queue.push_back((start_r, start_c));
    visited[start_r][start_c] = true;

    let mut area = 0;
    let mut perimeter = 0;

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((r, c)) = queue.pop_front() {
        area += 1;

        for &(dr, dc) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                perimeter += 1;
            } else {
                let nr = nr as usize;
                let nc = nc as usize;

                if map[nr][nc] != plant_type {
                    perimeter += 1;
                } else if !visited[nr][nc] {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    (area, perimeter)
}




pub mod solution {
    pub fn part1(input: &str) -> usize {
        let map = super::parse_input(input.trim());

        let rows = map.len();
        let cols = map[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut total_price = 0;

        for r in 0..rows {
            for c in 0..cols {
                if !visited[r][c] {
                    let (area, perimeter) = super::flood_fill(&map, &mut visited, r, c);
                    total_price += area * perimeter;
                }
            }
        }

        total_price
    }

    //TODO: Finish this 
    pub fn part2(input: &str) -> usize {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 1930);
    }

    #[test]
    fn test_part2() {
        let sol2 = solution::part2(TEST);
        assert_eq!(sol2, 1206);
    }

}
