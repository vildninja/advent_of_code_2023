#![feature(cmp_minmax)]

use itertools::{Itertools};
use std::cmp::{minmax, minmax_by};
use std::collections::{HashSet, VecDeque};

fn main() {

    let _debug_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    let _raw_input = include_str!("d22_input.txt");
    let input = _raw_input;

    let mut bricks = input.lines().map(|line| {
        let tokens = line.split(&[',','~']);
        let (p0, p1) = tokens.tuples()
            .map(|(x, y, z)|
                [
                    x.parse::<usize>().unwrap(),
                    y.parse::<usize>().unwrap(),
                    z.parse::<usize>().unwrap()
                ]).collect_tuple::<(_, _)>().unwrap();

        let [p0, p1] = minmax_by(p0, p1, |a, b| {
            a[0].cmp(&b[0])
                .then(a[1].cmp(&b[1]))
                .then(a[2].cmp(&b[2]))
        });

        (p0, p1)
    }).sorted_unstable_by_key(|([..,z], _)| *z).collect_vec();

    let (x_bounds, y_bounds) = bricks.iter()
        .fold(([usize::MAX, 0], [usize::MAX, 0]), |(x_bounds, y_bounds), (p0, p1)| {
            let [x_min, x_max] = minmax(p0[0], p1[0]);
            let [y_min, y_max] = minmax(p0[1], p1[1]);
            (
                [usize::min(x_bounds[0], x_min), usize::max(x_bounds[1], x_max)],
                [usize::min(y_bounds[0], y_min), usize::max(y_bounds[1], y_max)]
            )
        });

    println!("Bounds: {x_bounds:?} {y_bounds:?}"); // Bricks: {bricks:?}");

    let mut single_supporting_bricks = HashSet::new();

    // for part two
    // [supporting_me, supported_by_me]
    let mut supporting_bricks = vec![(vec![0usize; 0], vec![0usize; 0]); bricks.len()];

    let x_size = x_bounds[1] + 1;
    let y_size = y_bounds[1] + 1;
    let mut grid = vec![(0, None); x_size * y_size];
    bricks.iter_mut().enumerate().for_each(|(brick_index, (p0, p1))| {
        let indices = (0..2).find(|&i| p0[i] != p1[i])
            .map_or_else(|| vec![p0[0] + p0[1] * x_size], |index| {
                (0..).map_while(|i| {
                    let mut p = *p0;
                    p[index] += i;
                    if p[index] <= p1[index] { Some(p[0] + p[1] * x_size) }
                    else { None }
                }).collect_vec()
            });

        let drop_to_z = indices.iter().map(|&grid_index| grid[grid_index].0).max().unwrap() + 1;

        p1[2] -= p0[2] - drop_to_z;
        p0[2] = drop_to_z;

        let top_z = p1[2];

        enum SupportType {
            None,
            Single(usize),
            Multiple,
        }

        let mut supporters = SupportType::None;

        indices.iter().for_each(|&grid_index| {
            if grid[grid_index].0 == drop_to_z - 1 {
                if let Some(support) = grid[grid_index].1 {
                    supporters = match supporters {
                        SupportType::None => SupportType::Single(support),
                        SupportType::Single(cur) if cur == support => SupportType::Single(support),
                        _ => SupportType::Multiple,
                    };

                    // for part two
                    let supporting_me = &mut supporting_bricks[brick_index].0;
                    if !supporting_me.contains(&support) { supporting_me.push(support); }

                    let supported_by_other = &mut supporting_bricks[support].1;
                    if !supported_by_other.contains(&brick_index) { supported_by_other.push(brick_index); }
                }
            }
            grid[grid_index] = (top_z, Some(brick_index));
        });

        if let SupportType::Single(support) = supporters {
            single_supporting_bricks.insert(support);
        }
    });

    // println!("Fallen bricks: {bricks:?}");

    let unimportant_brick_count = bricks.len() - single_supporting_bricks.len();

    println!("Found {} supporting bricks out of {} total", single_supporting_bricks.len(), bricks.len());

    // 489 is too high
    // 477 correct (forgot to add upper z of vertical bars to height grid)
    println!("Removable brick count: {unimportant_brick_count}");

    // part two

    let drop_sum = (0..bricks.len()).map(|brick_index| {
        let mut dropped = HashSet::new();
        let mut queue = VecDeque::new();

        dropped.insert(brick_index);
        queue.push_back(brick_index);

        while let Some(cur_brick) = queue.pop_front() {
            let dropping_bricks = supporting_bricks[cur_brick].1.iter()
                .filter(|&dropping| !dropped.contains(dropping)
                    && supporting_bricks[*dropping].0.iter().all(|supporter| dropped.contains(supporter)))
                .copied().collect_vec();
            dropping_bricks.iter().for_each(|&dropping| if dropped.insert(dropping) {
                queue.push_back(dropping);
            });
        }

        println!("Disintegrating brick #{brick_index:4} will cause {} bricks to fall.", dropped.len() - 1);

        dropped.len() - 1
    }).sum::<usize>();

    println!("Sum of all disintegrated drops: {drop_sum}");
}