use std::collections::HashMap;

const WORKING:u8 = b'.';
const BROKEN:u8 = b'#';

// one call per data index per group index
// (36*5 * 6*5) = 5400 max calls
//
//	??###????????????#??###????????????# 5,1,1,1,1,5,1,1,1,1
// 	55555...
// 	.55555..
// 	..55555.
// 	= 3 ways to reach
// 	________?????????#??###????????????# _,1,1,1,1,5,1,1,1,1
//
//
// 	55555.1.1...
// 	55555.1..1..
// 	55555.1...1.
// 	55555..1.1..
// 	55555..1..1.
// 	55555...1.1.
// 	.55555.1.1..
// 	.55555.1..1.
// 	.55555..1.1.
// 	..55555.1.1.
// 	= 10 ways to reach
// 	____________?????#??###????????????# _,_,_,1,1,5,1,1,1,1
//
//  let map<(data_id, group_id), count>
//  for index in data
//     	for ((_, gi), count) in map where data_id == index
//     		try set data[index] = .
//     			map[index + 1, gi] += count
//     		try set data[index] = groups[gi] + .
//     			map[index groups[gi].len + 1, group + 1] += count
//  let final_count = map[data end, groups end]

fn find_solutions(records: &[u8], groups: &[usize]) -> Option<usize> {
    // map of substeps
    // (record index, group index) => possible ways to get there
    let mut maps = (0..=records.len()).map(|_| HashMap::new()).collect::<Vec<_>>();
    maps[0].insert(0usize, 1usize);

    for i in 0..records.len() {
        let entries = maps[i].iter()
            .map(|(&group_index, &count)| (group_index, count)).collect::<Vec<_>>();

        for (group_index, count) in entries {

            // try add '.'
            if records[i] != BROKEN {
                let old_count = *maps[i + 1].get(&group_index).unwrap_or(&0);
                maps[i + 1].insert(group_index, old_count + count);

            }
            if let Some(&group) = groups.get(group_index) {

                let is_last_group = group_index + 1 >= groups.len();
                // if not last leave room for '.' after
                let group_len = if is_last_group { group } else { group + 1 };

                if i + group_len <= records.len() &&
                    records[i..i + group].iter().all(|&b| b != WORKING) {

                    if is_last_group || records[i + group] != BROKEN {
                        let old_count = *maps[i + group_len].get(&(group_index + 1)).unwrap_or(&0);
                        maps[i + group_len].insert(group_index + 1, old_count + count);
                    }
                }
            }
        }
    }

    let result = maps.last().and_then(|map| map.get(&groups.len()).copied());

    if result.is_none()
    {
        println!("Debug groups '{groups:?}' maps {:?}", records.iter().chain(b"0000")
            .map(|&b| b as char).zip(maps.iter()).collect::<Vec<_>>());
    }

    result
}

fn main() {
    let raw_input = include_str!("d12_input.txt");

    let sum = raw_input.lines().map(|line| {
        let mut words = line.split_ascii_whitespace();
        let records = words.next().unwrap().as_bytes();
        let groups = words.next().unwrap()
            .split(',').map(|g| g.parse::<usize>().unwrap()).collect::<Vec<_>>();


        if let Some(count) = find_solutions(&records, &groups) {
            println!("Found {count} for {line}");
            count
        } else {
            println!("No solutions for {line}");
            0usize
        }
    }).sum::<usize>();

    println!("Total solutions {sum}");

    // part 2

    let sum2 = raw_input.lines().map(|line| {
        let mut words = line.split_ascii_whitespace();
        let records = words.next().unwrap().as_bytes();
        let groups = words.next().unwrap()
            .split(',').map(|g| g.parse::<usize>().unwrap()).collect::<Vec<_>>();

        if let Some(count) = find_solutions(
            &records.iter().chain(b"?").cycle().take(records.len() * 5 + 4).copied().collect::<Vec<_>>(),
            &groups.iter().cycle().take(groups.len() * 5).copied().collect::<Vec<_>>()) {
            println!("Found {count} for {line} x5");
            count
        } else {
            println!("No solutions for {line} x5");
            0usize
        }
    }).sum::<usize>();

    println!("Total x5 solutions {sum2}");

}