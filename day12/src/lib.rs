use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    North,
    South,
    East,
    West,
}


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


fn flood_fill_sides(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_r: usize,
    start_c: usize,
) -> (usize, HashMap<(usize, usize), Vec<Side>>) {

    let rows = map.len();
    let cols = map[0].len();
    let plant_type = map[start_r][start_c];

    let mut edges: HashMap<(usize, usize), Vec<Side>> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((start_r, start_c));
    visited[start_r][start_c] = true;

    let mut area = 0;

    let directions = [
        (-1, 0, Side::North),  
        (1, 0, Side::South),   
        (0, -1, Side::West),   
        (0, 1, Side::East),    
    ];

    while let Some((r, c)) = queue.pop_front() {
        area += 1;

        for &(dr, dc, side) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                edges.entry((r, c)).or_default().push(side);
            } else {
                let nr = nr as usize;
                let nc = nc as usize;

                if map[nr][nc] != plant_type {
                    edges.entry((r, c)).or_default().push(side);
                } else if !visited[nr][nc] {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    (area, edges)
}


fn count_sides(edges: &HashMap<(usize, usize), Vec<Side>>) -> usize {
    let mut row_sides: HashSet<(usize, Side)> = HashSet::new();
    let mut col_sides: HashSet<(usize, Side)> = HashSet::new();

    for (&(r, c), sides) in edges {
        for &side in sides {
            if side == Side::East || side == Side::West {
                println!("{c}: {:?}", side);
                col_sides.insert((c, side));
            } else {
                row_sides.insert((r, side));
            }
        }
    }

    //println!("{:?}", row_sides);
    //println!("{:?}", col_sides);
    col_sides.len() + row_sides.len()
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
        let map = super::parse_input(input.trim());

        let rows = map.len();
        let cols = map[0].len();

        let mut visited = vec![vec![false; cols]; rows];
        let mut total_price = 0;

        for r in 0..rows {
            for c in 0..cols {
                if !visited[r][c] {
                    let (area, edges) = super::flood_fill_sides(&map, &mut visited, r, c);
                    let sides = super::count_sides(&edges);
                    println!("{area} * {sides}");
                    total_price += area * sides;
                }
            }
        }

        total_price
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

    const TEST2: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

    const TEST3: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;

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

    #[test]
    fn test_part2_test2() {
        let sol2 = solution::part2(TEST2);
        assert_eq!(sol2, 368);
    }
    
    #[test]
    fn test_part2_test3() {
        let sol2 = solution::part2(TEST3);
        assert_eq!(sol2, 236);
    }

}
