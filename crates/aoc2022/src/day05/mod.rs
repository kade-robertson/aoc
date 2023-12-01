use std::collections::VecDeque;

use anyhow::Context;
use common::{Problem, Solution};
/// \--- Day 5: Supply Stacks ---
/// ----------
///
/// The expedition can depart as soon as the final supplies have been unloaded
/// from the ships. Supplies are stored in stacks of marked *crates*, but
/// because the needed supplies are buried under many other crates, the crates
/// need to be rearranged.
///
/// The ship has a *giant cargo crane* capable of moving crates between stacks.
/// To ensure none of the crates get crushed or fall over, the crane operator
/// will rearrange them in a series of carefully-planned steps. After the crates
/// are rearranged, the desired crates will be at the top of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate
/// procedure, but they forgot to ask her *which* crate will end up where, and
/// they want to be ready to unload them as soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates *and* the
/// rearrangement procedure (your puzzle input). For example:
///
/// ```
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
/// ```
///
/// In this example, there are three stacks of crates. Stack 1 contains two
/// crates: crate `Z` is on the bottom, and crate `N` is on top. Stack 2
/// contains three crates; from bottom to top, they are crates `M`, `C`, and
/// `D`. Finally, stack 3 contains a single crate, `P`.
///
/// Then, the rearrangement procedure is given. In each step of the procedure, a
/// quantity of crates is moved from one stack to a different stack. In the
/// first step of the above rearrangement procedure, one crate is moved from
/// stack 2 to stack 1, resulting in this configuration:
///
/// ```
/// [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
///
/// In the second step, three crates are moved from stack 1 to stack 3. Crates
/// are moved *one at a time*, so the first crate to be moved (`D`) ends up
/// below the second and third crates:
///
/// ```
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
/// ```
///
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates
/// are moved *one at a time*, crate `C` ends up below crate `M`:
///
/// ```
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
/// ```
///
/// Finally, one crate is moved from stack 1 to stack 2:
///
/// ```
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
/// ```
///
/// The Elves just need to know *which crate will end up on top of each stack*;
/// in this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z`
/// in stack 3, so you should combine these together and give the Elves the
/// message `*CMZ*`.
///
/// *After the rearrangement procedure completes, what crate ends up on top of
/// each stack?*
///
/// \--- Part Two ---
/// ----------
///
/// As you watch the crane operator expertly rearrange the crates, you notice
/// the process isn't following your prediction.
///
/// Some mud was covering the writing on the side of the crane, and you quickly
/// wipe it away. The crane isn't a CrateMover 9000 - it's a *CrateMover 9001*.
///
/// The CrateMover 9001 is notable for many new and exciting features: air
/// conditioning, leather seats, an extra cup holder, and *the ability to pick
/// up and move multiple crates at once*.
///
/// Again considering the example above, the crates begin in the same
/// configuration:
///
/// ```
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
///
/// Moving a single crate from stack 2 to stack 1 behaves the same as before:
///
/// ```
/// [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
///
/// However, the action of moving three crates from stack 1 to stack 3 means
/// that those three moved crates *stay in the same order*, resulting in this
/// new configuration:
///
/// ```
///         [D]
///         [N]
///     [C] [Z]
///     [M] [P]
///  1   2   3
/// ```
///
/// Next, as both crates are moved from stack 2 to stack 1, they *retain their
/// order* as well:
///
/// ```
///         [D]
///         [N]
/// [C]     [Z]
/// [M]     [P]
///  1   2   3
/// ```
///
/// Finally, a single crate is still moved from stack 1 to stack 2, but now it's
/// crate `C` that gets moved:
///
/// ```
///         [D]
///         [N]
///         [Z]
/// [M] [C] [P]
///  1   2   3
/// ```
///
/// In this example, the CrateMover 9001 has put the crates in a totally
/// different order: `*MCD*`.
///
/// Before the rearrangement process finishes, update your simulation so that
/// the Elves know where they should stand to be ready to unload the final
/// supplies. *After the rearrangement procedure completes, what crate ends up
/// on top of each stack?*
pub struct Day05;

#[derive(PartialEq)]
enum ParseState {
    LookingForCrates,
    SkipBlankLine,
    ReadingMoves,
}

#[derive(Debug)]
struct CraneGame {
    crate_columns: Vec<VecDeque<char>>,
}

impl CraneGame {
    fn new() -> Self {
        Self { crate_columns: vec![VecDeque::new(); 10] }
    }

    fn add_crate(&mut self, column: usize, crate_name: char) -> anyhow::Result<()> {
        self.crate_columns.get_mut(column).context("Column missing")?.push_front(crate_name);

        Ok(())
    }

    fn process_instruction(
        &mut self,
        amount: usize,
        from_col: usize,
        to_col: usize,
        pick_multiple: bool,
    ) -> anyhow::Result<()> {
        let column_from =
            self.crate_columns.get_mut(from_col - 1).context("From column does not exist")?;

        let crates_to_move: VecDeque<char> = if pick_multiple {
            column_from.split_off(column_from.len() - amount)
        } else {
            column_from.drain((column_from.len() - amount)..).rev().collect()
        };

        let column_to =
            self.crate_columns.get_mut(to_col - 1).context("To column does not exist")?;

        column_to.extend(crates_to_move);

        Ok(())
    }

    fn get_top_crates(&self) -> anyhow::Result<String> {
        let mut result = String::from("");

        self.crate_columns
            .iter()
            .filter(|col| !col.is_empty())
            .for_each(|col| result.push(*col.back().expect("No item in column")));

        Ok(result)
    }
}

impl Day05 {
    fn parse(&self, data: &str, pick_multiple: bool) -> anyhow::Result<CraneGame> {
        let mut state = ParseState::LookingForCrates;
        let mut crane_game = CraneGame::new();

        for line in data.lines() {
            let mut column = 0;
            let mut idx = 0;
            let line_chars = line.chars().collect::<Vec<char>>();

            if state == ParseState::LookingForCrates && line.starts_with(" 1") {
                state = ParseState::SkipBlankLine;
                continue;
            }

            if state == ParseState::SkipBlankLine {
                state = ParseState::ReadingMoves;
                continue;
            }

            if state == ParseState::LookingForCrates {
                while let Some(ch) = line_chars.get(idx) {
                    if ch == &'[' {
                        let new_crate = line_chars.get(idx + 1).expect("Missing crate identifier");
                        crane_game.add_crate(column, *new_crate)?;
                    }
                    column += 1;
                    idx += 4;
                }
            } else if state == ParseState::ReadingMoves {
                let words = line.split(' ').collect::<Vec<_>>();
                crane_game.process_instruction(
                    words.get(1).context("Could not find amount")?.parse()?,
                    words.get(3).context("Could not find from")?.parse()?,
                    words.get(5).context("Could not find to")?.parse()?,
                    pick_multiple,
                )?;
            }
        }

        Ok(crane_game)
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
        "Day 5: Supply Stacks"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        let crane_game = self.parse(input, false).unwrap();
        Solution::Str(crane_game.get_top_crates().unwrap_or_else(|_| "failed".into()))
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let crane_game = self.parse(input, true).unwrap();
        Solution::Str(crane_game.get_top_crates().unwrap_or_else(|_| "failed".into()))
    }
}
