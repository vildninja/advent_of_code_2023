
fn main() {
    let raw_input = include_str!("d06_input.txt");
    let lines = &mut raw_input.lines();

    let to_u64 = |word: &str| word.parse::<u64>().ok();

    let times = lines.next().unwrap().split_whitespace().filter_map(to_u64);
    let distances = lines.next().unwrap().split_whitespace().filter_map(to_u64);

    let games = times.zip(distances).collect::<Vec<_>>();

    let calc_dist = |push: u64, time: u64| push * (time - push);

    let result: usize = games.iter().map(|(time, distance)| {
        (1..*time).filter(|push| calc_dist(*push, *time) > *distance).count()
    }).product();

    println!("Result {result}");

    // part 2


    let lines = raw_input.lines();
    if let [time, distance] = lines.take(2).map(|line|
        line.chars().fold(0u64, |num, c|
            if let Some(digit) = c.to_digit(10) { num * 10 + digit as u64 }
            else { num })).collect::<Vec<_>>()[..] {
        println!("Part 2 time {time} distance {distance}");

        // f it, it's only 40 mil. No math today
        let result = (1..time).filter(|push| calc_dist(*push, time) > distance).count();

        println!("Part 2 result {result}");
    }
}