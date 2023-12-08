use common::{Problem, Solution};
use std::collections::HashMap;
/// \--- Day 8: Haunted Wasteland ---
/// ----------
///
/// You're still riding a camel across Desert Island when you spot a sandstorm
/// quickly approaching. When you turn to warn the Elf, she disappears before
/// your eyes! To be fair, she had just finished warning you about *ghosts* a
/// few minutes ago.
///
/// One of the camel's pouches is labeled "maps" - sure enough, it's full of
/// documents (your puzzle input) about how to navigate the desert. At least,
/// you're pretty sure that's what they are; one of the documents contains a
/// list of left/right instructions, and the rest of the documents seem to
/// describe some kind of *network* of labeled nodes.
///
/// It seems like you're meant to use the *left/right* instructions to *navigate
/// the network*. Perhaps if you have the camel follow the same instructions,
/// you can escape the haunted wasteland!
///
/// After examining the maps for a bit, two nodes stick out: `AAA` and `ZZZ`.
/// You feel like `AAA` is where you are now, and you have to follow the
/// left/right instructions until you reach `ZZZ`.
///
/// This format defines each *node* of the network individually. For example:
///
/// ```
/// RL
///
/// AAA = (BBB, CCC)
/// BBB = (DDD, EEE)
/// CCC = (ZZZ, GGG)
/// DDD = (DDD, DDD)
/// EEE = (EEE, EEE)
/// GGG = (GGG, GGG)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting with `AAA`, you need to *look up the next element* based on the
/// next left/right instruction in your input. In this example, start with `AAA`
/// and go *right* (`R`) by choosing the right element of `AAA`, `*CCC*`. Then,
/// `L` means to choose the *left* element of `CCC`, `*ZZZ*`. By following the
/// left/right instructions, you reach `ZZZ` in `*2*` steps.
///
/// Of course, you might not find `ZZZ` right away. If you run out of left/right
/// instructions, repeat the whole sequence of instructions as necessary: `RL`
/// really means `RLRLRLRLRLRLRLRL...` and so on. For example, here is a
/// situation that takes `*6*` steps to reach `ZZZ`:
///
/// ```
/// LLR
///
/// AAA = (BBB, BBB)
/// BBB = (AAA, ZZZ)
/// ZZZ = (ZZZ, ZZZ)
/// ```
///
/// Starting at `AAA`, follow the left/right instructions. *How many steps are
/// required to reach `ZZZ`?*
///
/// \--- Part Two ---
/// ----------
///
/// The sandstorm is upon you and you aren't any closer to escaping the
/// wasteland. You had the camel follow the instructions, but you've barely left
/// your starting position. It's going to take *significantly more steps* to
/// escape!
///
/// What if the map isn't for people - what if the map is for *ghosts*? Are
/// ghosts even bound by the laws of spacetime? Only one way to find out.
///
/// After examining the maps a bit longer, your attention is drawn to a curious
/// fact: the number of nodes with names ending in `A` is equal to the number
/// ending in `Z`! If you were a ghost, you'd probably just *start at every node
/// that ends with `A`* and follow all of the paths at the same time until they
/// all simultaneously end up at nodes that end with `Z`.
///
/// For example:
///
/// ```
/// LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)
/// ```
///
/// Here, there are two starting nodes, `11A` and `22A` (because they both end
/// with `A`). As you follow each left/right instruction, use that instruction
/// to *simultaneously* navigate away from both nodes you're currently on.
/// Repeat this process until *all* of the nodes you're currently on end with
/// `Z`. (If only some of the nodes you're on end with `Z`, they act like any
/// other node and you continue as normal.) In this example, you would proceed
/// as follows:
///
/// * Step 0: You are at `11A` and `22A`.
/// * Step 1: You choose all of the *left* paths, leading you to `11B` and
///   `22B`.
/// * Step 2: You choose all of the *right* paths, leading you to `*11Z*` and
///   `22C`.
/// * Step 3: You choose all of the *left* paths, leading you to `11B` and
///   `*22Z*`.
/// * Step 4: You choose all of the *right* paths, leading you to `*11Z*` and
///   `22B`.
/// * Step 5: You choose all of the *left* paths, leading you to `11B` and
///   `22C`.
/// * Step 6: You choose all of the *right* paths, leading you to `*11Z*` and
///   `*22Z*`.
///
/// So, in this example, you end up entirely on nodes that end in `Z` after
/// `*6*` steps.
///
/// Simultaneously start on every node that ends with `A`. *How many steps does
/// it take before you're only on nodes that end with `Z`?*
pub struct Day08;

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    left: u16,
    right: u16,
}

#[derive(Debug, Clone)]
struct Network {
    nodes: HashMap<u16, Node>,
}

const fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        (b, a) = (a % b, b);
    }
    a
}

const fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

const fn as_num(c: char) -> u16 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        '1' => 26,
        '2' => 27,
        _ => unimplemented!(),
    }
}

const fn hash(c: &[char]) -> u16 {
    // Since we have <= 32 options for characters, we only need 5 bits per
    // character, so each entry can be a u16 instead.
    as_num(c[0]) << 10 | as_num(c[1]) << 5 | as_num(c[2])
}

static PART_1_GOAL: u16 = hash(&['Z', 'Z', 'Z']);

impl Network {
    fn traverse(&self, moves: &[Move]) -> u16 {
        let mut total_steps = 0;
        let mut current = 0;
        let mut i: usize = 0;
        loop {
            let current_move = moves[i];
            let node = self.nodes.get(&current).expect("Invalid node");
            current = match current_move {
                Move::Left => node.left,
                Move::Right => node.right,
            };
            total_steps += 1;
            i += 1;
            if i >= moves.len() {
                i = 0;
            }
            if current == PART_1_GOAL {
                break;
            }
        }
        total_steps
    }

    fn ghost_traverse(&self, moves: &[Move]) -> u64 {
        let mut total_steps = 0;
        let mut current: [Option<u16>; 16] = [None; 16];
        let mut required_matches = 0;
        for (i, k) in self.nodes.keys().filter(|n| *n & 0b11111 == 0).enumerate() {
            current[i] = Some(*k);
            required_matches += 1;
        }
        let mut last_match: [Option<u16>; 16] = [None; 16];
        let mut last_cycle_count = [0; 16];
        let mut i: usize = 0;

        loop {
            total_steps += 1;
            let current_move = moves[i];

            let mut next = [None; 16];
            for (idx, current_node) in
                current.iter().enumerate().filter_map(|(idx, &n)| n.map(|n| (idx, n)))
            {
                let node = self.nodes.get(&current_node).expect("Invalid node");
                let next_node = match current_move {
                    Move::Left => &node.left,
                    Move::Right => &node.right,
                };
                next[idx] = Some(*next_node);
                if (next_node & as_num('Z') == as_num('Z')) && last_match[idx].is_none() {
                    last_match[idx] = Some(*next_node);
                    last_cycle_count[idx] = total_steps;
                }
            }

            if last_match.iter().take(required_matches).all(|&b| b.is_some()) {
                let all_lcm =
                    last_cycle_count.iter().take(required_matches).fold(1, |acc, &n| lcm(acc, n));
                total_steps = all_lcm;
                break;
            }

            i += 1;
            if i >= moves.len() {
                i = 0;
            }
            current = next;
        }

        total_steps
    }
}

impl Day08 {
    fn parse(&self, input: &str) -> (Vec<Move>, Network) {
        let mut lines = input.lines().map(|l| l.trim());
        let moves = lines
            .next()
            .unwrap_or_default()
            .chars()
            .map(|c| match c {
                'L' => Move::Left,
                'R' => Move::Right,
                _ => panic!("Invalid move: {}", c),
            })
            .collect::<Vec<_>>();
        let _ = lines.next();
        let mut nodes = HashMap::new();
        while let Some(line) = lines.next().and_then(|l| l.split_once(" = ")) {
            let (value, children) = line;
            let (left, right) = children.split_once(", ").expect("Expected two children for node");
            nodes.insert(
                hash(&value.chars().collect::<Vec<_>>()),
                Node {
                    left: hash(&left[1..].chars().collect::<Vec<_>>()),
                    right: hash(&right[..right.len() - 1].chars().collect::<Vec<_>>()),
                },
            );
        }
        (moves, Network { nodes })
    }
}

impl Problem for Day08 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        8u8
    }
    fn name(&self) -> &str {
        "Day 8: Haunted Wasteland"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        let (moves, network) = self.parse(input);
        Solution::U16(network.traverse(&moves))
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        let (moves, network) = self.parse(input);
        Solution::U64(network.ghost_traverse(&moves))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let input = "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)";
        let problem = Day08 {};
        assert_eq!(problem.solve_part1_with(input), Solution::U16(2));
        let input2 = "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)";
        assert_eq!(problem.solve_part1_with(input2), Solution::U16(6));
    }
    #[test]
    fn test_part1_real_input() {
        let problem = Day08 {};
        assert_eq!(problem.solve_part1(), Solution::U16(18827));
    }
    #[test]
    fn test_part2_example() {
        let input = "LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)";
        let problem = Day08 {};
        assert_eq!(problem.solve_part2_with(input), Solution::U64(6));
    }
    #[test]
    fn test_part2_real_input() {
        let problem = Day08 {};
        assert_eq!(problem.solve_part2(), Solution::U64(20220305520997));
    }
}
