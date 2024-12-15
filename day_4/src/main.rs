const INPUT: &str = include_str!("../input");

const MATCH_STR: &str = "XMAS";

/// A grid.
///
/// (0,0) is in the top left
struct Grid {
    raw: Vec<char>, // In the format of sequential rows
    x_count: usize,
    y_count: usize,
}

impl Grid {
    pub fn new() -> Self {
        let lines = INPUT.lines().collect::<Vec<&str>>();
        let y_count = lines.len();
        let x_count = lines.first().unwrap().len();

        let raw = lines
            .into_iter()
            .flat_map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        Self {
            raw,
            x_count,
            y_count,
        }
    }

    pub fn index(&self, x: usize, y: usize) -> Option<char> {
        if (x >= self.x_count) || (y > self.y_count) {
            return None;
        }

        let i = (y * self.x_count) + x;

        Some(self.raw[i])
    }

    pub fn index_unchecked(&self, x: usize, y: usize) -> char {
        let i = (y * self.x_count) + x;

        self.raw[i]
    }

    /// Scans in all directions for XMAS, starting at the given x position
    ///
    /// Returns the amount of matches found
    pub fn scan_at_x(&self, x: usize, y: usize) -> u64 {
        let mut count = 0;

        // left to right
        if x + 3 < self.x_count {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x + 1, y));
            buffer.push(self.index_unchecked(x + 2, y));
            buffer.push(self.index_unchecked(x + 3, y));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // right to left
        if x.checked_sub(3).is_some() {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x - 1, y));
            buffer.push(self.index_unchecked(x - 2, y));
            buffer.push(self.index_unchecked(x - 3, y));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // up to down
        if y + 3 < self.y_count {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x, y + 1));
            buffer.push(self.index_unchecked(x, y + 2));
            buffer.push(self.index_unchecked(x, y + 3));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // down to up
        if y.checked_sub(3).is_some() {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x, y - 1));
            buffer.push(self.index_unchecked(x, y - 2));
            buffer.push(self.index_unchecked(x, y - 3));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // Northeast
        if (x + 3 < self.x_count) && y.checked_sub(3).is_some() {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x + 1, y - 1));
            buffer.push(self.index_unchecked(x + 2, y - 2));
            buffer.push(self.index_unchecked(x + 3, y - 3));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // Southeast
        if (x + 3 < self.x_count) && (y + 3 < self.y_count) {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x + 1, y + 1));
            buffer.push(self.index_unchecked(x + 2, y + 2));
            buffer.push(self.index_unchecked(x + 3, y + 3));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // southwest
        if x.checked_sub(3).is_some() && (y + 3 < self.y_count) {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x - 1, y + 1));
            buffer.push(self.index_unchecked(x - 2, y + 2));
            buffer.push(self.index_unchecked(x - 3, y + 3));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        // northwest
        if x.checked_sub(3).is_some() && y.checked_sub(3).is_some() {
            let mut buffer = String::new();
            buffer.push(self.index_unchecked(x, y));
            buffer.push(self.index_unchecked(x - 1, y - 1));
            buffer.push(self.index_unchecked(x - 2, y - 2));
            buffer.push(self.index_unchecked(x - 3, y - 3));
            if buffer == MATCH_STR {
                count += 1;
            }
        }

        count
    }

    pub fn scan_at_a(&self, x: usize, y: usize) -> bool {
        if (x.checked_sub(1).is_none())
            || (y.checked_sub(1).is_none())
            || (x + 1 >= self.x_count)
            || (y + 1 >= self.y_count)
        {
            return false;
        }

        let northwest = self.index_unchecked(x - 1, y - 1);
        let southeast = self.index_unchecked(x + 1, y + 1);

        let northeast = self.index_unchecked(x + 1, y - 1);
        let southwest = self.index_unchecked(x - 1, y + 1);

        // northwest to southeast
        match northwest {
            'M' => {
                if southeast != 'S' {
                    return false;
                }
            }
            'S' => {
                if southeast != 'M' {
                    return false;
                }
            }
            _ => return false,
        }

        match northeast {
            'M' => {
                if southwest != 'S' {
                    return false;
                }
            }
            'S' => {
                if southwest != 'M' {
                    return false;
                }
            }
            _ => return false,
        }

        true
    }
}

fn main() {
    // We know that each XMAS only has one of each letter, so we only have to scan each time we see
    // one of the letters, and will see X

    let grid = Grid::new();

    let mut xmas_matches = 0;

    for x in 0..grid.x_count {
        for y in 0..grid.y_count {
            if grid.index_unchecked(x, y) == 'X' {
                xmas_matches += grid.scan_at_x(x, y);
            }
        }
    }

    println!("XMAS Matches: {}", xmas_matches);

    let mut x_mas_matches = 0;

    for x in 0..grid.x_count {
        for y in 0..grid.y_count {
            if (grid.index_unchecked(x, y) == 'A') && grid.scan_at_a(x, y) {
                x_mas_matches += 1;
            }
        }
    }

    println!("X-MAS Matches: {}", x_mas_matches)
}
