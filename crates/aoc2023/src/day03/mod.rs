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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Part {
    None,
    Single(u32),
    Double(u32, u32),
    Triple(u32, u32, u32),
    Quadruple(u32, u32, u32, u32),
}

impl Part {
    fn total(&self) -> u32 {
        match self {
            Part::None => 0,
            Part::Single(a) => *a,
            Part::Double(a, b) => *a + *b,
            Part::Triple(a, b, c) => *a + *b + *c,
            Part::Quadruple(a, b, c, d) => *a + *b + *c + *d,
        }
    }

    fn ratio(&self) -> u32 {
        match self {
            Part::Double(a, b) => *a * *b,
            _ => 0,
        }
    }

    fn add(&mut self, value: u32) {
        *self = match self {
            Part::None => Part::Single(value),
            Part::Single(a) => Part::Double(*a, value),
            Part::Double(a, b) => Part::Triple(*a, *b, value),
            Part::Triple(a, b, c) => Part::Quadruple(*a, *b, *c, value),
            Part::Quadruple(_, _, _, _) => {
                panic!("Too many numbers in one spot");
            }
        }
    }
}

struct PartSpan(isize, isize, isize, u32);

// Fortunately, our input and enum size are small enough we can fit this entire
// map on the stack so we can avoid heap allocations.
struct GondolaEngine {
    parts_map: [[Part; 150]; 150],
}

impl GondolaEngine {
    fn new() -> Self {
        Self { parts_map: [[Part::None; 150]; 150] }
    }

    fn add_part(&mut self, span: &PartSpan) {
        for x in span.0 - 1..=span.0 + 1 {
            for y in span.1 - 1..=span.2 + 1 {
                if x < 0 || y < 0 || x >= 150 || y >= 150 {
                    continue;
                }
                self.parts_map[x as usize][y as usize].add(span.3)
            }
        }
    }

    fn parse(&mut self, input: &str) {
        for (idx, row) in input.lines().enumerate().map(|(i, l)| (i, l.trim())) {
            let mut span_in_use = false;
            let mut current_span = PartSpan(idx as isize, 0, 0, 0);
            for (idy, ch) in row.chars().enumerate() {
                if let Some(d) = ch.to_digit(10) {
                    if !span_in_use {
                        current_span.1 = idy as isize;
                    }
                    span_in_use = true;
                    current_span.2 = idy as isize;
                    current_span.3 = (current_span.3 * 10) + d;
                } else if span_in_use {
                    self.add_part(&current_span);
                    current_span = PartSpan(idx as isize, 0, 0, 0);
                    span_in_use = false;
                }
            }

            if span_in_use {
                self.add_part(&current_span);
            }
        }
    }

    fn part_at(&self, x: usize, y: usize) -> Part {
        self.parts_map[x][y]
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
        let mut engine = GondolaEngine::new();
        engine.parse(input);

        let mut total = 0;
        for (idx, row) in input.lines().enumerate().map(|(i, l)| (i, l.trim())) {
            for (idy, ch) in row.chars().enumerate() {
                if !ch.is_ascii_digit() && ch != '.' {
                    total += engine.part_at(idx, idy).total();
                }
            }
        }

        Solution::U32(total)
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let mut engine = GondolaEngine::new();
        engine.parse(input);

        let mut total = 0;

        for (idx, row) in input.lines().enumerate().map(|(i, l)| (i, l.trim())) {
            for (idy, ch) in row.chars().enumerate() {
                if ch != '*' {
                    continue;
                }

                total += engine.part_at(idx, idy).ratio();
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
