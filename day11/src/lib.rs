use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok()) 
        .collect()
}

fn blink_individual(stone: u64, count: u64, splits: u64, cache: Arc<Mutex<HashMap<(u64, u64), u64>>>) -> u64 {
    // Cache check
    {
        let cache_lock = cache.lock().unwrap();
        if let Some(&cached_result) = cache_lock.get(&(stone, count)) {
            return cached_result;
        }
    }

    if count == 0 {
        return splits;
    } else if stone == 0 {
        return blink_individual(1_u64, count - 1, splits, Arc::clone(&cache));
    } else {
        let num_digits = ((stone as f64).log10().floor() as u32) + 1;
        if num_digits % 2 == 0 {
            let half_digits = num_digits / 2;
            let divisor = 10_u64.pow(half_digits);
            let left = stone / divisor;
            let right = stone % divisor;

            let result = blink_individual(left, count - 1, splits, Arc::clone(&cache))
                + blink_individual(right, count - 1, splits, Arc::clone(&cache)) + 1;

            // Cache the result for (stone, count)
            {
                let mut cache_lock = cache.lock().unwrap();
                cache_lock.insert((stone, count), result);
            }

            return result;
        } else {
            let result = blink_individual(stone * 2024, count - 1, splits, Arc::clone(&cache));

            // Cache the result for (stone, count)
            {
                let mut cache_lock = cache.lock().unwrap();
                cache_lock.insert((stone, count), result);
            }

            return result;
        }
    }
}


pub mod solution {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn part1(input: &str) -> u64 {
        let stones = super::parse_input(input.trim());
        let mut sum: u64 = stones.len() as u64;

        let cache = Arc::new(Mutex::new(HashMap::new())); 

        let mut handles = Vec::new();
        for stone in stones {
            let cache_clone = Arc::clone(&cache); 
            let handle = thread::spawn(move || {
                let splits = super::blink_individual(stone, 25, 0, cache_clone);

                splits
            });

            handles.push(handle);
        }

        for handle in handles {
            sum += handle.join().unwrap();
        }

        sum
    } 

    pub fn part2(input: &str) -> u64 {
        let stones = super::parse_input(input.trim());
        let mut sum: u64 = stones.len() as u64;

        let cache = Arc::new(Mutex::new(HashMap::new()));

        let mut handles = Vec::new();
        for stone in stones {
            let cache_clone = Arc::clone(&cache); 
            let handle = thread::spawn(move || {
                let splits = super::blink_individual(stone, 75, 0, cache_clone);

                splits
            });

            handles.push(handle);
        }

        for handle in handles {
            sum += handle.join().unwrap();
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = r#"125 17"#;

    #[test]
    fn test_part1() {
        let sol1 = solution::part1(TEST);
        assert_eq!(sol1, 55312);
    }

    //#[test]
    //fn test_part2() {
    //    let sol2 = solution::part2(TEST);
    //    assert_eq!(sol2, 0);
    //}
    //
}
