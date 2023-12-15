use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let raw_input = include_str!("d14_input.txt");


    let height = raw_input.lines().count();
    let width = raw_input.lines().next().unwrap().len();

    println!("Map is {width} x {height}");

    let north_weight = raw_input.lines().enumerate()
        .scan(vec![height + 1; width], |last_weights, (row, line)| {
            Some(line.bytes().enumerate().map(|(col, b)| {
                if b == b'O' {
                    last_weights[col] -= 1;
                    last_weights[col]
                } else {
                    if b == b'#' {
                        last_weights[col] = height - row;
                    }
                    0
                }
            }).sum::<usize>())
        }).sum::<usize>();

    println!("Weight sum = {north_weight}");

    // part 2
    part_two(width, height, raw_input);
}

// not great with borrow checker
// struct Square {
//     index: u16,
//     vertical: u8,
//     horizontal: u8,
// }

fn part_two(width: usize, height: usize, raw_input: &str) {

    // ok I might have done some optimizations, before realizing that this was just
    // day 8 "Haunted Wasteland" again

    // pad area
    let width = width + 2;
    let height = height + 2;
    let map_size = width * height;
    let mut sq_map = vec![0u16; map_size];
    let mut north_map = vec![0u16; map_size];
    let mut south_map = vec![0u16; map_size];
    let mut west_map = vec![0u16; map_size];
    let mut east_map = vec![0u16; map_size];

    // index 0 should never be accessed
    let squares = (0..width - 1) // north border
        .chain((1..height - 1).map(|i| i * width)) // west border
        .chain((1..height - 1).map(|i| i * width + width - 1)) // east border
        .chain(map_size - width + 1..map_size - 1) // south border
        .chain(raw_input.lines().enumerate().flat_map(|(row, line)| {
            let row = row + 1;
            line.bytes().enumerate().filter_map(move |(col, b)| {
                if b == b'#' { Some(col + 1 + row * width) }
                else { None }
            })
        }))
        .sorted_unstable().enumerate()
        .map(|(i, map_i)| {
            sq_map[map_i] = i as u16;
            north_map[map_i] = i as u16;
            south_map[map_i] = i as u16;
            west_map[map_i] = i as u16;
            east_map[map_i] = i as u16;

            map_i as u16
    }).collect::<Vec<_>>();

    // stones landed on each square
    let mut vertical_stacks = squares.iter().map(|&i| (i, 0u16)).collect::<Vec<_>>();
    let mut horizontal_stacks = squares.iter().map(|&i| (i, 0u16)).collect::<Vec<_>>();

    squares.iter().enumerate().skip(1).for_each(|(i, &sq)| {
        (sq as usize..map_size)
            .take_while(|&map_i| sq_map[map_i] == 0 || sq_map[map_i] == i as u16)
            .for_each(|map_i| { west_map[map_i] = i as u16; });
        (1..=sq as usize).rev()
            .take_while(|&map_i| sq_map[map_i] == 0 || sq_map[map_i] == i as u16)
            .for_each(|map_i| { east_map[map_i] = i as u16; });
        (sq as usize..map_size).step_by(width)
            .take_while(|&map_i| sq_map[map_i] == 0 || sq_map[map_i] == i as u16)
            .for_each(|map_i| { north_map[map_i] = i as u16; });
        (1..=sq as usize).rev().step_by(width)
            .take_while(|&map_i| sq_map[map_i] == 0 || sq_map[map_i] == i as u16)
            .for_each(|map_i| { south_map[map_i] = i as u16; });
    });

    // println!("West squares:\n{west_map:?}");

    raw_input.lines().enumerate().for_each(|(row, line)| {
        let row = row + 1;
        line.bytes().enumerate().filter_map(move |(col, b)| {
            if b == b'O' { Some(col + 1 + row * width) }
            else { None }
        }).for_each(|map_i| {
            vertical_stacks[north_map[map_i] as usize].1 += 1;
        })
    });


    let test_sum = vertical_stacks.iter().map(|&(index, stones)| {
        if stones > 0 {
            let row_score = height - 2 - index as usize / width;
            (0..stones as usize).map(|i| row_score - i).sum::<usize>()
        } else { 0 }
    }).sum::<usize>();

    println!("part 2: p1 sum: {test_sum}");

    let mut last_five = [0u32; 5];

    let mut cycle_map = HashMap::new();

    for i in 0.. {
        if i > 0 {
            // from east to north
            horizontal_stacks.iter_mut().filter(|(_, num)| *num > 0)
                .for_each(|(index, num)| {
                    for n in 1..=*num as usize {
                        vertical_stacks[north_map[*index as usize - n] as usize].1 += 1;
                    }
                    *num = 0;
                });
        }

        // from north to west
        vertical_stacks.iter_mut().filter(|(_, num)| *num > 0)
            .for_each(|(index, num)| {
                for n in 1..=*num as usize {
                    horizontal_stacks[west_map[*index as usize + n * width] as usize].1 += 1;
                }
                *num = 0;
            });

        // from west to south
        horizontal_stacks.iter_mut().filter(|(_, num)| *num > 0)
            .for_each(|(index, num)| {
                for n in 1..=*num as usize {
                    vertical_stacks[south_map[*index as usize + n] as usize].1 += 1;
                }
                *num = 0;
            });

        // from south to east
        vertical_stacks.iter_mut().filter(|(_, num)| *num > 0)
            .for_each(|(index, num)| {
                for n in 1..=*num as usize {
                    horizontal_stacks[east_map[*index as usize - n * width] as usize].1 += 1;
                }
                *num = 0;
            });

        last_five[4] = last_five[3];
        last_five[3] = last_five[2];
        last_five[2] = last_five[1];
        last_five[1] = last_five[0];
        last_five[0] = horizontal_stacks.iter().map(|&(index, stones)| {
            if stones > 0 {
                stones as u32 * (height - 1 - index as usize / width) as u32
            } else { 0 }
        }).sum::<u32>();

        if let Some(last_i) = cycle_map.insert(last_five, i) {
            let cycle_size = i - last_i;
            let remaining = 999999999 - i;
            let mod_sync = remaining % cycle_size;

            println!("possible cycle {i} -> {last_i} = {cycle_size}. {remaining} % {cycle_size} = {mod_sync}");
            if mod_sync == 0 {
                break;
            }
        }
    }

    let sum = horizontal_stacks.iter().map(|&(index, stones)| {
        if stones > 0 {
            stones as usize * (height - 1 - index as usize / width)
        } else { 0 }
    }).sum::<usize>();

    println!("Part 2: {sum}");

}