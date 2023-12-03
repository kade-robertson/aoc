use common::{Problem, Solution};
/// \--- Day 3: Gear Ratios ---
/// ----------
///
/// You and the Elf eventually reach a [gondola lift](https://en.wikipedia.org/wiki/Gondola_lift) station; he says the gondola lift will take you up to the *water source*, but this is as far as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem:
/// they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of
/// surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working
/// right now; it'll still be a while before I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the
/// engine, but nobody can figure out which one. If you can *add up all the part
/// numbers* in the engine schematic, it should be easy to work out which part
/// is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation
/// of the engine. There are lots of numbers and symbols you don't really
/// understand, but apparently *any number adjacent to a symbol*, even
/// diagonally, is a "part number" and should be included in your sum. (Periods
/// (`.`) do not count as a symbol.)
///
/// Here is an example engine schematic:
///
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
///
/// In this schematic, two numbers are *not* part numbers because they are not
/// adjacent to a symbol: `114` (top right) and `58` (middle right). Every other
/// number is adjacent to a symbol and so *is* a part number; their sum is
/// `*4361*`.
///
/// Of course, the actual engine schematic is much larger. *What is the sum of
/// all of the part numbers in the engine schematic?*
pub struct Day03;

struct GondolaEngine {
    number_spans: Vec<(usize, usize, usize, u32)>,
    symbol_positions: Vec<(usize, usize, char)>,
}

impl Day03 {
    fn parse_engine(&self, input: &str) -> GondolaEngine {
        let mut number_spans: Vec<(usize, usize, usize, u32)> = Vec::with_capacity(input.len());
        let mut symbol_positions: Vec<(usize, usize, char)> = Vec::with_capacity(input.len());

        for (idx, row) in input.lines().enumerate().map(|(i, l)| (i, l.trim())) {
            let mut span_in_use = false;
            let mut current_span = (idx, 0, 0, 0);
            for (idy, ch) in row.chars().enumerate() {
                if let Some(d) = ch.to_digit(10) {
                    if !span_in_use {
                        current_span.1 = idy;
                    }
                    span_in_use = true;
                    current_span.2 = idy;
                    current_span.3 = (current_span.3 * 10) + d;
                } else {
                    if span_in_use {
                        number_spans.push(current_span);
                        current_span = (idx, 0, 0, 0);
                        span_in_use = false;
                    }

                    if ch != '.' {
                        symbol_positions.push((idx, idy, ch));
                    }
                }
            }
            if span_in_use {
                number_spans.push(current_span);
            }
        }

        GondolaEngine { number_spans, symbol_positions }
    }
}

impl Problem for Day03 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        3u8
    }
    fn name(&self) -> &str {
        "Day 3: Gear Ratios"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        let engine = self.parse_engine(input);

        let mut total = 0;

        for (nx, ny, nz, value) in engine.number_spans {
            for (sx, sy, _) in &engine.symbol_positions {
                let within_row =
                    *sx as isize >= (nx as isize - 1) && *sx as isize <= (nx as isize + 1);
                let within_col =
                    *sy as isize >= (ny as isize - 1) && *sy as isize <= (nz as isize + 1);
                if within_row && within_col {
                    total += value;
                }
            }
        }

        Solution::U32(total)
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let engine = self.parse_engine(input);

        let mut total = 0;

        for (sx, sy, ch) in engine.symbol_positions {
            if ch != '*' {
                continue;
            }
            let mut num_count = 0;
            let mut gear_ratio = 1;
            for (nx, ny, nz, value) in &engine.number_spans {
                let within_row =
                    sx as isize >= (*nx as isize - 1) && sx as isize <= (*nx as isize + 1);
                let within_col =
                    sy as isize >= (*ny as isize - 1) && sy as isize <= (*nz as isize + 1);
                if within_row && within_col {
                    gear_ratio *= value;
                    num_count += 1;
                }
            }

            if num_count >= 2 {
                total += gear_ratio;
            }
        }

        Solution::U32(total)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let problem = Day03 {};
        assert_eq!(problem.solve_part1_with(input), Solution::U32(4361));
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day03 {};
        assert_eq!(problem.solve_part1(), Solution::U32(527144));
    }
    #[test]
    fn test_part2_example() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let problem = Day03 {};
        assert_eq!(problem.solve_part2_with(input), Solution::U32(467835));
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day03 {};
        assert_eq!(problem.solve_part2(), Solution::U32(81463996));
    }
}
