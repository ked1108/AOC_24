fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| {
            line
                .chars()
                .collect()
        }).collect()
}

fn check_word(
    grid: &Vec<Vec<char>>,
    word: &Vec<char>,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    n: usize
) -> bool {

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    if n == 4 {
        return true;
    }

    if x < 0 || x >= cols || y < 0 || y >= rows || grid[x as usize][y as usize] != word[n] {
        return false;
    }

    return check_word(grid, word, x+dx, y+dy, dx, dy, n+1);
}

pub mod solution {
    pub fn part1(input: &str) -> i32 {
        let grid = super::parse_input(input);
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        let word = "XMAS".chars().collect::<Vec<_>>();

        let directions: [(i32, i32); 8] = [
            ( 0,  1),
            ( 1,  1),
            ( 1,  0),
            ( 1, -1),
            ( 0, -1),
            (-1, -1),
            (-1,  0),
            (-1,  1)
        ];

        let mut count = 0;
        
        for y in 0..rows {
            for x in 0..cols {
                for &(dx, dy) in &directions {
                    if super::check_word(&grid, &word, x, y, dx, dy, 0) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn part2(input: &str) -> i32 {
        let grid = super::parse_input(input);
        let rows = grid.len();
        let cols = grid[0].len();

        let mut count = 0;

        for y in 1..rows-1 {
            for x in 1..cols-1 {
                if grid[y][x] == 'A' && grid[y-1][x-1] == 'M' && grid[y+1][x-1] == 'M' && grid[y-1][x+1] == 'S' && grid[y+1][x+1] == 'S' {
                    count += 1;
                } else if grid[y][x] == 'A' && grid[y-1][x-1] == 'S' && grid[y+1][x-1] == 'S' && grid[y-1][x+1] == 'M' && grid[y+1][x+1] == 'M' {
                    count += 1;
                } else if grid[y][x] == 'A' && grid[y-1][x-1] == 'S' && grid[y+1][x-1] == 'M' && grid[y-1][x+1] == 'S' && grid[y+1][x+1] == 'M' {
                    count += 1;
                } else if grid[y][x] == 'A' && grid[y-1][x-1] == 'M' && grid[y+1][x-1] == 'S' && grid[y-1][x+1] == 'M' && grid[y+1][x+1] == 'S' {
                    count += 1; 
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_data = "MMMSXXMASM\n\
                          MSAMXMSMSA\n\
                          AMXSXMAAMM\n\
                          MSAMASMSMX\n\
                          XMASAMXAMM\n\
                          XXAMMXXAMA\n\
                          SMSMSASXSS\n\
                          SAXAMASAAA\n\
                          MAMMMXMMMM\n\
                          MXMXAXMASX";


        let result = solution::part1(input_data);

        assert_eq!(result, 18);
    }

    fn test_part2() {
        let input_data = "MMMSXXMASM\n\
                          MSAMXMSMSA\n\
                          AMXSXMAAMM\n\
                          MSAMASMSMX\n\
                          XMASAMXAMM\n\
                          XXAMMXXAMA\n\
                          SMSMSASXSS\n\
                          SAXAMASAAA\n\
                          MAMMMXMMMM\n\
                          MXMXAXMASX";


        let result = solution::part2(input_data);

        assert_eq!(result, 9);
    }
}
