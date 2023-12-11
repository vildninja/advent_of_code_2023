fn main() {
    let raw_input = include_str!("d11_input.txt");

    const EXPANDED: i64 = 999999; // part one = 1, part two = 1000000

    let mut col_map = (0..raw_input.lines().next().unwrap().len()).map(|_| EXPANDED).collect::<Vec<_>>();
    let mut empty_rows = 0i64;
    let mut galaxies = raw_input.lines().enumerate().filter_map(|(row, line)| {
        let galaxies_on_line = line.as_bytes().iter().enumerate()
            .fold(Vec::<(i64, i64)>::new(), |mut found, (col, &c)| {
                if c == '#' as u8 {
                    found.push((col as i64, row as i64 + empty_rows));
                    col_map[col] = 0;
                }
                found
            });

        if galaxies_on_line.is_empty() {
            empty_rows += EXPANDED;
            None
        } else {
            Some(galaxies_on_line)
        }
    }).flatten().collect::<Vec<_>>();

    let mut col_price = 0i64;
    col_map.iter_mut().for_each(|price| {
        *price += col_price;
        col_price = *price + 1;
    });

    println!("col cost {col_map:?}");

    galaxies.iter_mut().for_each(|(col, _)| {
        *col = col_map[*col as usize];
    });

    println!("galacies {galaxies:?}");

    let dist_sum = galaxies.iter().enumerate().map(|(i, (c0, r0))| {
        galaxies[i + 1..].iter().fold(0, |sum, (c1, r1)| {
            sum + (c1 - c0).abs() + (r1 - r0).abs()
        })
    }).sum::<i64>();

    println!("Distances between all gallaxies {dist_sum}");
}
