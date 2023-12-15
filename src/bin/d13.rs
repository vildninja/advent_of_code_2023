
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

fn find_mirror_line(values: &Vec<u64>) -> Option<usize> {
    values.iter().enumerate().skip(1)
        .filter_map(|(i, &val)| {
            if values[i - 1] == val {
                // found repeated value
                Some(i)
            } else { None }
        }).filter(|&i| {
        // test against reversed
        values[i..].iter().zip(values[..i].iter().rev())
            .all(|(a, b)| a == b)
    }).next()
}
fn find_smudged_mirror_line(values: &Vec<u64>) -> Option<usize> {
    (1..values.len()).filter(|&i| {
        // test against reversed
        values[i..].iter().zip(values[..i].iter().rev())
            .fold_while(false, |smudge, (a, b)| {
                if !smudge && (a ^ b).is_power_of_two() {
                    Continue(true)
                } else if a == b { Continue(smudge) }
                else { Done(false) }
            }).into_inner()
    }).next()
}

fn main() {
    let raw_input = include_str!("d13_input.txt");

    let areas = raw_input.split("\n\n").map(|area| {
        area.lines()
            .fold((Vec::new(), Vec::new()) , |(mut cols, mut rows), line| {
                if cols.len() < line.len() {
                    cols.resize(line.len(), 0u64);
                }

                let row_i = rows.len();
                let row = line.bytes().enumerate()
                    .fold(0u64, |mut row,(col_i, b)| {
                        if b == b'#' {
                            row |= 1 << col_i;
                            cols[col_i] |= 1 << row_i;
                        }
                        row
                    });
                rows.push(row);

                (cols, rows)
            })
    }).collect::<Vec<_>>();

    let mirror_sum = areas.iter().map(|(cols, rows)| {

        if let Some(mirror) = find_mirror_line(cols) {
            println!("Found mirror line at col {mirror}");
            mirror
        } else if let Some(mirror) = find_mirror_line(rows) {
            println!("Found mirror line at row {mirror}");
            mirror * 100
        } else {
            println!("Failed to find mirror line:\n\tCols {cols:?}\n\tRows {rows:?}");
            0
        }
    }).sum::<usize>();

    println!("Mirror sum {mirror_sum}");

    // part 2
    let smudged_sum = areas.iter().map(|(cols, rows)| {

        if let Some(mirror) = find_smudged_mirror_line(cols) {
            println!("Found smudged mirror line at col {mirror}");
            mirror
        } else if let Some(mirror) = find_smudged_mirror_line(rows) {
            println!("Found smudged mirror line at row {mirror}");
            mirror * 100
        } else {
            println!("Failed to find smudged mirror line:\n\tCols {cols:?}\n\tRows {rows:?}");
            0
        }
    }).sum::<usize>();

    println!("Smudged mirror sum {smudged_sum}");

}