fn parse_input(input: &str) -> Vec<Option<usize>> {
    input.chars()
        .into_iter()
        .enumerate()
        .fold(Vec::new(), |mut disk, (i, ch)| {
            let mut n = ch.to_digit(10).unwrap_or(0);
            if i % 2 == 0 {
                while n > 0 {
                    disk.push(Some(i/2));
                    n -= 1;
                }
            } else {
                while n > 0 {
                    disk.push(None);
                    n -= 1;
                }
            }

            disk
        })
}

fn compress(uncomp: &mut Vec<Option<usize>>) {
    let mut left = 0;
    let mut right = uncomp.len() - 1;
    
    while left < right {
        while uncomp[left].is_some() {
            left += 1;
        }

        while uncomp[right].is_none() {
            right -= 1;
        }

        while uncomp[left].is_none() && uncomp[right].is_some()  && left < right{
            uncomp[left] = uncomp[right];
            uncomp[right] = None;

            left += 1;
            right -= 1;
        }
    }
}

fn best_fit_compress(uncomp: &mut Vec<Option<usize>>) {
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    let mut pos = 0;
    while pos < uncomp.len() {
        if let Some(file_id) = uncomp[pos] {
            let mut size = 0;
            while pos < uncomp.len() && uncomp[pos] == Some(file_id) {
                size += 1;
                pos += 1;
            }
            files.push((pos - size, size, file_id));
        } else {
            let mut size = 0;
            while pos < uncomp.len() && uncomp[pos].is_none() {
                size += 1;
                pos += 1;
            }
            spaces.push((pos - size, size));
        }
    }

    files.sort_by(|a, b| b.2.cmp(&a.2));
    for (pos, size, file_id) in files {
        for space in &mut spaces {
            if space.0 < pos && space.1 >= size {
                for i in 0..size {
                    uncomp[pos + i] = None;
                    uncomp[space.0 + i] = Some(file_id);
                }

                space.0 += size;
                space.1 -= size;
                break;
            }
        }
    }
}

fn checksum(uncomp: Vec<Option<usize>>) -> usize {
    uncomp.iter()
        .enumerate()
        .fold(0, |acc, (idx, ch)| acc + (idx * ch.unwrap_or(0)))
}


pub mod solution {
    pub fn part1(input: &str) -> usize {
        let mut uncompressed = super::parse_input(input.trim());
        super::compress(&mut uncompressed);
        super::checksum(uncompressed) 
    }

    pub fn part2(input: &str) -> usize {
        let mut uncompressed = super::parse_input(input.trim());
        super::best_fit_compress(&mut uncompressed);
        super::checksum(uncompressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 1928);
    }

    #[test]
    fn test_part2() {
        let sol2 = solution::part2(TEST);
        assert_eq!(sol2, 2858);
    }

}
