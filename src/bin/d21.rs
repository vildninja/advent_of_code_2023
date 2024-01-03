use std::collections::{HashSet, VecDeque};
use itertools::{Itertools};

fn main() {

    let _debug_input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";



    let _raw_input = include_str!("d21_input.txt");
    let input = _raw_input;

    const STEP_COUNT: usize = 64;

    // include newline in width
    let width = input.find('\n').unwrap() + 1;
    let height = input.lines().count();
    let start_index = input.find('S').unwrap();

    println!("{width}x{height} S at {start_index} ({}, {})", start_index % width, start_index / width);

    let mut visited = input.bytes().map(|b| b != b'.' && b != b'S').collect_vec();

    let mut steps = VecDeque::new();
    steps.push_back((start_index, 0usize));

    let mut reachable_plots = 0usize;

    while let Some((index, step_count)) = steps.pop_front() {
        if visited[index] { continue; }
        visited[index] = true;

        if step_count % 2 == 0 {
            reachable_plots += 1;
        }

        if step_count < STEP_COUNT {
            if index > width { steps.push_back((index - width, step_count + 1)); }
            if index + width < visited.len() { steps.push_back((index + width, step_count + 1)); }
            if index % width > 0 { steps.push_back((index - 1, step_count + 1)); }
            if index % width + 1 < width { steps.push_back((index + 1, step_count + 1)); }
        }
    }

    println!("After {STEP_COUNT} steps {reachable_plots} plots can be reached");

    // part two

    // 26501365 % 131 == 65
    // 26501365 / 131 == 202300.496183
    // (202300 - 65) / 131 == 202300
    // straight Manhattan paths to all edges and corners from start
    // start is at 65, 65
    //
    // find cell types for steps == 2 * 131 + 65
    //     .A,       123        nw NN ne        #
    //   ./ # \,    14#53    nw NW ## NE ne    ###
    //  < # # # >   6###7    WW ## ## ## NE   #####
    //   'L # 7´    89#ab    sw SW ## SE se    ###
    //     'V´       8cb        sw SS se        #

    let map = input.bytes().filter_map(|b| {
        if b == b'\n' { None}
        else if b == b'.' || b == b'S' {Some(true) }
        else { Some(false) }
    }).collect_vec();
    let width = input.find('\n').unwrap();
    // width is still the same

    const TILE_STEPS: usize = 2;
    const TILE_WIDTH: usize = TILE_STEPS * 2 + 1;

    let max_steps = width * TILE_STEPS + 65;
    let start = (max_steps, max_steps, 0);

    let mut plot_counts = [0usize; TILE_WIDTH * TILE_WIDTH];
    let mut visited = HashSet::new();

    let mut steps = VecDeque::new();
    steps.push_back(start);

    while let Some((x, y, step_count)) = steps.pop_front() {
        if !map[x % width + (y % height) * width] { continue; }
        if !visited.insert((x, y)) { continue; }

        if step_count % 2 == 1 {
            plot_counts[x / width + (y / height) * TILE_WIDTH] += 1;
        }

        if step_count < max_steps {
            if x > 0 { steps.push_back((x - 1, y, step_count + 1)); }
            if x + 1< width * TILE_WIDTH { steps.push_back((x + 1, y, step_count + 1)); }
            if y > 0 { steps.push_back((x, y - 1, step_count + 1)); }
            if y + 1 < height * TILE_WIDTH { steps.push_back((x, y + 1, step_count + 1)); }
        }
    }

    // Plot counts [    0,     0,  1906, 11458,  1904,     0,     0,
    // 7 x 7            0,  1906, 13389, 15287, 13356,  1904,     0,
    //               1906, 13389, 15287, 15287, 15287, 13356,  1904,
    //              11485, 15287, 15287, 15287, 15287, 15287, 11449,
    //               1931, 13383, 15287, 15287, 15287, 13380,  1897,
    //                  0,  1931, 13383, 15287, 13380,  1897,     0,
    //                  0,     0,  1931, 11476,  1897,     0,     0]

    // Plot counts [    0,  1906, 11458,  1904,     0,
    // 5 x 5         1906, 13389, 15287, 13356,  1904,
    //              11485, 15287, 15287, 15287, 11449,
    //               1931, 13383, 15287, 13380,  1897,
    //                  0,  1931, 11476,  1897,     0]


    // every second step
    // Plot counts [   0,  964, 5756,  965,    0,
    // 5 x 5         964, 6703, 7650, 6690,  965,
    //              5764, 7650, 7637, 7650, 5747,
    //               984, 6698, 7650, 6694,  964,
    //                 0,  984, 5755,  964,    0]
    //
    // Plot counts [   0,    0,  942, 5702,  939,    0,    0,
    // 7 x 7           0,  942, 6686, 7637, 6666,  939,    0,
    //               942, 6686, 7637, 7650, 7637, 6666,  939,
    //              5721, 7637, 7650, 7637, 7650, 7637, 5702,
    //               947, 6685, 7637, 7650, 7637, 6686,  933,
    //                 0,  947, 6685, 7637, 6686,  933,    0,
    //                 0,    0,  947, 5721,  933,    0,    0]

    println!("Plot counts {plot_counts:?}");

    let full_length = 202300_usize;
    let full_tiles = dbg!(4 * (full_length * (full_length - 1)) / 2 + 1);
    let (even_tiles, odd_tiles) = (1..full_length)
        .fold((1, 0), |(even, odd), i| {
            let count = i * 4;
            if i % 2 == 0 { (even + count, odd) }
            else { (even, odd + count) }
        });

    println!("Tiles even {even_tiles}, odd {odd_tiles}, total {}", even_tiles + odd_tiles);

    let small_nw = dbg!(plot_counts[1] * full_length);
    let small_ne = dbg!(plot_counts[3] * full_length);
    let small_sw = dbg!(plot_counts[21] * full_length);
    let small_se = dbg!(plot_counts[23] * full_length);

    let big_nw = dbg!(plot_counts[6] * (full_length - 1));
    let big_ne = dbg!(plot_counts[8] * (full_length - 1));
    let big_sw = dbg!(plot_counts[16] * (full_length - 1));
    let big_se = dbg!(plot_counts[18] * (full_length - 1));

    let north = dbg!(plot_counts[2]);
    let south = dbg!(plot_counts[22]);
    let west = dbg!(plot_counts[10]);
    let east = dbg!(plot_counts[14]);

    let center = plot_counts[12] * full_tiles;
    let evens = plot_counts[12] * even_tiles;
    let odds = plot_counts[13] * odd_tiles;


    println!("small_nw {small_nw}");
    println!("small_ne {small_ne}");
    println!("small_sw {small_sw}");
    println!("small_se {small_se}");
    println!("big_nw {big_nw}");
    println!("big_ne {big_ne}");
    println!("big_sw {big_sw}");
    println!("big_se {big_se}");
    println!("north {north}");
    println!("south {south}");
    println!("west {west}");
    println!("east {east}");
    println!("~ center {center}");
    println!("evens {evens}");
    println!("odds {odds}");

    let sum = small_nw
        + small_ne
        + small_sw
        + small_se
        + big_nw
        + big_ne
        + big_sw
        + big_se
        + north
        + south
        + west
        + east
        + evens
        + odds;

    // 1251256001183247 is too high
    println!("Total reachable plots {sum}");
    //  625628021226274 YUUUSSSS!

}