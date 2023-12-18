use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use itertools::Itertools;

const SOUTH: u8 = 0x01;
const WEST: u8 = 0x02;
const EAST: u8 = 0x04;
const NORTH: u8 = 0x08;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Step {
    index: u16,
    dir: u8,
    straight: u8,
    cost: u32,
}


impl PartialOrd<Self> for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // min
            .then_with(|| self.index.cmp(&other.index)) // max
            .then_with(|| other.straight.cmp(&self.straight)) // min
            .then_with(|| other.dir.cmp(&self.dir)) // min
    }
}


fn next_index(index: usize, dir: u8, width: usize, len: usize) -> Option<usize> {
    match dir {
        NORTH if index >= width => Some(index - width),
        SOUTH if index + width < len => Some(index + width),
        WEST if index % width > 0 => Some(index - 1),
        EAST if index % width + 1 < width => Some(index + 1),
        _ => None,
    }
}
fn main() {

    let _debug_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let _alt_debug_input = "111111111111
999999999991
999999999991
999999999991
999999999991";

    let _raw_input = include_str!("d17_input.txt");

    let used_input = _raw_input;

    let width = used_input.lines().next().unwrap().len();
    let height = used_input.lines().count();

    println!("area is {width} x {height}");

    let map = used_input.lines().flat_map(move |line| line.as_bytes())
        .map(|&b| b - b'0').collect_vec();

    // there are 12 ways to visit a tile ^v<> * 3

    // // to reset step counter (>>>^v>) cost at most 18 more than a regular step would
    // // we can do dijkstra, and cut branches if price is more than 18 higher than cheapest visit
    // // const WORST_CASE: u32 = 18;

    const MIN_STRAIGHT: u8 = 4;
    const MAX_STRAIGHT: u8 = 10;

    let mut queue = BinaryHeap::new();
    let mut cost_map = vec![1000000u32; map.len()];

    // only visit each combo once: (index, dir, straight)
    let mut visisted = HashSet::new();

    queue.push(Step {
        straight: 1,
        dir: EAST,
        index: 1,
        cost: map[1] as u32,
    });
    queue.push(Step {
        straight: 1,
        dir: SOUTH,
        index: width as u16,
        cost: map[width] as u32,
    });

    let mut iter_counter = 0;

    while let Some(step) = queue.pop() {
        iter_counter += 1;

        if iter_counter % 10000 == 0 {
            println!("Traversing {iter_counter} iters. found {} nodes", visisted.len());
        }

        let index = step.index as usize;
        // if cost_map[index] + WORST_CASE < step.cost {
        //     continue;
        // }

        if index + 1 == map.len() &&
            step.straight >= MIN_STRAIGHT {
            break;
        }

        let mut push_next = |dir: u8, straight: u8| {
            if let Some(i) = next_index(index, dir, width, map.len()) {
                let cost = step.cost + map[i] as u32;
                if /*cost_map[i] + WORST_CASE >= cost &&*/
                    // visisted.insert((i << 16) as u32 | ((straight as u32) << 8) | dir as u32) {
                    visisted.insert((i as u16, straight, dir)) {

                    if straight >= MIN_STRAIGHT {
                        cost_map[i] = u32::min(cost_map[i], cost);
                    }

                    queue.push(Step {
                        straight,
                        dir,
                        index: i as u16,
                        cost,
                    });
                }
            }
        };

        if step.straight < MAX_STRAIGHT {
            push_next(step.dir, step.straight + 1);
        }

        if step.straight >= MIN_STRAIGHT {
            match step.dir {
                NORTH | SOUTH => {
                    push_next(WEST, 1);
                    push_next(EAST, 1);
                },
                WEST | EAST => {
                    push_next(NORTH, 1);
                    push_next(SOUTH, 1);
                },
                _ => {}
            }
        }
    }

    println!("Traversed graph in {iter_counter} iters. Found {} nodes. Cost is {}",
             visisted.len(), cost_map.last().unwrap());
}