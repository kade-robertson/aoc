use common::{Problem, Solution};
/// \--- Day 1: Not Quite Lisp ---
/// ----------
///
/// Santa was hoping for a white Christmas, but his weather machine's "snow"
/// function is powered by stars, and he's fresh out! To save Christmas, he
/// needs you to collect *fifty stars* by December 25th.
///
/// Collect stars by helping Santa solve puzzles. Two puzzles will be made
/// available on each day in the Advent calendar; the second puzzle is unlocked
/// when you complete the first. Each puzzle grants *one star*. Good luck!
///
/// Here's an easy puzzle to warm you up.
///
/// Santa is trying to deliver presents in a large apartment building, but he
/// can't find the right floor - the directions he got are a little confusing.
/// He starts on the ground floor (floor `0`) and then follows the instructions
/// one character at a time.
///
/// An opening parenthesis, `(`, means he should go up one floor, and a closing
/// parenthesis, `)`, means he should go down one floor.
///
/// The apartment building is very tall, and the basement is very deep; he will
/// never find the top or bottom floors.
///
/// For example:
///
/// * `(())` and `()()` both result in floor `0`.
/// * `(((` and `(()(()(` both result in floor `3`.
/// * `))(((((` also results in floor `3`.
/// * `())` and `))(` both result in floor `-1` (the first basement level).
/// * `)))` and `)())())` both result in floor `-3`.
///
/// To *what floor* do the instructions take Santa?
///
/// \--- Part Two ---
/// ----------
///
/// Now, given the same instructions, find the *position* of the first character
/// that causes him to enter the basement (floor `-1`). The first character in
/// the instructions has position `1`, the second character has position `2`,
/// and so on.
///
/// For example:
///
/// * `)` causes him to enter the basement at character position `1`.
/// * `()())` causes him to enter the basement at character position `5`.
///
/// What is the *position* of the character that causes Santa to first enter the
/// basement?
pub struct Day01;
impl Problem for Day01 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        1u8
    }
    fn name(&self) -> &str {
        "Day 1: Not Quite Lisp"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        input
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            })
            .into()
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let mut floor = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor == -1 {
                return (i + 1).into();
            }
        }
        Solution::Error
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let problem = Day01 {};
        assert_eq!(problem.solve_part1_with("(())"), 0.into());
        assert_eq!(problem.solve_part1_with("()()"), 0.into());
        assert_eq!(problem.solve_part1_with("((("), 3.into());
        assert_eq!(problem.solve_part1_with("(()(()("), 3.into());
        assert_eq!(problem.solve_part1_with("))((((("), 3.into());
        assert_eq!(problem.solve_part1_with("())"), (-1).into());
        assert_eq!(problem.solve_part1_with("))("), (-1).into());
        assert_eq!(problem.solve_part1_with(")))"), (-3).into());
        assert_eq!(problem.solve_part1_with(")())())"), (-3).into());
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day01 {};
        assert_eq!(problem.solve_part1(), 232.into());
    }
    #[test]
    fn test_part2_example() {
        let problem = Day01 {};
        assert_eq!(problem.solve_part2_with(")"), Solution::USize(1));
        assert_eq!(problem.solve_part2_with("()())"), Solution::USize(5));
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day01 {};
        assert_eq!(problem.solve_part2(), Solution::USize(1783));
    }
}
