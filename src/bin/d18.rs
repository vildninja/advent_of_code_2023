use itertools::{Itertools};


// #[derive(Default, Debug)]
// struct Scanner {
//     first_dir: u8,
//     last_dir: u8,
//     last_x: i32,
//     area: usize,
// }

#[derive(Copy, Clone, Debug)]
struct DigPlan(i64, i64, u8);

fn dig_area(plan: &[DigPlan]) -> usize {

    let walls = plan.iter()
        .scan((0i64, 0i64), |(x, y), &DigPlan(dx, dy, dir)| {
        let (old_x, old_y) = (*x, *y);// + dy.signum());
        *x += dx;
        *y += dy;

        if dx == 0 {
            Some((i64::min(old_y, *y)..=i64::max(old_y, *y), *x, dir))
        } else if dy == 0 {
            Some((i64::min(old_x, *x)..=i64::max(old_x, *x), *y, dir))
        } else { None }
    }).collect_vec();

    let (up_down, left_right) = walls.iter()
        .circular_tuple_windows::<(_, _)>()
        .scan(walls.last().unwrap().2,
              |last_dir, ((range, offset, dir), (_, _, next_dir))| {
                  let (start, end) = (*range.start(), *range.end());
                  let range = match (*last_dir, *dir, *next_dir) {
                      (b'L', b'D', b'R') => start + 1..=end - 1,
                      (_, b'D', b'R') => start..=end - 1,
                      (b'L', b'D', _) => start + 1..=end,
                      _ => start..=end,
                  };

                  // println!("Turn {} {} {}", *last_dir as char, *dir as char, *next_dir as char);

                  *last_dir = *dir;
                  Some((range, *offset, *dir))
              })
        .sorted_unstable_by_key(|(_, key, _)| *key)
        .partition::<Vec<_>, _>(|(_, _, dir)| { *dir == b'U' || *dir == b'D' });

    // println!("up_down: {up_down:?}");

    let rows_of_interest = left_right.iter()
        .map(|(_, offset, _)| *offset).unique().sorted_unstable().collect_vec();

    // println!("Rows of interest {rows_of_interest:?}");


    let line_area = |y: i64| {
        up_down.iter().filter(|(range, _, _)| range.contains(&y))
            .scan((b'\0', None), |(first_dir, start), (_, x, dir)| {
                if *first_dir == b'\0' {
                    *first_dir = *dir;
                }
                if *dir == *first_dir {
                    if start.is_none() {
                        *start = Some(*x);
                    }

                    Some(0usize)
                } else if let Some(x0) = start.take() {
                    Some((*x - x0 + 1) as usize)
                }
                else {
                    Some(0usize)
                }
            }).sum::<usize>()
    };

    let (area, _, _) = rows_of_interest.iter()
        .fold((0usize, i64::MIN, 0usize), |(mut area, last, last_area), &y| {
            if last < y {
                area += (y - 1 - last) as usize * last_area;
                area += line_area(y);
            }

            let last_area = line_area(y + 1);

            (area + last_area, y + 1, last_area)
        });

    println!("dig plan area {area}");

    return area;
}


fn main() {

    let _debug_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";


    let _raw_input = include_str!("d18_input.txt");

    let used_input = _raw_input;

    let dig_plan = used_input.lines().map(|line| {
        let mut words = line.split_ascii_whitespace();
        let dir = *words.next().unwrap().as_bytes().first().unwrap();
        let steps = words.next().unwrap().parse::<i64>().unwrap();

        match dir {
            b'U' => DigPlan(0, -steps, dir),
            b'D' => DigPlan(0, steps, dir),
            b'L' => DigPlan(-steps, 0, dir),
            b'R' => DigPlan(steps, 0, dir),
            _ => panic!(),
        }
    }).collect_vec();

    dig_area(&dig_plan);

    // let walls = dig_plan.iter()
    //     .scan((0i64, 0i64, 0u8), |(x, y, last_dir), &DigPlan(dx, dy, dir)| {
    //     let (old_x, old_y) = (*x + dx.signum(), *y);// + dy.signum());
    //     *x += dx;
    //     *y += dy;
    //
    //     Some((old_x..=*x - dx.signum(), old_y..=*y, dir))
    // }).map(|(mut x, mut y, dir)| {
    //     if x.end() < x.start() {
    //         x = *x.end()..=*x.start();
    //     }
    //     if y.end() < y.start() {
    //         y = *y.end()..=*y.start();
    //     }
    //     // println!("Wall x {x:?}, y {y:?}, {dir}");
    //     (x, y, dir)
    // }).collect_vec();
    //
    // let (width_range, height_range) = walls.iter()
    //     .fold((0i64..=0i64, 0i64..=0i64), |(mut x, mut y), (wall_x, wall_y, dir)| {
    //     (i64::min(*x.start(), *wall_x.start())..=i64::max(*x.end(), *wall_x.end()),
    //      i64::min(*y.start(), *wall_y.start())..=i64::max(*y.end(), *wall_y.end()))
    // });
    //
    // let width = (width_range.end() - width_range.start()) as usize + 2;
    // let height = (height_range.end() - height_range.start()) as usize + 1;
    // println!("Area covers {width_range:?} x {height_range:?}");
    // println!("Area covers {width} x {height}");
    //
    // let mut map = vec![b' '; width * height];
    // (width - 1..map.len()).step_by(width).for_each(|i| map[i] = b'\n');
    //
    // walls.iter().cloned().for_each(|(x_range, y_range, dir)| {
    //     x_range.for_each(|x| {
    //         y_range.clone().for_each(|y| {
    //             let index = (x - width_range.start()) as usize
    //                 + (y - height_range.start()) as usize * width;
    //             map[index] = dir;
    //         });
    //     });
    // });
    //
    // let _ = map.iter_mut().fold(0u8, |mut last, cur| {
    //     match *cur {
    //         b'\n' => { last = 0; },
    //         b' ' => {
    //             if last == b'U' {
    //                 *cur = b'.';
    //             }
    //         },
    //         b'A'..=b'Z' => { last = *cur; }
    //         _ => {}
    //     }
    //     last
    // });
    //
    // std::fs::write("d18_map.txt", &map).unwrap();
    //
    // let area = map.iter().filter(|b| **b >= b'.').count();
    // println!("Area covered {area}");

    // part 2

    let plan2 = used_input.lines().map(|line| {
        let words = line.split_ascii_whitespace();
        let color = &words.last().unwrap()[2..8];
        let steps = i64::from_str_radix(&color[..5], 16).unwrap();

        match color.as_bytes()[5] {
            b'0' => DigPlan(steps, 0, b'R'),
            b'1' => DigPlan(0, steps, b'D'),
            b'2' => DigPlan(-steps, 0, b'L'),
            b'3' => DigPlan(0, -steps, b'U'),
            _ => panic!(),
        }
    }).collect_vec();

    dig_area(&plan2);


    //     .scan((0i64, 0i64), |(x, y), (dx, dy, dir)| {
    //     let (old_x, old_y) = (*x, *y);// + dy.signum());
    //     *x += dx;
    //     *y += dy;
    //
    //     if dx == 0 {
    //         Some((i64::min(old_y, *y)..=i64::max(old_y, *y), *x, dir))
    //     } else if dy == 0 {
    //         Some((i64::min(old_x, *x)..=i64::max(old_x, *x), *y, dir))
    //     } else { None }
    // }).inspect(|val| {
    //     println!("Found wall {val:?}");
    // }).collect_vec();
    //
    // let (width_range, height_range) = walls2.iter()
    //     .fold((0i64..=0i64, 0i64..=0i64), |(mut x, mut y), (wall, _, dir)| {
    //         let axis = match *dir {
    //             'U' | 'D' => &mut y,
    //             'L' | 'R' => &mut x,
    //             _ => panic!(),
    //         };
    //         let old_start = *axis.start();
    //         let old_end = *axis.end();
    //         *axis = i64::min(old_start, *wall.start())..=i64::max(old_end, *wall.end());
    //
    //         (x, y)
    //     });
    //
    // let width = (width_range.end() - width_range.start()) as usize + 2;
    // let height = (height_range.end() - height_range.start()) as usize + 1;
    // println!("Area2 covers {width_range:?} x {height_range:?}");
    // println!("Area2 covers {width} x {height}");
    //
    // let (up_down, left_right) = walls2.iter()
    //     .circular_tuple_windows::<(_, _)>()
    //     .scan(walls2.last().unwrap().2,
    //         |last_dir, ((range, offset, dir), (_, _, next_dir))| {
    //             let (mut start, mut end) = (*range.start(), *range.end());
    //             let range = match (*last_dir, *dir, *next_dir) {
    //                 (_, 'D', 'R') => start..=end - 1,
    //                 ('L', 'D', _) => start + 1..=end,
    //                 _ => start..=end,
    //             };
    //
    //             println!("Turn {last_dir} {dir} {next_dir}");
    //
    //             *last_dir = *dir;
    //             Some((range, *offset, *dir))
    //     })
    //     .sorted_unstable_by_key(|(_, key, _)| *key)
    //     .partition::<Vec<_>, _>(|(_, _, dir)| { *dir == 'U' || *dir == 'D' });
    //
    // println!("up_down: {up_down:?}");
    //
    // let area2 = height_range.fold(0usize, |area, y| {
    //     let line_area = up_down.iter().filter(|(range, _, _)| range.contains(&y))
    //         .scan(('\0', None), |(first_dir, start), (_, x, dir)| {
    //             if *first_dir == '\0' {
    //                 *first_dir = *dir;
    //             }
    //             if *dir == *first_dir {
    //                 if start.is_none() {
    //                     *start = Some(*x);
    //                 }
    //
    //                 Some(0usize)
    //             } else if let Some(x0) = start.take() {
    //                 Some((*x - x0 + 1) as usize)
    //             }
    //             else {
    //                 Some(0usize)
    //             }
    //         }).sum::<usize>();
    //
    //     if y & 0xfffff == 0 {
    //         println!("Line area at {y} is {line_area}");
    //     }
    //
    //     line_area + area
    // });
    //
    // // 122109850949101 too low
    // println!("Color area {area2}");

    // println!("{}", unsafe { from_utf8_unchecked(&map) });
    //
    // let vertical_walls = walls.iter().filter(|(_, _, dir)| *dir == b'U' || *dir == b'D')
    //     .cloned()
    //     .map(|(x, mut y, dir)| (*x.start(), y, dir))
    //     .sorted_by(|(a_x, a_y, a_dir), (b_x, b_y, b_dir)| {
    //         a_x.cmp(b_x)
    //             // .then_with(|| )
    //             // .then_with(|| b_y.start().cmp(a_y.start()))
    //     }).collect_vec();
    //
    //
    // height_range.clone().for_each(|y| {
    //     let line_index = (y - height_range.start()) as usize * width;
    //     let line_area = vertical_walls.iter().filter(|(_, wall, _)| wall.contains(&y))
    //         .scan((0u8, 0usize, 0i32, 0u8),
    //               |(first_dir, area, last_x, last_dir), (x, _, dir)| {
    //
    //             if first_dir == 0 {
    //                 *first_dir = *dir;
    //                 *last_x = *x;
    //                 println!("Hit first wall {dir} as {x}, {y}");
    //             } else if dir == first_dir {
    //
    //             }
    //
    //
    //             else if dir != last_dir {
    //                 if *inside {
    //                     (*last_x + 1..*x).for_each(|x| {
    //                         let index = (x - width_range.start()) as usize + line_index;
    //                         map[index] = b'#';
    //                     })
    //                 }
    //                 *inside = !*inside;
    //                 *last_x = *x;
    //             }
    //             *last_dir = *dir;
    //
    //             Some(*area)
    //         }).last();
    //     println!("Line area {line_area:?}");
    // });

}