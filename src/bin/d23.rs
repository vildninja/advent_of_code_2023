use std::collections::{HashMap, VecDeque};
use itertools::{Itertools};
use crate::Direction::{East, North, South, West};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

fn main() {

    let _debug_input = b"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    let _raw_input = include_bytes!("d23_input.txt");
    let mut input = _raw_input.to_owned();

    // start out by building graph of intersection nodes with sized directional edges

    // add 1 for newline
    let width = input.iter().take_while(|b| !b.is_ascii_whitespace()).count() + 1;
    let height = input.len() / width;

    let offset_dir = |i: usize, dir: Direction| {
        match dir {
            North => i - width,
            South => i + width,
            West => i - 1,
            East => i + 1,
        }
    };

    let mut nodes = (1..height - 1).flat_map(|row| (row * width + 1..row * width + width - 1)
        .filter(|&i| input[i] == b'.' &&
            DIRS.iter().filter(|&&dir| input[offset_dir(i, dir)] != b'#').count() >= 3))
        .collect_vec();

    let start_i = 1usize;
    let end_i = input.len() - 3;

    nodes.insert(0, start_i);
    nodes.push(end_i);
    nodes.iter().for_each(|&i| input[i] = b'O');

    println!("{}", std::str::from_utf8(&input).unwrap());


    println!("Map is {width} x {height}, width {} nodes", nodes.len());



    let get_next_dir = |i: usize, dir: Direction| {
        match input[i] {
            // // uncomment section for part one
            // b'^' => if dir == South { None } else { Some(North) },
            // b'>' => if dir == West { None } else { Some(East) },
            // b'v' => if dir == North { None } else { Some(South) },
            // b'<' => if dir == East { None } else { Some(West) },
            b'#' => None,
            _ => {
                let exclude = match dir {
                    North => South,
                    South => North,
                    West => East,
                    East => West,
                };
                DIRS.iter().filter(|&&dir| dir != exclude).copied()
                    .find(|&next| input[offset_dir(i, next)] != b'#')
            }
        }
    };

    let edges = nodes.iter().take(nodes.len() - 1).map(|&node_index| {
        (
            node_index,
            (if node_index < width { &[South][..] } else { &DIRS[..] }).iter()
                .filter_map(|&start_dir| {

                    let mut index = offset_dir(node_index, start_dir);
                    let mut dir = get_next_dir(index, start_dir);

                    let mut count = 1usize;
                    while let Some(next) = dir {
                        count += 1;
                        index = offset_dir(index, next);

                        if input[index] == b'O' { break; }

                        dir = get_next_dir(index, next);
                    }

                    if dir.is_some() { Some((nodes.iter().position(|&i| i == index).unwrap(), index, count)) }
                    else { None }
                }).collect_vec()
        )
    }).collect::<HashMap<_,_>>();

    println!("Found edges: {edges:?}");

    // I cloned the edge map for part one, but too expensive for part two.
    // luckily only very few nodes in graph, less than 64, so using a bitmask instead
    let mut queue = VecDeque::new();
    queue.push_back((1usize, 0usize, 0u64));

    let mut result_paths = Vec::new();

    while let Some((node, steps, visited_mask)) = queue.pop_front() {
        if let Some(edges) = edges.get(&node) {
            edges.iter().for_each(|&(node_index, grid_index, count)| {
                if grid_index == end_i {
                    result_paths.push(steps + count);
                } else if visited_mask & (1 << node_index) == 0 {
                    queue.push_back((grid_index, steps + count, visited_mask | (1 << node_index)));
                }
            })
        }
    }

    result_paths.sort();

    // part one => 2298
    // part two => 6602
    println!("Found {} valid paths {:?}", result_paths.len(), result_paths.iter().rev().take(10).collect_vec());
}