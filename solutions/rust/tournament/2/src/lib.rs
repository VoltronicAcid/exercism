use std::collections::BTreeMap;

enum GameResult {
    Win,
    Loss,
    Draw,
}

#[derive(Default)]
struct Record {
    matches: u8,
    wins: u8,
    losses: u8,
    draws: u8,
    points: u8,
}

impl Record {
    fn new(game: GameResult) -> Self {
        let mut record = Record::default();

        match game {
            GameResult::Win => record.add_win(),
            GameResult::Loss => record.add_loss(),
            GameResult::Draw => record.add_draw(),
        };

        record
    }

    fn add_win(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn add_loss(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }

    fn add_draw(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }
}

pub fn tally(games: &str) -> String {
    let mut sorted_standings = games
        .lines()
        .fold(
            BTreeMap::new(),
            |mut map: BTreeMap<&str, Record>, result| {
                if let [team1, team2, game_result] =
                    result.split(";").collect::<Vec<&str>>().as_slice()
                {
                    match *game_result {
                        "win" => {
                            map.entry(team1)
                                .and_modify(|record| record.add_win())
                                .or_insert(Record::new(GameResult::Win));
                            map.entry(team2)
                                .and_modify(|record| record.add_loss())
                                .or_insert(Record::new(GameResult::Loss));
                        }
                        "loss" => {
                            map.entry(team1)
                                .and_modify(|record| record.add_loss())
                                .or_insert(Record::new(GameResult::Loss));
                            map.entry(team2)
                                .and_modify(|record| record.add_win())
                                .or_insert(Record::new(GameResult::Win));
                        }
                        "draw" => {
                            map.entry(team1)
                                .and_modify(|record| record.add_draw())
                                .or_insert(Record::new(GameResult::Draw));
                            map.entry(team2)
                                .and_modify(|record| record.add_draw())
                                .or_insert(Record::new(GameResult::Draw));
                        }
                        _ => unreachable!(),
                    }
                };

                map
            },
        )
        .into_iter()
        .collect::<Vec<(&str, Record)>>();

    sorted_standings.sort_by(|(_, a), (_, b)| b.points.cmp(&a.points));

    std::iter::once(String::from(
        "Team                           | MP |  W |  D |  L |  P",
    ))
    .chain(sorted_standings.into_iter().map(
        |(
            team,
            Record {
                matches,
                wins,
                losses,
                draws,
                points,
            },
        )| {
            format!("{team:30} | {matches:2} | {wins:>2} | {draws:>2} | {losses:>2} | {points:>2}")
        },
    ))
    .collect::<Vec<String>>()
    .join("\n")
}
