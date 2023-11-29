use common::{Problem, Solution};
/// \--- Day 3: Rucksack Reorganization ---
/// ----------
///
/// One Elf has the important job of loading all of the [rucksacks](https://en.wikipedia.org/wiki/Rucksack) with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
///
/// Each rucksack has two large *compartments*. All items of a given type are
/// meant to go into exactly one of the two compartments. The Elf that did the
/// packing failed to follow this rule for exactly one item type per rucksack.
///
/// The Elves have made a list of all of the items currently in each rucksack
/// (your puzzle input), but they need your help finding the errors. Every item
/// type is identified by a single lowercase or uppercase letter (that is, `a`
/// and `A` refer to different types of items).
///
/// The list of items for each rucksack is given as characters all on a single
/// line. A given rucksack always has the same number of items in each of its
/// two compartments, so the first half of the characters represent items in the
/// first compartment, while the second half of the characters represent items
/// in the second compartment.
///
/// For example, suppose you have the following list of contents from six
/// rucksacks:
///
/// ```
/// vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw
/// ```
///
/// * The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which
///   means its first compartment contains the items `vJrwpWtwJgWr`, while the
///   second compartment contains the items `hcsFMMfFFhFp`. The only item type
///   that appears in both compartments is lowercase `*p*`.
/// * The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and
///   `rsFMfFZSrLrFZsSL`. The only item type that appears in both compartments
///   is uppercase `*L*`.
/// * The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the
///   only common item type is uppercase `*P*`.
/// * The fourth rucksack's compartments only share item type `*v*`.
/// * The fifth rucksack's compartments only share item type `*t*`.
/// * The sixth rucksack's compartments only share item type `*s*`.
///
/// To help prioritize item rearrangement, every item type can be converted to a
/// *priority*:
///
/// * Lowercase item types `a` through `z` have priorities 1 through 26.
/// * Uppercase item types `A` through `Z` have priorities 27 through 52.
///
/// In the above example, the priority of the item type that appears in both
/// compartments of each rucksack is 16 (`p`), 38 (`L`), 42 (`P`), 22 (`v`), 20
/// (`t`), and 19 (`s`); the sum of these is `*157*`.
///
/// Find the item type that appears in both compartments of each rucksack. *What
/// is the sum of the priorities of those item types?*
///
/// \--- Part Two ---
/// ----------
///
/// As you finish identifying the misplaced items, the Elves come to you with
/// another issue.
///
/// For safety, the Elves are divided into groups of three. Every Elf carries a
/// badge that identifies their group. For efficiency, within each group of
/// three Elves, the badge is the *only item type carried by all three Elves*.
/// That is, if a group's badge is item type `B`, then all three Elves will have
/// item type `B` somewhere in their rucksack, and at most two of the Elves will
/// be carrying any other item type.
///
/// The problem is that someone forgot to put this year's updated authenticity
/// sticker on the badges. All of the badges need to be pulled out of the
/// rucksacks so the new authenticity stickers can be attached.
///
/// Additionally, nobody wrote down which item type corresponds to each group's
/// badges. The only way to tell which item type is the right one is by finding
/// the one item type that is *common between all three Elves* in each group.
///
/// Every set of three lines in your list corresponds to a single group, but
/// each group can have a different badge item type. So, in the above example,
/// the first group's rucksacks are the first three lines:
///
/// ```
/// vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// ```
///
/// And the second group's rucksacks are the next three lines:
///
/// ```
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw
/// ```
///
/// In the first group, the only item type that appears in all three rucksacks
/// is lowercase `r`; this must be their badges. In the second group, their
/// badge item type must be `Z`.
///
/// Priorities for these items must still be found to organize the sticker
/// attachment efforts: here, they are 18 (`r`) for the first group and 52 (`Z`)
/// for the second group. The sum of these is `*70*`.
///
/// Find the item type that corresponds to the badges of each three-Elf group.
/// *What is the sum of the priorities of those item types?*
static PROBLEM_INPUT: &str = include_str!("input.txt");
pub struct Day03;

struct Rucksack {
    first: u64,
    second: u64,
    combined: u64,
}

fn char_to_priority(c: char) -> u8 {
    if c.is_uppercase() {
        // A-Z starts at index 65, so for the range to be from 27-52 we
        // subtract 38.
        (c as u8) - 38
    } else {
        // a-z starts at index 97, so we subtract 96.
        (c as u8) - 96
    }
}

const fn set_bit(start: u64, priority: u8) -> u64 {
    let bit_to_set = 1 << priority;

    if start & bit_to_set == 0 {
        start + bit_to_set
    } else {
        start
    }
}

impl Day03 {
    fn parse(&self, data: &str) -> Vec<Rucksack> {
        let mut all_rucksacks = Vec::new();

        for line in data.lines() {
            let stripped_line = line.trim();
            let halfway = stripped_line.len() / 2;
            let (first, second) = stripped_line.char_indices().fold((0u64, 0u64), |acc, (i, c)| {
                let priority = char_to_priority(c);
                (
                    if i < halfway { set_bit(acc.0, priority) } else { acc.0 },
                    if i >= halfway { set_bit(acc.1, priority) } else { acc.1 },
                )
            });
            all_rucksacks.push(Rucksack { first, second, combined: first | second })
        }

        all_rucksacks
    }

    fn solve_actual(&self, rucksacks: &[Rucksack]) -> u32 {
        rucksacks.iter().map(|r| (r.first & r.second).trailing_zeros()).sum()
    }

    fn solve_actual_part2(&self, rucksacks: &[Rucksack]) -> u32 {
        rucksacks
            .chunks(3)
            .map(|rs| {
                rs.iter().skip(1).fold(rs[0].combined, |acc, r| acc & r.combined).trailing_zeros()
            })
            .sum()
    }
}

impl Problem for Day03 {
    fn day(&self) -> u8 {
        3u8
    }
    fn name(&self) -> &str {
        "Day 3: Rucksack Reorganization"
    }
    fn solve(&self) -> Solution {
        Solution::U32(self.solve_actual(&self.parse(PROBLEM_INPUT)))
    }
    fn solve_part2(&self) -> Solution {
        Solution::U32(self.solve_actual_part2(&self.parse(PROBLEM_INPUT)))
    }
}
