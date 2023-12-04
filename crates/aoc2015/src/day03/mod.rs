use common::{Problem, Solution};
/// \--- Day 3: Perfectly Spherical Houses in a Vacuum ---
/// ----------
///
/// Santa is delivering presents to an infinite two-dimensional grid of houses.
///
/// He begins by delivering a present to the house at his starting location, and
/// then an elf at the North Pole calls him via radio and tells him where to
/// move next. Moves are always exactly one house to the north (`^`), south
/// (`v`), east (`>`), or west (`<`). After each move, he delivers another
/// present to the house at his new location.
///
/// However, the elf back at the north pole has had a little too much eggnog,
/// and so his directions are a little off, and Santa ends up visiting some
/// houses more than once. How many houses receive *at least one present*?
///
/// For example:
///
/// * `>` delivers presents to `2` houses: one at the starting location, and one
///   to the east.
/// * `^>v<` delivers presents to `4` houses in a square, including twice to the
///   house at his starting/ending location.
/// * `^v^v^v^v^v` delivers a bunch of presents to some very lucky children at
///   only `2` houses.
///
/// \--- Part Two ---
/// ----------
///
/// The next year, to speed up the process, Santa creates a robot version of
/// himself, *Robo-Santa*, to deliver presents with him.
///
/// Santa and Robo-Santa start at the same location (delivering two presents to
/// the same starting house), then take turns moving based on instructions from
/// the elf, who is eggnoggedly reading from the same script as the previous
/// year.
///
/// This year, how many houses receive *at least one present*?
///
/// For example:
///
/// * `^v` delivers presents to `3` houses, because Santa goes north, and then
///   Robo-Santa goes south.
/// * `^>v<` now delivers presents to `3` houses, and Santa and Robo-Santa end
///   up back where they started.
/// * `^v^v^v^v^v` now delivers presents to `11` houses, with Santa going one
///   direction and Robo-Santa going the other.
pub struct Day03;

const GRID_SIZE: isize = 256;
const U_GRID_SIZE: usize = GRID_SIZE as usize;

const fn into_pos(x: i8, y: i8) -> usize {
    (((x as isize) + (GRID_SIZE / 2)) * GRID_SIZE + ((y as isize) + (GRID_SIZE / 2))) as usize
}

impl Problem for Day03 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        3u8
    }
    fn name(&self) -> &str {
        "Day 3: Perfectly Spherical Houses in a Vacuum"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        let (mut x, mut y) = (0i8, 0i8);
        let mut deliveries = [0u8; U_GRID_SIZE * U_GRID_SIZE];
        deliveries[into_pos(x, y)] += 1;
        for c in input.chars() {
            match c {
                '^' => y += 1,
                'v' => y -= 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => {}
            }
            deliveries[into_pos(x, y)] += 1;
        }
        Solution::USize(deliveries.iter().filter(|&d| *d > 0).count())
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let (mut x, mut y) = (0i8, 0i8);
        let (mut rx, mut ry) = (0i8, 0i8);
        let mut deliveries = [0u8; U_GRID_SIZE * U_GRID_SIZE];
        deliveries[into_pos(x, y)] += 2;
        let mut turn = false;
        for c in input.chars() {
            match turn {
                false => {
                    match c {
                        '^' => y += 1,
                        'v' => y -= 1,
                        '>' => x += 1,
                        '<' => x -= 1,
                        _ => {}
                    };
                    deliveries[into_pos(x, y)] += 1;
                    turn = true;
                }
                true => {
                    match c {
                        '^' => ry += 1,
                        'v' => ry -= 1,
                        '>' => rx += 1,
                        '<' => rx -= 1,
                        _ => {}
                    };
                    deliveries[into_pos(rx, ry)] += 1;
                    turn = false;
                }
            }
        }
        Solution::USize(deliveries.iter().filter(|&d| *d > 0).count())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let problem = Day03 {};
        assert_eq!(problem.solve_part1_with(">"), Solution::USize(2));
        assert_eq!(problem.solve_part1_with("^>v<"), Solution::USize(4));
        assert_eq!(problem.solve_part1_with("^v^v^v^v^v"), Solution::USize(2));
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day03 {};
        assert_eq!(problem.solve_part1(), Solution::USize(2565));
    }
    #[test]
    fn test_part2_example() {
        let problem = Day03 {};
        assert_eq!(problem.solve_part2_with("^v"), Solution::USize(3));
        assert_eq!(problem.solve_part2_with("^>v<"), Solution::USize(3));
        assert_eq!(problem.solve_part2_with("^v^v^v^v^v"), Solution::USize(11));
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day03 {};
        assert_eq!(problem.solve_part2(), Solution::USize(2639));
    }
}
