

fn delta_sequence_extrapolate<'a>(sequence: impl Iterator<Item=&'a i32>) -> i32 {
    let mut prev = Option::<i32>::None;
    let deltas = sequence.filter_map(|cur| {
        if let Some(prev_val) = prev.replace(*cur) {
            Some(cur - prev_val)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    if deltas.is_empty() || deltas.iter().all(|i| *i == 0) {
        prev.unwrap_or(0)
    } else {
        prev.unwrap_or(0) + delta_sequence_extrapolate(deltas.iter())
    }
}

fn main() {
    let raw_input = include_str!("d09_input.txt");
    let lines = raw_input.lines();

    let (sum_before, sum_next) = lines.map(|line|
        line.split_whitespace().map(|word| word.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .map(|sequence| {
            let next = delta_sequence_extrapolate(sequence.iter());
            println!("Sequence {sequence:?} extrapolates to {next}");

            let before = delta_sequence_extrapolate(sequence.iter().rev());
            (before, next)
        }).fold((0, 0), |(sum_before, sum_next), (before, next)| {
            (sum_before + before, sum_next + next)
        });

    println!("Sequences extrapolated sum is {sum_next}, the extrapolated history sum is {sum_before}");
}