use itertools::{FoldWhile, Itertools};
use itertools::FoldWhile::Done;


fn main() {
    let raw_input = include_str!("d15_input.txt");

    let hash_sum = raw_input.lines().flat_map(|line| line.split(','))
        .map(|step| {
            step.bytes().fold(0u32, |hash, b| ((hash + b as u32) * 17) & 0xff)
        }).sum::<u32>();

    println!("Hash sum {hash_sum}");

    // part 2

    let mut lens_boxes = (0..256).map(|_| Vec::<(&str, i32)>::new()).collect_vec();

    raw_input.lines().flat_map(|line| line.split(',')).for_each(|step| {
        let (i, hash) = step.bytes().fold_while((0usize, 0usize), |(i, hash), b| {
            if b.is_ascii_alphabetic() {
                FoldWhile::Continue((i + 1, ((hash + b as usize) * 17) & 0xff))
            } else { Done((i, hash))}
        }).into_inner();

        let label = &step[..i];
        let lens_box = &mut lens_boxes[hash];

        if step.as_bytes()[i] == b'=' {
            let val = step[i + 1..].parse::<i32>().unwrap();

            if let Some((_, v)) = lens_box.iter_mut().find(|(l, _)| *l == label) {
                *v = val;
            } else {
                lens_box.push((label, val));
            }
        } else if step.as_bytes()[i] == b'-' {
            lens_box.retain(|(l, _)| *l != label);
        }
    });

    let lens_sum = lens_boxes.iter().enumerate().map(|(box_i, lenses)| {
        lenses.iter().enumerate().map(|(lens_i, &(_, val))| {
            (box_i + 1) * (lens_i + 1) * val as usize
        }).sum::<usize>()
    }).sum::<usize>();

    println!("Lens sum {lens_sum}");
}