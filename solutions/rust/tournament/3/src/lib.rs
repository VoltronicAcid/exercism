use std::collections::BTreeMap;

pub fn tally(games: &str) -> String {
    let mut sorted_standings = games
        .lines()
        .fold(BTreeMap::new(), |mut map:BTreeMap<&str, (u32,u32,u32)>, line| {
            if let [team1, team2, result] = line.split(";").collect::<Vec<&str>>().as_slice() {
                match *result {
                    "win" => {
                        map.entry(team1).and_modify(|(w, _, _)| *w += 1).or_insert((1, 0, 0));
                        map.entry(team2).and_modify(|(_, l, _)| *l += 1).or_insert((0, 1, 0));
                    }
                    "loss" => {
                        map.entry(team1).and_modify(|(_, l, _)| *l += 1).or_insert((0, 1, 0));
                        map.entry(team2).and_modify(|(w, _, _)| *w += 1).or_insert((1, 0, 0));
                    }
                    "draw" => {
                        map.entry(team1).and_modify(|(_, _, d)| *d += 1).or_insert((0, 0, 1));
                        map.entry(team2).and_modify(|(_, _, d)| *d += 1).or_insert((0, 0, 1));
                    }
                    _ => unreachable!(),
                }
            }

            map
        }).into_iter()
        .map(|(name, (wins, losses, draws))| (name, (wins + losses + draws, wins, losses, draws, wins * 3 + draws)))
        .collect::<Vec<(&str, (u32, u32, u32, u32, u32))>>();

    sorted_standings.sort_by(|&(_, (_, _, _, _, a)), &(_, (_, _, _, _, b))| (b).cmp(&a));

    std::iter::once(String::from("Team                           | MP |  W |  D |  L |  P"))
        .chain(sorted_standings.into_iter()
            .map(|(team, (matches, wins, losses, draws, points))|
                format!("{team:30} | {matches:2} | {wins:>2} | {draws:>2} | {losses:>2} | {points:>2}")
            )).collect::<Vec<String>>().join("\n")
}
