const INPUT: &str = include_str!("../input");

fn make_sorted_lists() -> (Vec<u64>, Vec<u64>) {
    let (list1_paired, list2_paired): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .flat_map(|x| x.split_whitespace().map(|s| s.parse::<u64>().unwrap()))
        .enumerate()
        .partition(|(i, _)| (i % 2) == 0);

    let mut list1: Vec<u64> = list1_paired.into_iter().map(|(_, v)| v).collect();
    let mut list2: Vec<u64> = list2_paired.into_iter().map(|(_, v)| v).collect();

    list1.sort();
    list2.sort();

    (list1, list2)
}

fn main() {
    // Part 1
    let (list1, list2) = make_sorted_lists();

    let sum: i64 = list1
        .clone()
        .into_iter()
        .zip(&list2)
        .map(|(left, right)| (left as i64 - *right as i64).abs())
        .sum();

    println!("Sum: {}", sum);

    // Part 2
    let similarity_score: u64 = list1
        .into_iter()
        .map(|left| {
            let times_found = list2.iter().filter(|right| left == **right).count() as u64;

            left * times_found
        })
        .sum();

    println!("Similarity Score: {}", similarity_score);
}
