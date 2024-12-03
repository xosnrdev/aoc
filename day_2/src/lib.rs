pub fn run() {
    let input =
        std::fs::read_to_string("./day_2/input.txt").expect("Failed to read file to string");

    let solution_part_1: i32 = parse_input(input.clone())
        .iter()
        .filter(|&seq| solution_part_1(seq))
        .count()
        .try_into()
        .unwrap();

    let solution_part_2: i32 = parse_input(input.clone())
        .iter()
        .filter(|&seq| solution_part_2(seq))
        .count()
        .try_into()
        .unwrap();

    println!(
        "Safe Report {} with Problem Dampener {}",
        solution_part_1, solution_part_2
    )
}

fn solution_part_1(sequence: &[i32]) -> bool {
    // WE NEED AT LEAST TWO LEVELS BEFORE PROCESSING
    if sequence.len() < 2 {
        return false;
    }

    // WE DETERMINE THE DIRECTION INCREASING OR DECREASING AND VALIDATE
    // 0 = UNSET, 1 = INCREASING, -1 = DECREASING
    let mut direction = 0;

    // WE CHECK IF THE DIFFERENCE IS WITHIN THE VALID RANGE
    for window in sequence.windows(2) {
        let diff = window[1] - window[0];

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        // WE DETERMINE AND ENFORCE THE DIRECTION
        // 1 FOR POSITIVE, -1 FOR NEGATIVE
        let current_direction = diff.signum();
        if direction == 0 {
            // SET THE DIRECTION
            direction = current_direction
        } else if direction != current_direction {
            // MIXED DIRECTIONS ARE NOT ALLOWED
            return false;
        }
    }

    true
}

// WE CAN JUST CHECK IF A SEQUENCE CAN BE MADE SAFE BY REMOVING ONE LEVEL.
fn solution_part_2(sequence: &[i32]) -> bool {
    // WE TRY REMOVING EACH LEVEL AND CHECK IF THE MODIFIED SEQUENCE IS VALID
    for i in 0..sequence.len() {
        let mut modified_sequence = sequence.to_vec();
        modified_sequence.remove(i);

        if solution_part_1(&modified_sequence) {
            return true;
        }
    }
    false
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        let input = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#
        .to_string();

        let safe_count: i32 = parse_input(input)
            .iter()
            .filter(|&seq| solution_part_1(seq))
            .count()
            .try_into()
            .unwrap();

        assert_eq!(safe_count, 2)
    }

    #[test]
    fn test_solution_part_2() {
        let input = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#
        .to_string();

        let safe_count: i32 = parse_input(input)
            .iter()
            .filter(|&seq| solution_part_2(seq))
            .count()
            .try_into()
            .unwrap();

        assert_eq!(safe_count, 4)
    }
}
