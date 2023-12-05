use std::ops::Range;

fn main() {
    let raw_input = include_str!("d05_input.txt");
    let lines = &mut raw_input.lines();

    let to_i64 = |word: &str| word.parse::<i64>().ok();

    let seeds = lines.next().unwrap().split_whitespace().
        filter_map(to_i64).collect::<Vec<_>>();
    println!("Found seeds {seeds:?}");

    // let _skipped = lines.skip(1);
    // println!("Skipped {_skipped} lines");

    // let mut maps = Vec::new();

    // take_while(|&line| line.len() > 5).
    // let seed_to_soil = lines.skip_while(|line| line.is_empty()).filter_map(|line| {
    //     let numbers = line.split_whitespace().filter_map(to_i64).collect::<Vec<_>>();
    //
    //     println!("Line: '{line}'");
    //     if numbers.len() == 3 {
    //         println!("Found seed-to-soil {numbers:?}");
    //         Some((numbers[0]..numbers[0] + numbers[2], numbers[1]..numbers[1] + numbers[2]))
    //     }
    //     else if line.ends_with(" map:") {
    //         maps.push((line, Vec::new()));
    //         None
    //     }
    //     else {
    //         None
    //     }
    // }).collect::<Vec<_>>();

    let maps = lines.filter(|line| !line.is_empty()).
        fold(Vec::<(&str, Vec::<(Range<i64>, Range<i64>)>)>::new(), |mut maps, line| {
            let numbers = line.split_whitespace().filter_map(to_i64).collect::<Vec<_>>();

            // println!("Line: '{line}'");
            // if numbers.len() == 3 {
            if let [dst, src, len] = numbers[..] {
                if let Some((name, map)) = maps.last_mut() {
                    println!("Found {name} {numbers:?}");
                    map.push((dst..dst + len, src..src + len));
                    // map.push((numbers[0]..numbers[0] + numbers[2], numbers[1]..numbers[1] + numbers[2]));
                }
            }
            else if line.ends_with(" map:") {
                maps.push((line, Vec::<(Range<i64>, Range<i64>)>::new()));
            }

            maps
        });

    let mut locations = seeds.iter().map(|&seed| {
        maps.iter().fold(seed, |number, (_, map)| {
            if let Some((dst, src)) =
                map.iter().find(|(_, src)| src.contains(&number)) {
                number - src.start + dst.start
            }
            else { number }
        })
    }).collect::<Vec<_>>();

    locations.sort();

    println!("Nearest location is {} ... {locations:?}", locations.first().unwrap());

    // part 2
    let seed_ranges = seeds.iter().scan(Option::<i64>::None, |prev, &seed| {
        if let Some(start) = prev.take() {
            Some((start, seed))
        }
        else {
            *prev = Some(seed);
            Some((0, 0))
        }
    }).filter_map(|(start, size)| {
        if size > 0 { Some(start..start + size) }
        else { None }
    }).collect::<Vec<_>>();

    println!("Seed ranges: {seed_ranges:?}");

    let remap = |dst: &Range<i64>, src: &Range<i64>, range: Range<i64>| {
        (range.start - src.start + dst.start)..(range.end - src.start + dst.start)
    };

    let mut locations_2 = maps.iter().fold(seed_ranges, |ranges, (_, map)| {
        let mut remapped = Vec::<Range<i64>>::new();
        let mut unmapped_ranges = map.iter().fold(ranges, |unmapped, (dst, src)| {
            unmapped.iter().fold(Vec::new(), |mut unmapped, range| {
                if src.start >= range.end ||  src.end <= range.start {
                    // outside
                    unmapped.push(range.clone());
                    // Some(range.clone())
                }
                else if range.start >= src.start && range.end <= src.end {
                    // fully omitted
                    remapped.push(remap(dst, src, range.clone()));
                    // None
                }
                else if range.start < src.start && range.end > src.end {
                    // cut in three
                    unmapped.push(range.start..src.start);
                    remapped.push(remap(dst, src, src.clone()));
                    unmapped.push(src.end..range.end);
                    // panic!("cut in three");
                    // None
                }
                else if range.start < src.start {
                    // range overlaps src start
                    // fully omitted is handled
                    remapped.push(remap(dst, src, src.start..range.end));
                    unmapped.push(range.start..src.start);
                    // Some(range.start..src.start)
                }
                else if range.end > src.end {
                    // range overlaps src end
                    // fully omitted is handled
                    remapped.push(remap(dst, src, range.start..src.end));
                    unmapped.push(src.end..range.end);
                    // Some(src.end..range.end)
                }
                else {
                    unreachable!("unhandled overlap");
                    // None
                }
                unmapped
                })
            });
        remapped.append(&mut unmapped_ranges);
        remapped
    });

    locations_2.sort_by_key(|range| range.start);

    println!("Nearest location ranges {locations_2:?}");
}