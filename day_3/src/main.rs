const INPUT: &str = include_str!("../input");

fn main() {
    let sum: u64 = INPUT
        .match_indices("mul(")
        .filter_map(|(m_index, _)| {
            let offset = m_index + 4;

            let mut comma_index = None;

            let mut first_number_str = String::new();
            let mut second_number_str = String::new();

            for i in offset..(INPUT.len() - 1) {
                let c = INPUT.chars().nth(i).unwrap();

                match c.to_digit(10) {
                    Some(_) => {
                        first_number_str.push(c);
                    }
                    None => {
                        if c == ',' {
                            comma_index = Some(i);
                        }

                        break;
                    }
                }
            }

            comma_index?;

            for i in (comma_index.unwrap() + 1)..(INPUT.len() - 1) {
                let c = INPUT.chars().nth(i).unwrap();

                match c.to_digit(10) {
                    Some(_) => {
                        second_number_str.push(c);
                    }
                    None => {
                        if c == ')' {
                            return Some(
                                first_number_str.parse::<u64>().unwrap()
                                    * second_number_str.parse::<u64>().unwrap(),
                            );
                        }

                        break;
                    }
                }
            }

            None
        })
        .sum();

    println!("Sum: {}", sum);

    // Part 2
    let mut active = true;

    let mut last_scanned_index = 0;

    let sum: u64 = INPUT
        .match_indices("mul(")
        .filter_map(|(m_index, _)| {
            let s = &INPUT[last_scanned_index..m_index].chars();

            let highest_index_do = {
                let mut highest_index_do = None;

                for i in last_scanned_index..(m_index - 4) {
                    if &INPUT[i..i + 4] == "do()" {
                        highest_index_do = Some(i);
                    }
                }

                highest_index_do
            };

            let highest_index_dont = {
                let mut highest_index_dont = None;

                for i in last_scanned_index..(m_index - 7) {
                    if &INPUT[i..i + 7] == "don't()" {
                        highest_index_dont = Some(i);
                    }
                }

                highest_index_dont
            };

            if highest_index_do.is_some() || highest_index_dont.is_some() {
                last_scanned_index = m_index;
            }

            if highest_index_do.unwrap_or_default() > highest_index_dont.unwrap_or_default() {
                if !active {
                    println!("Switched to active");
                }
                active = true;
            }

            if highest_index_do.unwrap_or_default() < highest_index_dont.unwrap_or_default() {
                if active {
                    println!("Switched to inactive");
                }
                active = false;
            }

            if !active {
                return None;
            }

            let offset = m_index + 4;

            let mut comma_index = None;

            let mut first_number_str = String::new();
            let mut second_number_str = String::new();

            for i in offset..(INPUT.len() - 1) {
                let c = INPUT.chars().nth(i).unwrap();

                match c.to_digit(10) {
                    Some(_) => {
                        first_number_str.push(c);
                    }
                    None => {
                        if c == ',' {
                            comma_index = Some(i);
                        }

                        break;
                    }
                }
            }

            comma_index?;

            for i in (comma_index.unwrap() + 1)..(INPUT.len() - 1) {
                let c = INPUT.chars().nth(i).unwrap();

                match c.to_digit(10) {
                    Some(_) => {
                        second_number_str.push(c);
                    }
                    None => {
                        if c == ')' {
                            return Some(
                                first_number_str.parse::<u64>().unwrap()
                                    * second_number_str.parse::<u64>().unwrap(),
                            );
                        }

                        break;
                    }
                }
            }

            None
        })
        .sum::<u64>();

    println!("With modifiers: {}", sum);
}
