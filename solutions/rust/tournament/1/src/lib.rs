use std::collections::BTreeMap;

pub fn tally(match_results: &str) -> String {
    let mut table: Vec<String> = Vec::new();
    table.push(format!(
        "{:30} | {:2} | {:>2} | {:>2} | {:>2} | {:>2}",
        "Team", "MP", "W", "D", "L", "P",
    ));

    let standings: BTreeMap<&str, Record> =
        match_results
            .lines()
            .fold(BTreeMap::<&str, Record>::new(), |mut acc, line| {
                let match_result: Vec<&str> = line.split(";").collect();

                if let Some(&outcome) = match_result.get(2) {
                    let away = *match_result.get(0).unwrap();
                    let home = *match_result.get(1).unwrap();

                    match outcome {
                        "win" => {
                            acc.entry(away)
                                .and_modify(|rec| rec.wins += 1)
                                .or_insert(Record {
                                    wins: 1,
                                    losses: 0,
                                    draws: 0,
                                });
                            acc.entry(home)
                                .and_modify(|rec| rec.losses += 1)
                                .or_insert(Record {
                                    wins: 0,
                                    losses: 1,
                                    draws: 0,
                                });
                        }
                        "loss" => {
                            acc.entry(away)
                                .and_modify(|rec| rec.losses += 1)
                                .or_insert(Record {
                                    wins: 0,
                                    losses: 1,
                                    draws: 0,
                                });
                            acc.entry(home)
                                .and_modify(|rec| rec.wins += 1)
                                .or_insert(Record {
                                    wins: 1,
                                    losses: 0,
                                    draws: 0,
                                });
                        }
                        "draw" => {
                            acc.entry(away)
                                .and_modify(|rec| rec.draws += 1)
                                .or_insert(Record {
                                    wins: 0,
                                    losses: 0,
                                    draws: 1,
                                });
                            acc.entry(home)
                                .and_modify(|rec| rec.draws += 1)
                                .or_insert(Record {
                                    wins: 0,
                                    losses: 0,
                                    draws: 1,
                                });
                        }
                        _ => {}
                    }
                };

                acc
            });

    let mut sorted_records: Vec<(&str, Record)> =
        standings.into_iter().collect::<Vec<(&str, Record)>>();
    sorted_records.sort_by(|a, b| b.1.points().cmp(&a.1.points()).then(a.0.cmp(b.0)));

    for (team, rec) in sorted_records.iter() {
        table.push(format!(
            "{:30} | {:2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team,
            rec.matches(),
            rec.wins,
            rec.draws,
            rec.losses,
            rec.points()
        ));
    }

    table.join("\n")
}

struct Record {
    wins: u32,
    losses: u32,
    draws: u32,
}

impl Record {
    pub fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }

    pub fn matches(&self) -> u32 {
        self.wins + self.losses + self.draws
    }
}
