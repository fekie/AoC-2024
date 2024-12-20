const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy)]
pub struct Pair(pub u64, pub u64);

impl Pair {
    pub fn new(line: &str) -> Self {
        Self(line[0..=1].parse().unwrap(), line[3..=4].parse().unwrap())
    }
}

#[derive(Debug)]
pub struct Update(pub Vec<u64>);

impl Update {
    pub fn new(line: &str) -> Self {
        Self(line.split(',').map(|x| x.parse().unwrap()).collect())
    }

    pub fn is_valid(&self, mut pairs: Vec<Pair>) -> bool {
        // Retain relevant pairs
        pairs.retain(|pair| {
            let first = pair.0;
            let mut first_match = false;

            for v in &self.0 {
                if first == *v {
                    first_match = true;
                    break;
                }
            }

            if !first_match {
                return false;
            }

            let second = pair.1;
            let mut second_match = false;

            for v in &self.0 {
                if second == *v {
                    second_match = true;
                    break;
                }
            }

            if !second_match {
                return false;
            }

            true
        });

        let mut valid = true;

        for pair in pairs {
            let first_index = self.0.iter().position(|v| *v == pair.0);
            let second_index = self.0.iter().position(|v| *v == pair.1);

            if first_index > second_index {
                valid = false;
                break;
            }
        }

        valid
    }

    pub fn middle(&self) -> u64 {
        let len = self.0.len();
        self.0[len / 2]
    }
}

fn main() {
    let lines = INPUT.lines();

    let pairs = {
        let mut lines = lines.clone().collect::<Vec<&str>>();
        lines.retain(|line| line.chars().nth(2).unwrap_or_default() == '|');
        lines.into_iter().map(Pair::new).collect::<Vec<_>>()
    };

    let updates = {
        let mut lines = lines.clone().collect::<Vec<&str>>();
        lines.retain(|line| line.chars().nth(2).unwrap_or_default() == ',');
        lines.into_iter().map(Update::new).collect::<Vec<_>>()
    };

    let middle_sum: u64 = updates
        .into_iter()
        .filter_map(|update| match update.is_valid(pairs.clone()) {
            true => Some(update.middle()),
            false => None,
        })
        .sum();

    println!("Middle Sum: {}", middle_sum);
}
