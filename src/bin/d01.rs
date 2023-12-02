
fn main() {
    let raw_input = include_str!("d01_input.txt");
    let lines = raw_input.split("\n");

    let tokens = [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut sum = 0;

    for line in lines {
        let first = if let Some((_, val)) = find_number(line, &tokens) { val % 10 } else { 0 };
        let last = if let Some((_, val)) = rfind_number(line, &tokens) { val % 10 } else { 0 };

        // let first = if let Some(i) = line.find(|c| c >= '0' && c <= '9') { line.as_bytes()[i] - '0' as u8 } else { 0 };
        // let last = if let Some(i) = line.rfind(|c| c >= '0' && c <= '9') { line.as_bytes()[i] - '0' as u8 } else { 0 };
        let result = first * 10 + last;
        println!("{line} = {result}");

        sum += result as u64;
    }

    println!("result sum = {sum}");
}

fn find_number(line: &str, tokens: &[&str]) -> Option<(usize, u8)> {
    tokens.iter().enumerate().filter_map(|(val, token)| line.find(token).map(|i| (i, val as u8))).min_by_key(|(i, _)| *i)
}
fn rfind_number(line: &str, tokens: &[&str]) -> Option<(usize, u8)> {
    tokens.iter().enumerate().filter_map(|(val, token)| line.rfind(token).map(|i| (i, val as u8))).max_by_key(|(i, _)| *i)
}