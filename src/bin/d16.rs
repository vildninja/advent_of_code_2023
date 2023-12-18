use itertools::Itertools;

const NORTH: u8 = 0x01;
const SOUTH: u8 = 0x02;
const WEST: u8 = 0x04;
const EAST: u8 = 0x08;

fn light_up(entry: (usize, u8), width: usize, map: &[u8]) -> usize {

    let mut visited = vec![0u8; map.len()];

    // not really a queue
    let mut queue = Vec::new();
    queue.push(entry);

    let next_index = |index: usize, dir: u8| {
        match dir {
            NORTH if index >= width => Some(index - width),
            SOUTH if index + width < map.len() => Some(index + width),
            WEST if index % width > 0 => Some(index - 1),
            EAST if index % width + 1 < width => Some(index + 1),
            _ => None,
        }
    };

    'node_loop: while let Some((mut index, dir)) = queue.pop() {

        if visited[index] & dir == dir {
            continue;
        }

        while map[index] == b'.' {
            // bi-directional should produce same result.
            visited[index] |= if dir < WEST { NORTH | SOUTH } else { WEST | EAST };

            index = if let Some(i) = next_index(index, dir) { i } else { continue 'node_loop; };
        }

        if visited[index] & dir == 0 {
            visited[index] |= dir;
            match map[index] {
                b'-' => {
                    visited[index] |= EAST | WEST;
                    if let Some(i) = next_index(index, WEST) { queue.push((i, WEST)); }
                    if let Some(i) = next_index(index, EAST) { queue.push((i, EAST)); }
                },
                b'|' => {
                    visited[index] |= NORTH | SOUTH;
                    if let Some(i) = next_index(index, NORTH) { queue.push((i, NORTH)); }
                    if let Some(i) = next_index(index, SOUTH) { queue.push((i, SOUTH)); }
                },
                b'/' => {
                    let next_dir = match dir {
                        NORTH => EAST,
                        SOUTH => WEST,
                        WEST => SOUTH,
                        EAST => NORTH,
                        _ => 0,
                    };
                    if let Some(i) = next_index(index, next_dir) { queue.push((i, next_dir)); }
                },
                b'\\' => {
                    let next_dir = match dir {
                        NORTH => WEST,
                        SOUTH => EAST,
                        WEST => NORTH,
                        EAST => SOUTH,
                        _ => 0,
                    };
                    if let Some(i) = next_index(index, next_dir) { queue.push((i, next_dir)); }
                },
                _ => {},
            }
        }
    }

    let visited_cells = visited.iter().filter(|&&v| v != 0).count();
    visited_cells
}

fn main() {
    let raw_input = include_str!("d16_input.txt");

    let width = raw_input.lines().next().unwrap().len();
    let height = raw_input.lines().count();

    println!("area is {width} x {height}");

    let map = raw_input.lines().flat_map(move |line| line.as_bytes())
        .copied().collect_vec();

    let visited_cells = light_up((0, EAST), width, &map);

    println!("Visited {visited_cells} cells.");

    // part 2

    let max_visited = (0..width).map(|i| (i, SOUTH))
        .chain((map.len() - width..map.len()).map(|i| (i, NORTH)))
        .chain((0..map.len()).step_by(width).map(|i| (i, EAST)))
        .chain((width - 1..map.len()).step_by(width).map(|i| (i, WEST)))
        .map(|(i, dir)| {
            let visited_cells = light_up((i, dir), width, &map);
            println!("Visited {visited_cells} cells, from {i}:{dir:X}");
            visited_cells
        }).max();

    println!("Max visited cells: {max_visited:?}.");
}