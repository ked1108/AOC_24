use std::collections::{HashMap, VecDeque};

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    
    let lines: Vec<&str> = input.lines().collect();
    
    for line in lines {
        if line.is_empty() {
            continue;
        }
        
        if line.contains(',') {
            let update: Vec<i32> = line.split(',')
                .filter_map(|num| num.parse().ok()) 
                .collect();
            updates.push(update);

        } else {
            if let Some((x, y)) = line.split_once('|') {
                if let (Ok(x_num), Ok(y_num)) = (x.parse(), y.parse()) {
                    rules.push((x_num, y_num));
                }
            }
        }
    }
    
    (rules, updates)
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>) -> Option<Vec<i32>> {
    let mut in_degree = HashMap::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    for &node in graph.keys() {
        in_degree.entry(node).or_insert(0);
    }
    
    for neighbors in graph.values() {
        for &neighbor in neighbors {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }
    
    for (&node, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node);
        }
    }
    
    while let Some(node) = queue.pop_front() {
        result.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let deg = in_degree.get_mut(&neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    
    if result.len() == graph.len() {
        Some(result)
    } else {
        None
    }
}

fn fix_sorted_order(update: &[i32], graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut sub_graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for &page in update {
        if let Some(neighbors) = graph.get(&page) {
            let relevant_neighbours = neighbors
                .iter()
                .filter(|&neighbour| update.contains(neighbour))
                .copied()
                .collect();
            sub_graph.insert(page, relevant_neighbours);
        } else {
            sub_graph.insert(page, Vec::new());
        }
    }
    
    if let Some(sorted_order) = topological_sort(&sub_graph) {
        sorted_order
    } else {
        Vec::new()
    }
}


fn is_order_valid(update: &[i32], rule_map: &HashMap<i32, Vec<i32>>) -> bool {
    let page_positions: HashMap<i32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for (&before, afters) in rule_map {
        if let Some(&before_pos) = page_positions.get(&before) {
            for &after in afters {
                if let Some(&after_pos) = page_positions.get(&after) {
                    if before_pos > after_pos {
                        return false;
                    }
                }
            }
        }
    }

    true
}


pub mod solution {
    use super::*;
    use std::collections::HashMap;

    pub fn part1(input: &str) -> i32 {

        let (ordering_rules, updates) = parse_input(input);

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for (x, y) in ordering_rules {
            graph.entry(x).or_insert_with(Vec::new).push(y);
        }

        for update in &updates {
            for &page in update {
                graph.entry(page).or_insert_with(Vec::new);
            }
        }

        let mut total_middle_pages = 0;
        for update in updates {
            if is_order_valid(&update, &graph) {
                total_middle_pages += update[update.len() / 2];
            }
        }

        total_middle_pages
    }

    pub fn part2(input: &str) -> i32 {

        let (ordering_rules, updates) = parse_input(input);

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for (x, y) in ordering_rules {
            graph.entry(x).or_insert_with(Vec::new).push(y);
        }

        for update in &updates {
            for &page in update {
                graph.entry(page).or_insert_with(Vec::new);
            }
        }

        let mut total_middle_pages = 0;
        for update in updates {
            if !is_order_valid(&update, &graph) {
                let fixed = fix_sorted_order(&update, &graph);
                total_middle_pages += fixed[fixed.len() / 2];
            }
        }

        total_middle_pages
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part1() {
        let solution1 = solution::part1(TEST);
        assert_eq!(solution1, 143);
    }

    #[test]
    fn test_part2() {
        let solution2 = solution::part2(TEST);
        assert_eq!(solution2, 123);
    }

}
