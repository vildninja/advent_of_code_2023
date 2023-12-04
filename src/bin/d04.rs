use std::collections::HashSet;

fn main() {
    let raw_input = include_str!("d04_input.txt");
    let lines = raw_input.lines();

    let res = lines.fold(0u32, | sum, line| {
        if let (Some(i_begin), Some(i_mid)) = (line.find(':'), line.find('|')) {
            let card = line[i_mid + 1..].split_whitespace().collect::<HashSet<_>>();
            // let winning = line[i_begin + 1..i_mid].split_whitespace().collect::<Vec<_>>();

            let winning = line[i_begin + 1..i_mid].split_whitespace().fold(0u32, |score, number| {
                if card.contains(number) { (score * 2).max(1) }
                else { score }
            });

            println!("{} scored {winning}", &line[..i_begin]);
            sum + winning
        }
        else {
            sum
        }
    });

    println!("total won {res}");


}