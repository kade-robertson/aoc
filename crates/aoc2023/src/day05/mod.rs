use std::str::FromStr;

use common::{Problem, Solution};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
/// \--- Day 5: If You Give A Seed A Fertilizer ---
/// ----------
///
/// You take the boat and find the gardener right where you were told he would
/// be: managing a giant "garden" that looks more to you like a farm.
///
/// "A water source? Island Island *is* the water source!" You point out that
/// Snow Island isn't receiving any water.
///
/// "Oh, we had to stop the water because we *ran out of sand* to [filter](https://en.wikipedia.org/wiki/Sand_filter) it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.
///
/// "I've been so busy making sure everyone here has food that I completely
/// forgot to check why we stopped getting more sand! There's a ferry leaving
/// soon that is headed over in that direction - it's much faster than your
/// boat. Could you please go check it out?"
///
/// You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our *food production problem*. The latest Island Island [Almanac](https://en.wikipedia.org/wiki/Almanac) just arrived and we're having trouble making sense of it."
///
/// The almanac (your puzzle input) lists all of the seeds that need to be
/// planted. It also lists what type of soil to use with each kind of seed, what
/// type of fertilizer to use with each kind of soil, what type of water to use
/// with each kind of fertilizer, and so on. Every type of seed, soil,
/// fertilizer and so on is identified with a number, but numbers are reused by
/// each category - that is, soil `123` and fertilizer `123` aren't necessarily
/// related to each other.
///
/// For example:
///
/// ```
/// seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
///
/// soil-to-fertilizer map:
/// 0 15 37
/// 37 52 2
/// 39 0 15
///
/// fertilizer-to-water map:
/// 49 53 8
/// 0 11 42
/// 42 0 7
/// 57 7 4
///
/// water-to-light map:
/// 88 18 7
/// 18 25 70
///
/// light-to-temperature map:
/// 45 77 23
/// 81 45 19
/// 68 64 13
///
/// temperature-to-humidity map:
/// 0 69 1
/// 1 0 69
///
/// humidity-to-location map:
/// 60 56 37
/// 56 93 4
/// ```
///
/// The almanac starts by listing which seeds need to be planted: seeds `79`,
/// `14`, `55`, and `13`.
///
/// The rest of the almanac contains a list of *maps* which describe how to
/// convert numbers from a *source category* into numbers in a *destination
/// category*. That is, the section that starts with `seed-to-soil map:`
/// describes how to convert a *seed number* (the source) to a *soil number*
/// (the destination). This lets the gardener and his team know which soil to
/// use with which seeds, which water to use with which fertilizer, and so on.
///
/// Rather than list every source number and its corresponding destination
/// number one by one, the maps describe entire *ranges* of numbers that can be
/// converted. Each line within a map contains three numbers: the *destination
/// range start*, the *source range start*, and the *range length*.
///
/// Consider again the example `seed-to-soil map`:
///
/// ```
/// 50 98 2
/// 52 50 48
/// ```
///
/// The first line has a *destination range start* of `50`, a *source range
/// start* of `98`, and a *range length* of `2`. This line means that the source
/// range starts at `98` and contains two values: `98` and `99`. The destination
/// range is the same length, but it starts at `50`, so its two values are `50`
/// and `51`. With this information, you know that seed number `98` corresponds
/// to soil number `50` and that seed number `99` corresponds to soil number
/// `51`.
///
/// The second line means that the source range starts at `50` and contains `48`
/// values: `50`, `51`, ..., `96`, `97`. This corresponds to a destination range
/// starting at `52` and also containing `48` values: `52`, `53`, ..., `98`,
/// `99`. So, seed number `53` corresponds to soil number `55`.
///
/// Any source numbers that *aren't mapped* correspond to the *same* destination
/// number. So, seed number `10` corresponds to soil number `10`.
///
/// So, the entire list of seed numbers and their corresponding soil numbers
/// looks like this:
///
/// ```
/// seed  soil
/// 0     0
/// 1     1
/// ...   ...
/// 48    48
/// 49    49
/// 50    52
/// 51    53
/// ...   ...
/// 96    98
/// 97    99
/// 98    50
/// 99    51
/// ```
///
/// With this map, you can look up the soil number required for each initial
/// seed number:
///
/// * Seed number `79` corresponds to soil number `81`.
/// * Seed number `14` corresponds to soil number `14`.
/// * Seed number `55` corresponds to soil number `57`.
/// * Seed number `13` corresponds to soil number `13`.
///
/// The gardener and his team want to get started as soon as possible, so they'd
/// like to know the closest location that needs a seed. Using these maps, find
/// *the lowest location number that corresponds to any of the initial seeds*.
/// To do this, you'll need to convert each seed number through other categories
/// until you can find its corresponding *location number*. In this example, the
/// corresponding types are:
///
/// * Seed `79`, soil `81`, fertilizer `81`, water `81`, light `74`, temperature
///   `78`, humidity `78`, *location `82`*.
/// * Seed `14`, soil `14`, fertilizer `53`, water `49`, light `42`, temperature
///   `42`, humidity `43`, *location `43`*.
/// * Seed `55`, soil `57`, fertilizer `57`, water `53`, light `46`, temperature
///   `82`, humidity `82`, *location `86`*.
/// * Seed `13`, soil `13`, fertilizer `52`, water `41`, light `34`, temperature
///   `34`, humidity `35`, *location `35`*.
///
/// So, the lowest location number in this example is `*35*`.
///
/// *What is the lowest location number that corresponds to any of the initial
/// seed numbers?*
pub struct Day05;

#[derive(Debug)]
struct Range {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

impl Range {
    fn contains(&self, src: u64) -> bool {
        src >= self.src_start && src < self.src_start + self.length
    }

    fn map(&self, src: u64) -> Option<u64> {
        if self.contains(src) {
            Some(self.dest_start + (src - self.src_start))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<(u64, u64)>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

macro_rules! read_until_empty {
    ($lines:ident, $name:ident) => {
        let mut $name = Vec::new();
        while let Some(line) = $lines.next() {
            if line.trim().is_empty() {
                break;
            }

            let mut parts = line.split(' ').filter_map(|s| s.parse::<u64>().ok()).take(3);
            if let (Some(dest_start), Some(src_start), Some(length)) =
                (parts.next(), parts.next(), parts.next())
            {
                $name.push(Range { dest_start, src_start, length });
            }
        }
        $lines.next(); // blank
    };
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let seeds = lines
            .next()
            .ok_or(())?
            .split_once(": ")
            .ok_or(())?
            .1
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect::<Vec<_>>();
        let seed_ranges = seeds.chunks(2).map(|c| (c[0], c[0] + c[1] - 1)).collect::<Vec<_>>();

        lines.next(); // blank
        lines.next(); // title

        read_until_empty!(lines, seed_to_soil);
        read_until_empty!(lines, soil_to_fertilizer);
        read_until_empty!(lines, fertilizer_to_water);
        read_until_empty!(lines, water_to_light);
        read_until_empty!(lines, light_to_temperature);
        read_until_empty!(lines, temperature_to_humidity);
        read_until_empty!(lines, humidity_to_location);

        Ok(Almanac {
            seeds,
            seed_ranges,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Almanac {
    fn map_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.iter().find_map(|r| r.map(seed)).unwrap_or(seed);
        let fertilizer = self.soil_to_fertilizer.iter().find_map(|r| r.map(soil)).unwrap_or(soil);
        let water =
            self.fertilizer_to_water.iter().find_map(|r| r.map(fertilizer)).unwrap_or(fertilizer);
        let light = self.water_to_light.iter().find_map(|r| r.map(water)).unwrap_or(water);
        let temperature =
            self.light_to_temperature.iter().find_map(|r| r.map(light)).unwrap_or(light);
        let humidity = self
            .temperature_to_humidity
            .iter()
            .find_map(|r| r.map(temperature))
            .unwrap_or(temperature);
        self.humidity_to_location.iter().find_map(|r| r.map(humidity)).unwrap_or(humidity)
    }

    fn lowest_location(&self) -> u64 {
        self.seeds.iter().map(|&s| self.map_seed(s)).min().unwrap_or(0)
    }

    fn lowest_location_range(&self) -> u64 {
        self.seed_ranges
            .par_iter()
            .map(|&(s, l)| {
                (s..=l).into_par_iter().map(|s| self.map_seed(s)).min().unwrap_or(u64::MAX)
            })
            .min()
            .unwrap_or(0)
    }
}

impl Problem for Day05 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        5u8
    }
    fn name(&self) -> &str {
        "Day 5: If You Give A Seed A Fertilizer"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        Solution::U64(input.parse::<Almanac>().map(|a| a.lowest_location()).unwrap_or(0))
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        Solution::U64(input.parse::<Almanac>().map(|a| a.lowest_location_range()).unwrap_or(0))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        let problem = Day05 {};
        assert_eq!(problem.solve_part1_with(input), Solution::U64(35))
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day05 {};
        assert_eq!(problem.solve_part1(), Solution::U64(51752125));
    }
    #[test]
    fn test_part2_example() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";
        let problem = Day05 {};
        assert_eq!(problem.solve_part2_with(input), Solution::U64(46))
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day05 {};
        assert_eq!(problem.solve_part2(), Solution::Todo);
    }
}
