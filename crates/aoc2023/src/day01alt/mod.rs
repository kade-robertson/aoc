use common::{Problem, Solution};
/// \--- Day 1: Trebuchet?! ---
/// ----------
///
/// Something is wrong with global snow production, and you've been selected to
/// take a look. The Elves have even given you a map; on it, they've used stars
/// to mark the top fifty locations that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations,
/// you need to check all *fifty stars* by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each
/// day in the Advent calendar; the second puzzle is unlocked when you complete
/// the first. Each puzzle grants *one star*. Good luck!
///
/// You try to ask why they can't just use a [weather machine](/2015/day/1) ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a [trebuchet](https://en.wikipedia.org/wiki/Trebuchet) ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that their
/// calibration document (your puzzle input) has been *amended* by a very young
/// Elf who was apparently just excited to show off her art skills.
/// Consequently, the Elves are having trouble reading the values on the
/// document.
///
/// The newly-improved calibration document consists of lines of text; each line
/// originally contained a specific *calibration value* that the Elves now need
/// to recover. On each line, the calibration value can be found by combining
/// the *first digit* and the *last digit* (in that order) to form a single
/// *two-digit number*.
///
/// For example:
///
/// ```
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
///
/// In this example, the calibration values of these four lines are `12`, `38`,
/// `15`, and `77`. Adding these together produces `*142*`.
///
/// Consider your entire calibration document. *What is the sum of all of the
/// calibration values?*
///
/// \--- Part Two ---
/// ----------
///
/// Your calculation isn't quite right. It looks like some of the digits are
/// actually *spelled out with letters*: `one`, `two`, `three`, `four`, `five`,
/// `six`, `seven`, `eight`, and `nine` *also* count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and
/// last digit on each line. For example:
///
/// ```
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
/// ```
///
/// In this example, the calibration values are `29`, `83`, `13`, `24`, `42`,
/// `14`, and `76`. Adding these together produces `*281*`.
///
/// *What is the sum of all of the calibration values?*
pub struct Day01Alt;

static REPLACEMENTS3: [([char; 3], u32); 3] =
    [(['o', 'n', 'e'], 1), (['t', 'w', 'o'], 2), (['s', 'i', 'x'], 6)];

static REPLACEMENTS4: [([char; 4], u32); 3] =
    [(['f', 'o', 'u', 'r'], 4), (['f', 'i', 'v', 'e'], 5), (['n', 'i', 'n', 'e'], 9)];

static REPLACEMENTS5: [([char; 5], u32); 3] = [
    (['t', 'h', 'r', 'e', 'e'], 3),
    (['s', 'e', 'v', 'e', 'n'], 7),
    (['e', 'i', 'g', 'h', 't'], 8),
];

struct CharStack<const N: usize> {
    stack: [char; N],
    index: usize,
}

impl<const N: usize> CharStack<N> {
    fn new() -> Self {
        Self { stack: ['\0'; N], index: 0 }
    }

    fn push(&mut self, c: char) {
        if self.index < N {
            self.stack[self.index] = c;
            self.index += 1;
        } else {
            self.stack.rotate_left(1);
            self.stack[N - 1] = c;
        }
    }

    fn clear(&mut self) {
        self.stack = ['\0'; N];
        self.index = 0;
    }

    fn contains<const M: usize>(&self, value: &[char; M]) -> bool {
        for window in self.stack.windows(M) {
            if window == value {
                return true;
            }
        }

        false
    }
}

struct FirstLastDigits(u32, u32);

impl FirstLastDigits {
    fn new() -> Self {
        Self(0, 0)
    }

    fn add(&mut self, digit: u32) {
        if self.0 == 0 {
            self.0 = digit;
        }
        self.1 = digit;
    }

    fn value(&self) -> u32 {
        (self.0 * 10) + self.1
    }
}

impl Day01Alt {
    fn line_value(&self, line: &str, string_digits: bool) -> u32 {
        let mut digits = FirstLastDigits::new();
        let mut stack = CharStack::<5>::new();

        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.add(digit);
            } else if string_digits {
                stack.push(c);
                for (value, replacement) in REPLACEMENTS3.iter() {
                    if stack.contains(value) {
                        digits.add(*replacement);
                        stack.clear();
                        stack.push(c);
                        continue;
                    }
                }

                for (value, replacement) in REPLACEMENTS4.iter() {
                    if stack.contains(value) {
                        digits.add(*replacement);
                        stack.clear();
                        stack.push(c);
                        continue;
                    }
                }

                for (value, replacement) in REPLACEMENTS5.iter() {
                    if stack.contains(value) {
                        digits.add(*replacement);
                        stack.clear();
                        stack.push(c);
                        continue;
                    }
                }
            }
        }

        digits.value()
    }
}

impl Problem for Day01Alt {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        1u8
    }
    fn name(&self) -> &str {
        "Day 1: Trebuchet?! (Alt)"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        let mut total = 0;
        for line in input.lines() {
            total += self.line_value(line.trim(), false)
        }
        Solution::U32(total)
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let mut total = 0;
        for line in input.lines() {
            total += self.line_value(line.trim(), true)
        }
        Solution::U32(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let problem = Day01Alt {};
        assert_eq!(problem.solve_part1_with(input), Solution::U32(142));
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day01Alt {};
        assert_eq!(problem.solve_part1(), Solution::U32(52974));
    }
    #[test]
    fn test_part2_example() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";
        let problem = Day01Alt {};
        assert_eq!(problem.solve_part2_with(input), Solution::U32(281));
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day01Alt {};
        assert_eq!(problem.solve_part2(), Solution::U32(53340));
    }
}
