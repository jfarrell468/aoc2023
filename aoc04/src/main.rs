use std::collections::HashSet;

struct Game {
    winning: HashSet<u32>,
    yours: Vec<u32>,
}

impl Game {
    fn parse(g: &str) -> Game {
        let (_game, rest) = g.split_once(": ").unwrap();
        let (winning, yours) = rest.split_once(" | ").unwrap();
        Game {
            winning: winning
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
            yours: yours
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect(),
        }
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let mut score = 0;
    for line in &lines {
        let g = Game::parse(&line);
        let num_matches = g.yours.iter().filter(|y| g.winning.contains(y)).count() as u32;
        if num_matches > 0 {
            score += 2_u32.pow(num_matches - 1);
        }
    }
    println!("Part 1: {}", score);
    let mut copies = Vec::new();
    for line in &lines {
        copies.push(1_u32);
    }
    for (idx, line) in lines.iter().enumerate() {
        let g = Game::parse(&line);
        let num_matches = g.yours.iter().filter(|y| g.winning.contains(y)).count() as usize;
        for m in 1..=num_matches {
            copies[idx + m] += copies[idx];
        }
    }
    println!("Part 2: {}", copies.iter().sum::<u32>());
}
