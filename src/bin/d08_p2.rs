use std::collections::HashMap;
use prime_factorization::Factorization;

fn main() {
    let raw_input = include_str!("d08_input.txt");
    let _demo_input =
"LR

AAA = (EEB, XXX)
EEB = (XXX, ZZZ)
ZZZ = (EEB, XXX)
FFA = (FFB, XXX)
FFB = (FFC, FFC)
FFC = (FFZ, FFZ)
FFZ = (FFB, FFB)
XXX = (XXX, XXX)";
    let mut lines = raw_input.lines();

    let directions = lines.next().unwrap();

    // reverse to simplify start/end checks
    let to_id = |bytes: &[u8]| { bytes.iter().rev().fold(0, |sum, c| (sum << 5) | (*c as u16 & 0x1f)) };

    lines.next();
    let mut locations = lines.map(|line| {
        let bytes = line.as_bytes();
        let id = to_id(&bytes[0..3]);
        let left = to_id(&bytes[7..10]);
        let right = to_id(&bytes[12..15]);

        (id, left, right, TryInto::<[u8; 3]>::try_into(&bytes[0..3]).unwrap())
    }).collect::<Vec<_>>();

    // sort to get nodes ending with z last
    locations.sort_unstable_by_key(|(id,..)| *id);

    let map = locations.iter().fold(HashMap::<u16, u16>::new(), |mut map, (id,..)| {
        map.insert(*id, map.len() as u16);
        map
    });

    let make_vec = || { Vec::<u16>::with_capacity(locations.len()) };

    let (lefts, rights) = locations.iter()
        .fold((make_vec(), make_vec()), |(mut lefts, mut rights), (_, left, right, _)| {
            lefts.push(*map.get(left).unwrap());
            rights.push(*map.get(right).unwrap());
            (lefts, rights)
        });

    let last_a = locations.iter().take_while(|(.., bytes)| bytes[2] == 'A' as u8).count() as u16;
    let first_z = locations.iter().take_while(|(.., bytes)| bytes[2] != 'Z' as u8).count() as u16;
    let last_z = locations.len() as u16 - 1;

    println!("debug locations A {:X?} .. Z {:X?}", &locations[..last_a as usize], &locations[first_z as usize..]);

    let steps = directions.as_bytes().iter().cycle().scan(0, |loc, dir| {
        if *loc == last_z {
            None
        } else if *dir == 'L' as u8 {
            *loc = lefts[*loc as usize];
            Some(*loc)
        } else {
            *loc = rights[*loc as usize];
            Some(*loc)
        }
    }).count();

    println!("Took {steps} steps from AAA to ZZZ");

    // part 2

    let start_points: Vec<u16> = (0u16..).take(last_a as usize).collect::<Vec<_>>();


    println!("One cycle is {} steps", directions.len());
    println!("Found {} __A", start_points.len());

    let loops = start_points.iter().map(|start_point| {
        let mut cycles = HashMap::new();
        let mut possible_ends = Vec::new();
        let mut cycle_point = *start_point;
        let mut cycle_count = 0usize;

        let repeat = loop {
            if let Some(first) = cycles.insert(cycle_point, cycle_count) {
                break first..cycle_count;
            }

            cycle_point = directions.as_bytes().iter().enumerate().fold(cycle_point, |point, (i, dir)| {
                if point >= first_z {
                    possible_ends.push((cycle_count, i));
                }
                (if *dir == 'L' as u8 { &lefts } else  { &rights })[point as usize]
            });

            cycle_count += 1;
        };

        let factors = Factorization::run(repeat.len() as u32).factors;
        possible_ends = possible_ends.iter().skip_while(|(cycle, _)| *cycle < repeat.start).copied().collect::<Vec<_>>();

        println!("Loop {start_point} repeats {repeat:?}. Found {possible_ends:?} possible end points. Factors {factors:?}");

        (repeat, possible_ends)
    }).collect::<Vec<_>>();

    // turns out everything is a prime

    let total_cycles = loops.iter().map(|(range, _)| range.len()).product::<usize>();

    println!("Total cycles {total_cycles} => {} steps", total_cycles * directions.len());


    // Not bruteforce this time

    // let mut points = start_points.clone();
    //
    // let multi_steps = directions.as_bytes().iter().cycle().scan(points.as_mut_slice(), |points, dir| {
    //
    //     // println!("iter: {points:?}");
    //     let mut all_zs = true;
    //     for p in points.iter() {
    //         if *p < first_z {
    //             all_zs = false;
    //             break;
    //         }
    //     }
    //     if all_zs {
    //         println!("Success!!");
    //         return None;
    //     }
    //
    //     let table = if *dir == 0x4C { &lefts } else { &rights };
    //     for p in points.iter_mut() {
    //         *p = table[*p as usize];
    //     }
    //
    //     Some(())
    // }).count();
    //
    // println!("Batch move took {multi_steps} steps");

}