
fn main() {
    let raw_input = include_str!("d07_input.txt");
    let lines = raw_input.lines();

    let mut hands = lines.filter_map(|line| {
        let mut words = line.split_whitespace();
        if let (Some(cards_str), Some(Ok(bid))) = (words.next(), words.next().map(|bid| bid.parse::<u32>())) {
            let (cards, mut combo) = cards_str.chars().fold((0u32, [(0, 0); 4]), |(num, mut combo), c| {
                let new_card = c.to_digit(10).or_else(|| {
                    match c {
                        'A' => Some(14),
                        'K' => Some(13),
                        'Q' => Some(12),
                        'J' => Some(11),
                        'T' => Some(10),
                        _ => None,
                    }
                }).unwrap();

                if let Some((card, count)) = combo.iter_mut().find(|(card, _count)| *card == new_card || *card == 0) {
                    *card = new_card;
                    *count += 1;
                }

                ((num << 4) + new_card, combo)
            });

            combo.sort_by_key(|(_, count)| *count);
            let combo_type: u32 = match combo {
                [.., (_, 2), (_, n)] => n * 0x10 + 0x02,
                [.., (_, n)]         => n * 0x10,
                // [_, (_, 5)] => 7,
                // [_, (_, 4)] => 6,
                // [(_, 2), (_, 3)] => 5,
                // [_, (_, 3)] => 4,
                // [(_, 2), (_, 2)] => 3,
                // [_, (_, 2)] => 2,
                // _ => 1,
            };

            Some(((combo_type << 24) + cards, bid))
        }
        else { None }
    }).collect::<Vec<_>>();

    hands.sort_by_key(|(cards, _)| *cards);
    let total_winnings: u32 = hands.iter().enumerate().map(|(rank, (_, bid))| (rank as u32 + 1) * bid).sum();

    println!("Total winnings {total_winnings}");


    // Part 2

//     let lines = "32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483".lines();
    let lines = raw_input.lines();

    let mut hands2 = lines.filter_map(|line| {
        let mut words = line.split_whitespace();
        if let (Some(cards_str), Some(Ok(bid))) = (words.next(), words.next().map(|bid| bid.parse::<u32>())) {
            let (cards, mut combo, jokers) = cards_str.chars().fold((0u32, [(0, 0); 4], 0), |(num, mut combo, mut jesters), c| {
                let new_card = c.to_digit(10).or_else(|| {
                    match c {
                        'A' => Some(14),
                        'K' => Some(13),
                        'Q' => Some(12),
                        'J' => Some(0),
                        'T' => Some(10),
                        _ => None,
                    }
                }).unwrap();

                if c == 'J' {
                    jesters += 1;
                }
                else if let Some((card, count)) = combo.iter_mut().find(|(card, _count)| *card == new_card || *card == 0) {
                    *card = new_card;
                    *count += 1;
                }

                ((num << 4) + new_card, combo, jesters)
            });

            combo.sort_by_key(|(_, count)| *count);
            let combo_type: u32 = match combo {
                // can never happen with jokers
                // [(_, 2), (_, 3)] if jokers == 0 => 3 * 0x10 + 0x02,
                // can never happen with more than one joker
                // [(_, 2), (_, 2)] if jokers <= 1 => 2 * 0x10 + 0x02 + jokers * 0x10,

                [.., (_, 2), (_, n)] => n * 0x10 + 0x02 + jokers * 0x10,
                [.., (_, n)]         => n * 0x10 + jokers * 0x10,
            };


            Some(((combo_type << 24) + cards, bid))
        }
        else { None }
    }).collect::<Vec<_>>();

    hands2.sort_by_key(|(cards, _)| *cards);
    let total_winnings2: u32 = hands2.iter().enumerate().map(|(rank, (_, bid))| (rank as u32 + 1) * bid).sum();

    // let hands_value = hands2.iter().map(|(cards, _bid)| *cards).collect::<Vec<_>>();
    // println!("Hands: {:X?}", hands_value);

    println!("Total wildcard winnings {total_winnings2}");


}