use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Choice,
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
    Yacht,
}

pub fn score(dice: [u8; 5], category: Category) -> u8 {
    use Category::*;

    match category {
        Choice => dice.iter().sum(),
        Ones | Twos | Threes | Fours | Fives | Sixes => {
            dice.iter().filter(|&&dice| dice == category as u8).sum()
        }
        FullHouse => {
            let map = dice.iter().counts();

            if map.keys().len() == 2 && map.values().contains(&3) {
                dice.iter().sum()
            } else {
                0
            }
        }
        FourOfAKind => {
            let counts = dice.iter().counts();

            match counts.keys().find(|&&key| counts.get(key).unwrap() >= &4) {
                Some(&die) => *die * 4,
                None => 0,
            }
        }
        LittleStraight => {
            if dice.iter().copied().sorted().collect::<Vec<u8>>() == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        BigStraight => {
            if dice.iter().copied().sorted().collect::<Vec<u8>>() == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Yacht => {
            if dice.iter().all_equal() {
                50
            } else {
                0
            }
        }
    }
}
