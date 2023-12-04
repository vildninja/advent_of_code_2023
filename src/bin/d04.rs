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


    // part 2

    let lines = raw_input.lines();

    let res2: u32 = lines.filter_map(|line| {
        if let (Some(i_begin), Some(i_mid)) = (line.find(':'), line.find('|')) {
            let card = line[i_mid + 1..].split_whitespace().collect::<HashSet<_>>();

            let winning = line[i_begin + 1..i_mid].split_whitespace().fold(0u32, |score, number| {
                if card.contains(number) { score + 1 }
                else { score }
            });

            Some(winning)
        }
        else { None }
    }).enumerate().fold(Vec::<u32>::new(), |mut stack, (card, won)| {

        println!("Card {card} won {won} cards");

        while stack.len() <= card + won as usize {
            stack.push(1);
        }

        let copies = stack[card];


        println!("Card {card} won {won} cards * {copies} copies");

        for i in card + 1 ..= card + won as usize {
            stack[i] += copies;
        }

        stack
    }).iter().sum();


    println!("Total cards {res2}");
}