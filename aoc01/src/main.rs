fn find_patterns(lines: &Vec<String>, patterns: &Vec<(&str, i32)>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut firstoffset = None;
        let mut lastoffset = None;
        let mut first = 0;
        let mut last = 0;
        for (pat, val) in patterns.iter() {
            if let Some(offset) = line.find(pat) {
                if firstoffset.is_none() || offset < firstoffset.unwrap() {
                    firstoffset = Some(offset);
                    first = *val;
                }
            }
            if let Some(offset) = line.rfind(pat) {
                if lastoffset.is_none() || offset > lastoffset.unwrap() {
                    lastoffset = Some(offset);
                    last = *val;
                }
            }
        }
        sum += 10 * first + last;
    }
    sum
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();

    let patterns = vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    println!("Part 1: {}", find_patterns(&lines, &patterns));
    let patterns = vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    println!("Part 2: {}", find_patterns(&lines, &patterns));
}
