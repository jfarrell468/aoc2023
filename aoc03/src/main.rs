#[derive(Debug)]
struct Number {
    row: usize,
    col_start: usize,
    col_end: usize,
    val: u32,
}
impl Number {
    fn is_part_number(&self, s: &Schematic) -> bool {
        // println!("{:?}", self);
        for c in self.col_start..=self.col_end {
            for (adj_r, adj_c) in s.adjacent(self.row, c) {
                // println!("{}, {}", adj_r, adj_c);
                let adj = s.raw[adj_r][adj_c];
                if !adj.is_ascii_digit() && adj != '.' {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct Schematic {
    raw: Vec<Vec<char>>,
    numbers: Vec<Number>,
}

impl Schematic {
    fn from(lines: Vec<String>) -> Schematic {
        let mut numbers = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            let mut accum = String::from("");
            let mut col_start = 0;
            let mut col_end = 0;
            for (col, c) in line.char_indices() {
                col_end = col; // For handling number at end.
                if c.is_ascii_digit() {
                    if accum.is_empty() {
                        col_start = col;
                    }
                    accum.push(c);
                } else {
                    if !accum.is_empty() {
                        let val: u32 = accum.parse().unwrap();
                        // println!("Found {}", val);
                        numbers.push(Number {
                            row,
                            col_start,
                            col_end: col - 1,
                            val,
                        });
                    }
                    accum = String::from("");
                }
            }
            if !accum.is_empty() {
                let val: u32 = accum.parse().unwrap();
                // println!("Found {}", val);
                numbers.push(Number {
                    row,
                    col_start,
                    col_end,
                    val,
                });
            }
        }
        Schematic {
            raw: lines.iter().map(|l| l.chars().collect()).collect(),
            numbers,
        }
    }

    fn adjacent(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut adj = Vec::new();
        for rdelta in [-1, 0, 1] {
            if (rdelta == -1 && r <= 0) || (rdelta == 1 && r >= self.raw.len() - 1) {
                continue;
            }
            for cdelta in [-1, 0, 1] {
                if (cdelta == -1 && c <= 0)
                    || (cdelta == 1 && c >= self.raw[0].len() - 1)
                    || (rdelta == 0 && cdelta == 0)
                {
                    continue;
                }
                adj.push(((r as i32 + rdelta) as usize, (c as i32 + cdelta) as usize));
            }
        }
        adj
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let mut sum_of_part_numbers = 0;
    let schematic = Schematic::from(lines);
    // println!("{:?}", schematic);
    for number in &schematic.numbers {
        if number.is_part_number(&schematic) {
            // println!("Is part number: {:?}", number);
            sum_of_part_numbers += number.val;
        } else {
            // println!("Is not part number: {:?}", number);
        }
    }
    println!("Part 1: {}", sum_of_part_numbers);
}
