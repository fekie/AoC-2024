use std::cmp::Ordering;

const INPUT: &str = include_str!("../input");

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

fn is_safe(nums: &[i64]) -> bool {
    let diff = nums[1] - nums[0];

    let direction = match diff.cmp(&0) {
        Ordering::Greater => Direction::Up,
        Ordering::Equal => return false,
        Ordering::Less => Direction::Down,
    };

    for pair in nums.windows(2) {
        let diff = pair[1] - pair[0];

        if diff == 0 {
            return false;
        }

        if diff.abs() > 3 {
            return false;
        }

        match diff.cmp(&0) {
            Ordering::Greater => {
                if direction != Direction::Up {
                    return false;
                }
            }
            Ordering::Equal => unreachable!(),
            Ordering::Less => {
                if direction != Direction::Down {
                    return false;
                }
            }
        };
    }

    true
}

fn main() {
    // Day 1
    let safe_count = INPUT
        .lines()
        .filter_map(|x| {
            let nums: Vec<_> = x
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            match is_safe(&nums) {
                true => Some(()),
                false => None,
            }
        })
        .count();

    println!("Safe Report Count: {}", safe_count);

   // Day 2
    let safe_count = INPUT
        .lines()
        .filter_map(|x| {
            let nums: Vec<_> = x
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            if is_safe(&nums) {
                return Some(());
            }

            // Make permutations of nums, each removing 1 level

            for i in 0..nums.len() {
                let mut permutated = nums.clone();
                permutated.remove(i);

                if is_safe(&permutated) {
                    return Some(());
                }
            }

            None
        })
        .count();

    println!("Safe Report Count With Up To 1 Failure: {}", safe_count);
}
