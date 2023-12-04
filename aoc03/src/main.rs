use std::collections::HashMap;
use util::Adjacent;

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
            for (adj_r, adj_c) in s.raw.adjacent_to(self.row, c) {
                // println!("{}, {}", adj_r, adj_c);
                let adj = s.raw[adj_r][adj_c];
                if !adj.is_ascii_digit() && adj != '.' {
                    return true;
                }
            }
        }
        false
    }
    fn get_gear(&self, s: &Schematic) -> Option<(usize, usize)> {
        for c in self.col_start..=self.col_end {
            for (adj_r, adj_c) in s.raw.adjacent_to(self.row, c) {
                if s.raw[adj_r][adj_c] == '*' {
                    return Some((adj_r, adj_c));
                }
            }
        }
        None
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
    let mut gears = HashMap::new();
    for num in &schematic.numbers {
        if let Some((r, c)) = num.get_gear(&schematic) {
            gears.entry((r, c)).or_insert(Vec::new()).push(num.val);
        }
    }
    let mut sum_of_gear_ratios = 0;
    for ((_r, _c), part_nums) in gears {
        if part_nums.len() == 2 {
            sum_of_gear_ratios += part_nums[0] * part_nums[1];
        }
    }
    println!("Part 1: {}", sum_of_gear_ratios);
}
