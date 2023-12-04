
fn main() {
    let raw_input = include_str!("d03_input.txt");
    let lines = raw_input.lines();

    let is_marker = |(i, c): (usize, char)| {
        if c == '.' || c.is_ascii_digit() || c.is_whitespace() { None }
        else { Some(i) }
    };

    let take_numbers = |markers: &[usize], line: &mut[u8]| {

        let mut result = 0;

        for &marker in markers {
            for m in (marker.max(1) - 1)..=marker + 1 {
                if line.len() > m && line[m].is_ascii_digit() {
                    let mut start = m;
                    while start > 0 && line[start - 1].is_ascii_digit() {
                        start -= 1;
                    }

                    result += line[start..].iter_mut().map_while(|c| {
                        if let Some(num) = (*c as char).to_digit(10) {
                            *c = '.' as u8;
                            Some(num)
                        }
                        else { None }
                    }).fold(0, | num, digit| {
                        digit + num * 10
                    });
                }
            }
        }

        result
    };

    let res = lines.scan((String::new(), Vec::<usize>::new()), | (old, old_markers), line| {
        let mut cur = line.to_owned();
        let markers = cur.chars().enumerate().filter_map(is_marker).collect::<Vec<_>>();

        let cur_old_num = take_numbers(&markers, unsafe {old.as_bytes_mut()});

        let cur_cur_num = take_numbers(&markers, unsafe {cur.as_bytes_mut()});
        let old_cur_num = take_numbers(&old_markers, unsafe {cur.as_bytes_mut()});

        println!("Cur marked {cur_old_num} on old and {cur_cur_num} on cur. Old marked {old_cur_num} on cur");

        *old = cur;
        *old_markers = markers;
        Some(cur_old_num + cur_cur_num + old_cur_num)
    }).sum::<u32>();

    println!("total marked {res}");

    // part 2
    // f it just assume same line length
    let empty = "............................................................................................................................................";

    let lines = raw_input.lines();

    let read_number = |line: &[u8], i: usize| {
        let mut start = i;
        while start > 0 && line[start - 1].is_ascii_digit() {
            start -= 1;
        }

        line[start..].iter().map_while(|c| (*c as char).to_digit(10)).fold(0, | num, digit| {
            digit + num * 10
        })
    };

    // Append line to end
    let empty_lines = empty.lines();
    // let more_lines = [&lines, &empty_lines];

    let res2: u32 = lines.chain(empty_lines).scan([empty, empty, empty], |old, line| {

        old[0] = old[1];
        old[1] = old[2];
        old[2] = line;

        let gears: u32 = old[1].chars().enumerate().filter_map(|(i, c)|
            if c == '*' { Some(i) } else { None }
        ).map(|i| {
            let (connections, gear_sum) = old.map(|line: &str| {
                let mut numbers = Vec::new();
                if line.as_bytes()[i].is_ascii_digit() {
                    numbers.push(read_number(&line.as_bytes(), i));
                }
                else {
                    if i + 1 < line.len() && line.as_bytes()[i + 1].is_ascii_digit() {
                        numbers.push(read_number(&line.as_bytes(), i + 1));
                    }
                    if i > 0 && line.as_bytes()[i - 1].is_ascii_digit() {
                        numbers.push(read_number(&line.as_bytes(), i - 1));
                    }
                }
                numbers
                // match line[range].chars().map(|c| c.is_ascii_digit()).collect::<[bool;3]>() {
                //     [_, true, _] => Some(vec![(line, i)]), // 1 at i
                //     [true, false, true] => Some(vec![(line, i - 1), (line, i + 1)]), // 2 at -1 and +1
                //     [true, _, _] => Some(vec![(line, i - 1)]), // 1 at -1
                //     [_, _, true] => Some(vec![(line, i + 1)]), // 1 at +1
                //     _ => None // none
                // }
            }).into_iter().flatten().fold((0, 1), |(count, sum), num| {
                (count + 1, sum * num)
            });

            if connections == 2 { gear_sum }
            else { 0 }
        }).sum();

        println!("found gears {gears}");


        Some(gears)
    }).sum();

    println!("found total gear sum {res2}");
}