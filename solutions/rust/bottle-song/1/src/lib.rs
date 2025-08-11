use std::collections::BTreeMap;

pub fn recite(mut start: u32, mut take: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    let words = BTreeMap::from([
        (10, "Ten"),
        (9, "Nine"),
        (8, "Eight"),
        (7, "Seven"),
        (6, "Six"),
        (5, "Five"),
        (4, "Four"),
        (3, "Three"),
        (2, "Two"),
        (1, "One"),
        (0, "no"),
    ]);
    let mut output: Vec<String> = Vec::new();
    let print_s = |n: u32| if n > 1 || n == 0 { "s" } else { "" };

    while take > 0 {
        let word = words.get(&start).unwrap();
        output.push(format!(
            "{} green bottle{} hanging on the wall,\n",
            word,
            print_s(start)
        ));
        output.push(format!(
            "{} green bottle{} hanging on the wall,\n",
            word,
            print_s(start)
        ));
        output.push(format!(
            "And if one green bottle should accidentally fall,\n"
        ));
        start -= 1;
        let word = words.get(&start).unwrap();
        output.push(format!(
            "There'll be {} green bottle{} hanging on the wall.\n",
            word.to_lowercase(),
            print_s(start)
        ));
        take -= 1;
        if take > 0 {
            output.push(String::from("\n"));
        }
    }

    output.join("")
}
