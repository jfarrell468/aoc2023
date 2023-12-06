use util::input_lines;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_beat_record(&self) -> u64 {
        let mut ways = 0;
        for press in 0..=self.time {
            if (self.time - press) * press > self.distance {
                ways += 1;
            }
        }
        ways
    }
}

fn main() {
    let lines = input_lines();
    let races = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .zip(
            lines[1]
                .split_once(":")
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap()),
        )
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<_>>();
    // println!("{:?}", races);
    let mut product_of_ways = 1;
    for race in &races {
        product_of_ways *= race.ways_to_beat_record();
        // println!("{:?} {}", race, race.ways_to_beat_record());
    }
    println!("Part 1: {}", product_of_ways);

    let race = Race {
        time: lines[0]
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap(),
        distance: lines[1]
            .split_once(":")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .parse()
            .unwrap(),
    };
    // println!("{:?}", race);
    println!("Part 2: {}", race.ways_to_beat_record());
}
