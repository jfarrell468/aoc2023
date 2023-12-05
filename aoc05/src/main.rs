use util::input_lines;

#[derive(Debug)]
struct Map {
    dest: u64,
    src: u64,
    len: u64,
}

impl Map {
    fn parse(lines: &[String]) -> Vec<Map> {
        let mut v = Vec::new();
        for line in lines {
            if line.is_empty() {
                break;
            }
            let mut iter = line.split_whitespace();
            v.push(Map {
                dest: iter.next().unwrap().parse().unwrap(),
                src: iter.next().unwrap().parse().unwrap(),
                len: iter.next().unwrap().parse().unwrap(),
            })
        }
        v
    }
}

#[derive(Debug, Default)]
struct Input {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temp: Vec<Map>,
    temp_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

trait GetDest {
    fn get_dest(&self, src: u64) -> u64;
}

impl GetDest for Vec<Map> {
    fn get_dest(&self, src: u64) -> u64 {
        for m in self {
            if src >= m.src && src < m.src + m.len {
                println!("{} --> {}", src, m.dest + src - m.src);
                return m.dest + src - m.src;
            }
        }
        println!("{} --> {}", src, src);
        src
    }
}

impl Input {
    fn parse(lines: &[String]) -> Input {
        let seeds = lines[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        let mut start: usize = 3;
        let seed_to_soil = Map::parse(&lines[start..]);
        start += seed_to_soil.len() + 2;
        let soil_to_fertilizer = Map::parse(&lines[start..]);
        start += soil_to_fertilizer.len() + 2;
        let fertilizer_to_water = Map::parse(&lines[start..]);
        start += fertilizer_to_water.len() + 2;
        let water_to_light = Map::parse(&lines[start..]);
        start += water_to_light.len() + 2;
        let light_to_temp = Map::parse(&lines[start..]);
        start += light_to_temp.len() + 2;
        let temp_to_humidity = Map::parse(&lines[start..]);
        start += temp_to_humidity.len() + 2;
        let humidity_to_location = Map::parse(&lines[start..]);
        Input {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_location,
        }
    }
    fn get_loc(&self, src: u64) -> u64 {
        self.humidity_to_location.get_dest(
            self.temp_to_humidity.get_dest(
                self.light_to_temp.get_dest(
                    self.water_to_light.get_dest(
                        self.fertilizer_to_water.get_dest(
                            self.soil_to_fertilizer
                                .get_dest(self.seed_to_soil.get_dest(src)),
                        ),
                    ),
                ),
            ),
        )
    }
}

fn main() {
    let i = Input::parse(&input_lines());
    let mut min_loc = i.get_loc(i.seeds[0]);
    for seed in &i.seeds {
        min_loc = std::cmp::min(min_loc, i.get_loc(*seed));
    }
    println!("Part 1: {}", min_loc);
}
