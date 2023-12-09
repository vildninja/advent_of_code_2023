use std::collections::HashMap;

fn main() {
    let raw_input = include_str!("d08_input.txt");
    let mut lines = raw_input.lines();

    let directions = lines.next().unwrap();

    // let to_id = |bytes: &[u8]| { bytes.fold(0, |sum, c| (sum << 5) | (c as u16 & 0x1f)) };
    let to_id = |bytes: &[u8]| { bytes.iter().fold(0, |sum, c| (sum << 8) | (*c as u32)) };

    lines.next();
    let map = lines.map(|line| {
        let bytes = line.as_bytes();
        let id = to_id(&bytes[0..3]);
        let left = to_id(&bytes[7..10]);
        let right = to_id(&bytes[12..15]);

        (id, left, right)
    }).fold(HashMap::new(), |mut map, (id, left, right)| {
        map.insert(id, (left, right));
        map
    });


    let start = to_id("AAA".as_bytes());
    let dest = to_id("ZZZ".as_bytes());

    let steps = directions.chars().cycle().scan(start, |loc, dir| {
        if *loc == dest {
            None
        } else if let Some((left, right)) = map.get(loc) {
            *loc = if dir == 'L' { *left } else { *right };
            Some(*loc)
        } else { panic!("path not found! {loc:X}") }

    }).count();

    println!("Took {steps} steps from AAA to ZZZ");

    // part 2

    let a_bits = 'A' as u32 & 0x1f;
    let z_bits = 'Z' as u32 & 0x1f;

    let start_points = map.keys().filter_map(|id| {
        if (*id & 0x1f) == a_bits { Some(*id) }
        else {
            println!("Not match {id:X} = {:X}", *id & 0x1f1f1f);
            None
        }
    }).collect::<Vec<_>>();

    println!("Found {} start points __A", start_points.len());


    // Not bruteforce this time :( see d08_p2.rs
    let multi_steps = directions.chars().cycle().scan(start_points, |locs, dir| {
        if locs.iter().all(|id| (*id & 0x1f) == z_bits) {
            None
        } else if dir == 'L' {
            locs.iter_mut().for_each(|loc| *loc = map[loc].0);
            Some(())
        } else {
            locs.iter_mut().for_each(|loc| *loc = map[loc].1);
            Some(())
        }
    }).count();

    println!("Took {multi_steps} multi steps from __A to __Z");
}