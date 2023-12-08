use common::{Problem, Solution};
/// \--- Day 7: Camel Cards ---
/// ----------
///
/// Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an [airship](https://en.wikipedia.org/wiki/Airship). (At least it's a *cool* airship!) It drops you off at the edge of a vast desert and descends back to Island Island.
///
/// "Did you bring the parts?"
///
/// You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large [camel](https://en.wikipedia.org/wiki/Dromedary).
///
/// "Did you bring the parts?" she asks again, louder this time. You aren't sure
/// what parts she's looking for; you're here to figure out why the sand
/// stopped.
///
/// "The parts! For the sand, yes! Come with me; I will show you." She beckons
/// you onto the camel.
///
/// After riding a bit across the sands of Desert Island, you can see what look
/// like very large rocks covering half of the horizon. The Elf explains that
/// the rocks are all along the part of Desert Island that is directly above
/// Island Island, making it hard to even get there. Normally, they use big
/// machines to move the rocks and filter the sand, but the machines have broken
/// down because Desert Island recently stopped receiving the *parts* they need
/// to fix the machines.
///
/// You've already assumed it'll be your job to figure out why the parts stopped
/// when she asks if you can help. You agree automatically.
///
/// Because the journey will take a few days, she offers to teach you the game of *Camel Cards*. Camel Cards is sort of similar to [poker](https://en.wikipedia.org/wiki/List_of_poker_hands) except it's designed to be easier to play while riding a camel.
///
/// In Camel Cards, you get a list of *hands*, and your goal is to order them
/// based on the *strength* of each hand. A hand consists of *five cards*
/// labeled one of `A`, `K`, `Q`, `J`, `T`, `9`, `8`, `7`, `6`, `5`, `4`, `3`,
/// or `2`. The relative strength of each card follows this order, where `A` is
/// the highest and `2` is the lowest.
///
/// Every hand is exactly one *type*. From strongest to weakest, they are:
///
/// * *Five of a kind*, where all five cards have the same label: `AAAAA`
/// * *Four of a kind*, where four cards have the same label and one card has a
///   different label: `AA8AA`
/// * *Full house*, where three cards have the same label, and the remaining two
///   cards share a different label: `23332`
/// * *Three of a kind*, where three cards have the same label, and the
///   remaining two cards are each different from any other card in the hand:
///   `TTT98`
/// * *Two pair*, where two cards share one label, two other cards share a
///   second label, and the remaining card has a third label: `23432`
/// * *One pair*, where two cards share one label, and the other three cards
///   have a different label from the pair and each other: `A23A4`
/// * *High card*, where all cards' labels are distinct: `23456`
///
/// Hands are primarily ordered based on type; for example, every *full house*
/// is stronger than any *three of a kind*.
///
/// If two hands have the same type, a second ordering rule takes effect. Start
/// by comparing the *first card in each hand*. If these cards are different,
/// the hand with the stronger first card is considered stronger. If the first
/// card in each hand have the *same label*, however, then move on to
/// considering the *second card in each hand*. If they differ, the hand with
/// the higher second card wins; otherwise, continue with the third card in each
/// hand, then the fourth, then the fifth.
///
/// So, `33332` and `2AAAA` are both *four of a kind* hands, but `33332` is
/// stronger because its first card is stronger. Similarly, `77888` and `77788`
/// are both a *full house*, but `77888` is stronger because its third card is
/// stronger (and both hands have the same first and second card).
///
/// To play Camel Cards, you are given a list of hands and their corresponding
/// *bid* (your puzzle input). For example:
///
/// ```
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// This example shows five hands; each hand is followed by its *bid* amount.
/// Each hand wins an amount equal to its bid multiplied by its *rank*, where
/// the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on
/// up to the strongest hand. Because there are five hands in this example, the
/// strongest hand will have rank 5 and its bid will be multiplied by 5.
///
/// So, the first step is to put the hands in order of strength:
///
/// * `32T3K` is the only *one pair* and the other hands are all a stronger
///   type, so it gets rank *1*.
/// * `KK677` and `KTJJT` are both *two pair*. Their first cards both have the
///   same label, but the second card of `KK677` is stronger (`K` vs `T`), so
///   `KTJJT` gets rank *2* and `KK677` gets rank *3*.
/// * `T55J5` and `QQQJA` are both *three of a kind*. `QQQJA` has a stronger
///   first card, so it gets rank *5* and `T55J5` gets rank *4*.
///
/// Now, you can determine the total winnings of this set of hands by adding up
/// the result of multiplying each hand's bid with its rank (`765` \* 1 + `220`
/// \* 2 + `28` \* 3 + `684` \* 4 + `483` \* 5). So the *total winnings* in this
/// example are `*6440*`.
///
/// Find the rank of every hand in your set. *What are the total winnings?*
///
/// \--- Part Two ---
/// ----------
///
/// To make things a little more interesting, the Elf introduces one additional rule. Now, `J` cards are [jokers](https://en.wikipedia.org/wiki/Joker_(playing_card)) - wildcards that can act like whatever card would make the hand the strongest type possible.
///
/// To balance this, *`J` cards are now the weakest* individual cards, weaker
/// even than `2`. The other cards stay in the same order: `A`, `K`, `Q`, `T`,
/// `9`, `8`, `7`, `6`, `5`, `4`, `3`, `2`, `J`.
///
/// `J` cards can pretend to be whatever card is best for the purpose of
/// determining hand type; for example, `QJJQ2` is now considered *four of a
/// kind*. However, for the purpose of breaking ties between two hands of the
/// same type, `J` is always treated as `J`, not the card it's pretending to be:
/// `JKKK2` is weaker than `QQQQ2` because `J` is weaker than `Q`.
///
/// Now, the above example goes very differently:
///
/// ```
/// 32T3K 765
/// T55J5 684
/// KK677 28
/// KTJJT 220
/// QQQJA 483
/// ```
///
/// * `32T3K` is still the only *one pair*; it doesn't contain any jokers, so
///   its strength doesn't increase.
/// * `KK677` is now the only *two pair*, making it the second-weakest hand.
/// * `T55J5`, `KTJJT`, and `QQQJA` are now all *four of a kind*! `T55J5` gets
///   rank 3, `QQQJA` gets rank 4, and `KTJJT` gets rank 5.
///
/// With the new joker rule, the total winnings in this example are `*5905*`.
///
/// Using the new joker rule, find the rank of every hand in your set. *What are
/// the new total winnings?*
pub struct Day07;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<Card> for usize {
    fn from(card: Card) -> Self {
        match card {
            Card::Joker => 0,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::Ten => 9,
            Card::Jack => 10,
            Card::Queen => 11,
            Card::King => 12,
            Card::Ace => 13,
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(cards: [Card; 5], use_jokers: bool) -> HandType {
    let mut counts = [0u8; 14];
    let mut jokers = 0u8;
    for card in cards {
        if card == Card::Joker {
            jokers += 1;
            continue;
        }
        counts[usize::from(card)] += 1;
    }
    counts.sort();
    counts.reverse();
    if use_jokers {
        counts[0] += jokers;
    }
    match (counts[0], counts[1]) {
        (5, _) => HandType::FiveOfAKind,
        (4, _) => HandType::FourOfAKind,
        (3, 2) => HandType::FullHouse,
        (3, _) => HandType::ThreeOfAKind,
        (2, 2) => HandType::TwoPair,
        (2, _) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn compare(
    cards1: [Card; 5],
    type1: HandType,
    cards2: [Card; 5],
    type2: HandType,
) -> std::cmp::Ordering {
    let type_cmp = type1.cmp(&type2);
    if type_cmp != std::cmp::Ordering::Equal {
        return type_cmp;
    }

    for i in 0..5 {
        match cards1[i].cmp(&cards2[i]) {
            std::cmp::Ordering::Equal => {}
            o => return o,
        }
    }
    std::cmp::Ordering::Equal
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        Self { cards, hand_type: hand_type(cards, false) }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare(self.cards, self.hand_type, other.cards, other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct JokerHand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl JokerHand {
    fn new(cards: [Card; 5]) -> Self {
        let mut replaced = cards;
        replaced.iter_mut().for_each(|c| {
            if *c == Card::Jack {
                *c = Card::Joker;
            }
        });
        Self { cards: replaced, hand_type: hand_type(replaced, true) }
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare(self.cards, self.hand_type, other.cards, other.hand_type)
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

enum CamelHand {
    Hand(Hand),
    JokerHand(JokerHand),
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Hand(h1), Self::Hand(h2)) => {
                h1.cards == h2.cards && h1.hand_type == h2.hand_type
            }
            (Self::JokerHand(h1), Self::JokerHand(h2)) => {
                h1.cards == h2.cards && h1.hand_type == h2.hand_type
            }
            _ => false,
        }
    }
}

impl Eq for CamelHand {}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Hand(h1), Self::Hand(h2)) => h1.cmp(h2),
            (Self::JokerHand(h1), Self::JokerHand(h2)) => h1.cmp(h2),
            _ => unimplemented!(),
        }
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Day07 {
    fn parse(&self, input: &str, use_jokers: bool) -> Vec<(CamelHand, u64)> {
        input
            .lines()
            .map(|l| l.trim())
            .filter_map(|l| l.split_once(' '))
            .map(|(cards, bet)| {
                let mut card_chars = cards.chars();
                let cards = if let (Some(c1), Some(c2), Some(c3), Some(c4), Some(c5)) = (
                    card_chars.next(),
                    card_chars.next(),
                    card_chars.next(),
                    card_chars.next(),
                    card_chars.next(),
                ) {
                    [Card::from(c1), Card::from(c2), Card::from(c3), Card::from(c4), Card::from(c5)]
                } else {
                    panic!("Invalid card string: {}", cards)
                };
                let hand = if use_jokers {
                    CamelHand::JokerHand(JokerHand::new(cards))
                } else {
                    CamelHand::Hand(Hand::new(cards))
                };
                let bet = bet.parse::<u64>().unwrap_or_default();
                (hand, bet)
            })
            .collect()
    }

    fn solve_shared(&self, input: &str, use_jokers: bool) -> u64 {
        let mut parsed = self.parse(input, use_jokers);
        parsed.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
        parsed.iter().enumerate().fold(0, |acc, (i, (_, bet))| acc + (i + 1) as u64 * bet)
    }
}

impl Problem for Day07 {
    fn problem_input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn day(&self) -> u8 {
        7u8
    }
    fn name(&self) -> &str {
        "Day 7: Camel Cards"
    }
    fn solve_part1_with(&self, input: &str) -> Solution {
        Solution::U64(self.solve_shared(input, false))
    }
    fn solve_part2_with(&self, input: &str) -> Solution {
        Solution::U64(self.solve_shared(input, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let problem = Day07 {};
        assert_eq!(problem.solve_part1_with(input), Solution::U64(6440));
    }

    #[test]
    fn test_part1_real_input() {
        let problem = Day07 {};
        assert_eq!(problem.solve_part1(), Solution::U64(250957639));
    }

    #[test]
    fn test_part2_example() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let problem = Day07 {};
        assert_eq!(problem.solve_part2_with(input), Solution::U64(5905));
    }

    #[test]
    fn test_part2_real_input() {
        let problem = Day07 {};
        assert_eq!(problem.solve_part2(), Solution::U64(251515496));
    }
}
