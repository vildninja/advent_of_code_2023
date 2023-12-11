const NORTH_BIT: u8 = 0x10;
const SOUTH_BIT: u8 = 0x20;
const EAST_BIT: u8 = 0x40;
const WEST_BIT: u8 = 0x80;
const VISITED_BIT: u8 = 0x01;

const DIR_MASK:u8 = 0xf0;

const PIPE_NS: u8 = '|' as u8;
const PIPE_EW: u8 = '-' as u8;
const PIPE_NE: u8 = 'L' as u8;
const PIPE_NW: u8 = 'J' as u8;
const PIPE_SE: u8 = 'F' as u8;
const PIPE_SW: u8 = '7' as u8;

const START_CHAR: u8 = 'S' as u8;

const fn flip_dir(dir: u8) -> u8 {
    match dir & DIR_MASK {
        NORTH_BIT => SOUTH_BIT,
        SOUTH_BIT => NORTH_BIT,
        EAST_BIT => WEST_BIT,
        WEST_BIT => EAST_BIT,
        _ => 0,
    }
}

const fn next_dir(dir: u8, pipe: u8) -> u8 {
    pipe & DIR_MASK & !flip_dir(dir)
}

fn enter(dir: &mut u8, pipe: &mut u8) -> bool {
    if (flip_dir(*dir) & *pipe) != 0 {
        if *pipe & VISITED_BIT == 0 {
            *dir = next_dir(*dir, *pipe);
            *pipe = *pipe | VISITED_BIT;
            true
        } else { false }
    } else { false }
}


const fn dir_to_index(dir: u8, index: usize, width: usize) -> usize {
    match dir & DIR_MASK {
        NORTH_BIT => index - width,
        SOUTH_BIT => index + width,
        EAST_BIT => index + 1,
        WEST_BIT => index - 1,
        _ => index,
    }
}

fn main() {
    let raw_input = include_str!("d10_input.txt");


    let width = raw_input.lines().next().unwrap().len();
    let height = raw_input.lines().count();

    let mut start = None;

    let mut grid = raw_input.lines().enumerate()
        .map(|(row, line)| {
            line.as_bytes().iter().enumerate().map(|(col, b)| {
                // convert and strip pipes leading out of bounds
                match *b {
                    PIPE_NS => if row > 0 && row < height - 1 { NORTH_BIT | SOUTH_BIT } else { 0 },
                    PIPE_EW => if col < width - 1 && col > 0 { EAST_BIT | WEST_BIT } else { 0 },
                    PIPE_NE => if row > 0 && col < width - 1 { NORTH_BIT | EAST_BIT } else { 0 },
                    PIPE_NW => if row > 0 && col > 0 { NORTH_BIT | WEST_BIT } else { 0 },
                    PIPE_SE => if row < height - 1 && col < width - 1 { SOUTH_BIT | EAST_BIT } else { 0 },
                    PIPE_SW => if row < height - 1 && col > 0 { SOUTH_BIT | WEST_BIT } else { 0 },
                    START_CHAR => {
                        start = Some((col, row));
                        VISITED_BIT
                    },
                    _ => 0,
                }
            }).collect::<Vec<_>>()
        }).flatten().collect::<Vec<_>>();


    let start_index = {
        let (col, row) = start.unwrap();
        col + row * width
    };

    let start_indices = [NORTH_BIT, SOUTH_BIT, EAST_BIT, WEST_BIT].iter().copied().filter_map(|mut dir| {
        let index = dir_to_index(dir, start_index, width);
        let old_dir = dir;
        if enter(&mut dir, &mut grid[index]) {
            grid[start_index] |= old_dir;
            Some((index, dir))
        } else { None }
    }).collect::<Vec<_>>();

    println!("Starting at {start:?} -> {start_indices:?}");

    let mut indices: [(usize, u8); 2] = start_indices.try_into().unwrap();

    for count in 1.. {
        if indices.iter_mut().map_while(|(index, dir)| {
            *index = dir_to_index(*dir, *index, width);
            if enter(dir, &mut grid[*index]) { Some(()) } else { None }
        }).count() == 0 {
            println!("Loop joined after {count} steps");
            break;
        }
    }


    // part 2

    let mut inside = false;
    let mut last_turn = None;
    let area_map = grid.iter().map(|&tile| {
        if tile & VISITED_BIT == 0 {
            if inside { 'I' } else { '.' }
        } else {
            let tile = tile & DIR_MASK;
            if tile == NORTH_BIT | SOUTH_BIT {
                inside = !inside;
                if inside { '>' } else { '<' }
            } else if tile == EAST_BIT | WEST_BIT {
                '-'
            } else {
                if let Some(turn) = last_turn.take() {
                    if (turn ^ tile) & (NORTH_BIT | SOUTH_BIT) != 0 {
                        inside = !inside;
                    }
                } else {
                    last_turn = Some(tile);
                }
                if inside { '>' } else { '<' }
            }
        }
    }).collect::<String>();

    println!("{area_map}");

    let area = area_map.chars().filter(|&c| c == 'I').count();

    println!("Enclosed area {area}");
}