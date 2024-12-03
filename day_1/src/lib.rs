pub fn run() {
    let input =
        std::fs::read_to_string("./day_1/input.txt").expect("Failed to read file to string");

    println!(
        "Part One is {}, Part Two {}",
        solution_part_1(input.clone()),
        solution_part_2(input.clone())
    );
}

pub fn solution_part_1(input: String) -> i32 {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let mut ids = line.split_whitespace();

            Some((
                ids.next()?.parse::<i32>().ok()?,
                ids.next()?.parse::<i32>().ok()?,
            ))
        })
        .collect();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = pairs.into_iter().unzip();

    left.sort_unstable();
    right.sort_unstable();

    let result: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    result
}

pub fn solution_part_2(input: String) -> usize {
    let pairs: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|line| {
            let mut ids = line.split_whitespace();

            Some((
                ids.next()?.parse::<usize>().ok()?,
                ids.next()?.parse::<usize>().ok()?,
            ))
        })
        .collect();
    let (left, right): (Vec<usize>, Vec<usize>) = pairs.into_iter().unzip();

    let result = left
        .iter()
        .map(|num| right.iter().filter(|r| &num == r).count() * num)
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        let input = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#
        .to_string();

        let result = solution_part_1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_solution_part_2() {
        let input = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#
        .to_string();

        let result = solution_part_2(input);
        assert_eq!(result, 31);
    }
}
