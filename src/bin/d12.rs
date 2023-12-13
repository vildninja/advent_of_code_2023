use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const WORKING:u8 = b'.';
const BROKEN:u8 = b'#';
const UNKNOWN:u8 = b'?';


// very slow solution
// see d12_p2.rs

fn next_sequence(records: &[u8], groups: &[usize]) -> usize {
    if let Some(&group) = groups.first() {

        let min_length = groups.iter().map(|&g| g + 1).sum::<usize>() - 1;

        if records.len() < min_length {
            return 0;
        }


        return (0..=records.len() - min_length)
            .scan(UNKNOWN, |last, i| {
                if *last == BROKEN {
                    None
                } else {
                    *last = records[i];
                    Some(i)
                }
            }).filter(|&i| {
                records.get(i + group).map_or(true, |&c| c != BROKEN) &&
                    records[i..i + group].iter().all(|&c| c == BROKEN || c == UNKNOWN)
        }).map(|i| {

            next_sequence(&records[(i + group + 1).min(records.len())..], &groups[1..])
        }).sum();
    } else if records.iter().all(|&c| c == WORKING || c == UNKNOWN) {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let raw_input = include_str!("d12_input.txt");

    let sum = raw_input.lines().map(|line| {
        let mut words = line.split_ascii_whitespace();
        let records = words.next().unwrap().as_bytes();
        let groups = words.next().unwrap()
            .split(',').map(|g| g.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let solutions = next_sequence(records, &groups);

        println!("Found {solutions} for {line}");

        solutions
    }).sum::<usize>();

    println!("Total solutions {sum}");

    // part 2


    let runner = |channel: Arc<Mutex<VecDeque<&str>>>| {
        let mut sum = 0;
        loop {
            let (records, groups) = if let Some(line) = channel.lock().unwrap().pop_front() {
                let mut words = line.split_ascii_whitespace();
                (words.next().unwrap().as_bytes().iter().copied().collect::<Vec<_>>(),
                    words.next().unwrap()
                    .split(',').map(|g| g.parse::<usize>().unwrap()).collect::<Vec<_>>())
            } else { return sum };

            let solutions = next_sequence(
                &records.iter().chain(b"?").cycle().take(records.len() * 5 + 4).copied().collect::<Vec<_>>(),
                &groups.iter().cycle().take(groups.len() * 5).copied().collect::<Vec<_>>());

            println!("Found {solutions} for {}", records.iter().map(|c| *c as char).collect::<String>());

            sum += solutions;
        }
    };

    let channel: Arc<Mutex<VecDeque<&str>>> = Arc::new(Mutex::new(
        raw_input.lines().collect::<VecDeque::<_>>()));

    let mut handles = Vec::new();

    for _ in 0..12 {
        let chan = channel.clone();
        handles.push(Some(std::thread::spawn(move || runner(chan))));
    }

    let sum2 = handles.iter_mut().map(|handle| {
        handle.take().unwrap().join().unwrap()
    }).sum::<usize>();

    println!("Total solutions x5 {sum2}");
}