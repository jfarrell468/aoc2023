#[derive(Debug)]
struct Draw {
    r: u32,
    g: u32,
    b: u32,
}

impl Draw {
    fn new() -> Draw {
        Draw { r: 0, g: 0, b: 0 }
    }
    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn parse(g: &str) -> Game {
        let (game, rest) = g.split_once(": ").unwrap();
        Game {
            id: game.strip_prefix("Game ").unwrap().parse().unwrap(),
            draws: rest
                .split("; ")
                .map(|draw| {
                    let mut d = Draw::new();
                    for count_and_color in draw.split(", ") {
                        let (count, color) = count_and_color.split_once(" ").unwrap();
                        match color {
                            "red" => d.r = count.parse().unwrap(),
                            "green" => d.g = count.parse().unwrap(),
                            "blue" => d.b = count.parse().unwrap(),
                            _ => panic!("Unrecognized color"),
                        }
                    }
                    d
                })
                .collect(),
        }
    }
    fn possible(&self) -> bool {
        for draw in &self.draws {
            if draw.r > 12 || draw.g > 13 || draw.b > 14 {
                return false;
            }
        }
        true
    }
    fn min_cubes(&self) -> Draw {
        let mut min = Draw::new();
        for draw in &self.draws {
            min.r = std::cmp::max(draw.r, min.r);
            min.g = std::cmp::max(draw.g, min.g);
            min.b = std::cmp::max(draw.b, min.b);
        }
        min
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    let mut sum_of_ids = 0;
    let mut power = 0;
    for line in lines {
        let game = Game::parse(&line);
        // println!("{:?}", game);
        if game.possible() {
            sum_of_ids += game.id;
        }
        power += game.min_cubes().power();
    }
    println!("Part 1: {}", sum_of_ids);
    println!("Part 2: {}", power);
}
