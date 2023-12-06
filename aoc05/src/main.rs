use util::input_lines;

#[derive(Debug, Clone)]
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
        v.sort_by(|a, b| a.src.cmp(&b.src));
        v
    }
    fn get_dest(&self, src: u64) -> u64 {
        assert!(self.contains(src));
        self.dest + src - self.src
    }
    fn contains(&self, src: u64) -> bool {
        src >= self.src && src < self.src + self.len
    }
    fn src_upper_limit(&self) -> u64 {
        let max = self.src + self.len - 1;
        assert!(self.contains(max));
        assert!(!self.contains(max + 1));
        max
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
    fn get_first_src_range(&self, src: u64, max: u64) -> (u64, u64);
    fn get_src_ranges(&self, min: u64, max: u64) -> Vec<(u64, u64)> {
        let mut v = Vec::new();
        let mut x = min;
        while x < max {
            v.push(self.get_first_src_range(x, max));
            x = v.last().unwrap().1 + 1;
        }
        v
    }
    fn get_dest_ranges(&self, src_min: u64, src_max: u64) -> Vec<(u64, u64)>;
    fn get_many_dest_ranges(&self, ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut v = Vec::new();
        for (min, max) in ranges {
            v.extend(self.get_dest_ranges(*min, *max));
        }
        v
    }
}

impl GetDest for Vec<Map> {
    fn get_dest(&self, src: u64) -> u64 {
        for m in self {
            if m.contains(src) {
                // println!("{} --> {}", src, m.dest + src - m.src);
                return m.get_dest(src);
            }
        }
        // println!("{} --> {}", src, src);
        src
    }

    fn get_first_src_range(&self, src: u64, max: u64) -> (u64, u64) {
        for m in self {
            if m.contains(src) {
                return (src, std::cmp::min(max, m.src_upper_limit()));
            }
            if m.src > src {
                return (src, std::cmp::min(max, m.src - 1));
            }
        }
        (src, max)
    }

    fn get_dest_ranges(&self, src_min: u64, src_max: u64) -> Vec<(u64, u64)> {
        let mut v = Vec::new();
        for (min, max) in self.get_src_ranges(src_min, src_max) {
            v.push((self.get_dest(min), self.get_dest(max)));
        }
        v
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
    fn get_best(&self, min: u64, max: u64) -> u64 {
        let locs = self.humidity_to_location.get_many_dest_ranges(
            &self.temp_to_humidity.get_many_dest_ranges(
                &self.light_to_temp.get_many_dest_ranges(
                    &self.water_to_light.get_many_dest_ranges(
                        &self.fertilizer_to_water.get_many_dest_ranges(
                            &self
                                .soil_to_fertilizer
                                .get_many_dest_ranges(&self.seed_to_soil.get_dest_ranges(min, max)),
                        ),
                    ),
                ),
            ),
        );
        let mut min = locs[0].0;
        for loc in locs {
            min = std::cmp::min(min, loc.0);
        }
        min
    }
}

fn main() {
    let i = Input::parse(&input_lines());
    let mut min_loc = i.get_loc(i.seeds[0]);
    for seed in &i.seeds {
        min_loc = std::cmp::min(min_loc, i.get_loc(*seed));
    }
    println!("Part 1: {}", min_loc);
    let mut min_loc = i.get_loc(i.seeds[0]);
    let mut iter = i.seeds.iter();
    while let Some(start) = iter.next() {
        min_loc = std::cmp::min(min_loc, i.get_best(*start, *start + *iter.next().unwrap()));
    }
    println!("Part 2: {}", min_loc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map {
            dest: 10,
            src: 100,
            len: 10,
        };
        assert!(!map.contains(99));
        assert!(map.contains(100));
        assert!(map.contains(109));
        assert!(!map.contains(110));
        assert_eq!(map.src_upper_limit(), 109);
        assert_eq!(map.get_dest(100), 10);
        assert_eq!(map.get_dest(109), 19);
    }

    #[test]
    fn test_get_dest() {
        let v = vec![Map {
            dest: 10,
            src: 100,
            len: 10,
        }];
        assert_eq!(v.get_dest(99), 99);
        assert_eq!(v.get_dest(100), 10);
        assert_eq!(v.get_dest(109), 19);
        assert_eq!(v.get_dest(110), 110);

        assert_eq!(v.get_first_src_range(10, 1000), (10, 99));
        assert_eq!(v.get_first_src_range(103, 1000), (103, 109));
        assert_eq!(v.get_first_src_range(120, 1000), (120, 1000));

        assert_eq!(v.get_src_ranges(10, 20), [(10, 20)]);
        assert_eq!(v.get_src_ranges(101, 103), [(101, 103)]);
        assert_eq!(v.get_src_ranges(110, 120), [(110, 120)]);
        assert_eq!(v.get_src_ranges(10, 105), [(10, 99), (100, 105)]);
        assert_eq!(v.get_src_ranges(105, 1000), [(105, 109), (110, 1000)]);
        assert_eq!(
            v.get_src_ranges(10, 1000),
            [(10, 99), (100, 109), (110, 1000)]
        );

        assert_eq!(v.get_dest_ranges(10, 20), [(10, 20)]);
        assert_eq!(v.get_dest_ranges(101, 103), [(11, 13)]);
        assert_eq!(v.get_dest_ranges(110, 120), [(110, 120)]);
        assert_eq!(v.get_dest_ranges(10, 105), [(10, 99), (10, 15)]);
        assert_eq!(v.get_dest_ranges(105, 1000), [(15, 19), (110, 1000)]);
        assert_eq!(
            v.get_dest_ranges(10, 1000),
            [(10, 99), (10, 19), (110, 1000)]
        );
    }
}
