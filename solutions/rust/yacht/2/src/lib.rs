use itertools::Itertools;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

enum Straight {
    Little,
    Big,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    use Category::*;
    use Straight::{Big, Little};

    match category {
        Ones => score_nums(dice, 1),
        Twos => score_nums(dice, 2),
        Threes => score_nums(dice, 3),
        Fours => score_nums(dice, 4),
        Fives => score_nums(dice, 5),
        Sixes => score_nums(dice, 6),
        FullHouse => score_full_house(dice),
        FourOfAKind => score_four_of_a_kind(dice),
        LittleStraight => score_straight(dice, Little),
        BigStraight => score_straight(dice, Big),
        Choice => dice.iter().sum(),
        Yacht => score_yacht(dice),
    }
}

fn score_nums(dice: Dice, val: u8) -> u8 {
    val * dice.into_iter().filter(|&die| die == val).count() as u8
}

fn score_full_house(dice: Dice) -> u8 {
    let sum = dice.iter().sum();
    let counts = get_counts(dice)
        .into_iter()
        .filter(|&val| val > 1)
        .collect::<Vec<u8>>();

    if counts.len() == 2 && counts.into_iter().sum::<u8>() == 5 {
        sum
    } else {
        0
    }
}

fn score_four_of_a_kind(dice: Dice) -> u8 {
    let counts = get_counts(dice);

    if let Some(idx) = counts.iter().position(|&val| val > 3) {
        4 * (idx + 1) as u8
    } else {
        0
    }
}

fn score_straight(dice: Dice, kind: Straight) -> u8 {
    use Straight::{Big, Little};

    let expected = match kind {
        Little => [1u8, 2, 3, 4, 5],
        Big => [2u8, 3, 4, 5, 6],
    };

    let sorted: Vec<u8> = dice.into_iter().sorted().collect();

    if sorted == expected {
        30
    } else {
        0
    }
}

fn score_yacht(dice: Dice) -> u8 {
    if dice.into_iter().all(|die| die == dice[0]) {
        50
    } else {
        0
    }
}

fn get_counts(dice: Dice) -> Vec<u8> {
    dice.iter()
        .fold([0u8; 6], |mut acc, &die| {
            acc[(die - 1) as usize] += 1;
            acc
        })
        .into_iter()
        .collect::<Vec<u8>>()
}
