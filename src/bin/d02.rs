use std::ops::Index;
use regex::Regex;

fn main() {
    let raw_input = include_str!("d02_input.txt");
    let lines = raw_input.split("\n");

    let num_cube_pattern = Regex::new(r"([0-9]+)\s([a-z]+)").unwrap();
    let game_num_pattern = Regex::new(r"Game ([0-9]+):").unwrap();


    let mut rgb = [
        12u32,
        13u32,
        14u32,
    ];

    let mut result_sum = 0;
    let mut power_sum = 0;

    for line in lines {

        if let Some(game_num_cap) = game_num_pattern.captures(line)
        {
            let game_num = game_num_cap.index(1).parse::<u64>().unwrap();
            let mut possible = true;

            // part 2
            let mut rgb_sum = [0u32; 3];

            for (_, [num, color]) in num_cube_pattern.captures_iter(line).map(|c| c.extract()) {
                let number = num.parse::<u32>().unwrap();
                if match color {
                        "red" => number > rgb[0],
                        "green" => number > rgb[1],
                        "blue" => number > rgb[2],
                        _ => true,
                    } {
                    println!("Game {game_num} has {number} {color} cubes.");
                    possible = false;
                    // break;
                }

                // part 2
                match color {
                    "red" => { rgb_sum[0] = rgb_sum[0].max(number); },
                    "green" => { rgb_sum[1] = rgb_sum[1].max(number); },
                    "blue" => { rgb_sum[2] = rgb_sum[2].max(number); },
                    _ => {},
                }
            }

            power_sum += rgb_sum[0] * rgb_sum[1] * rgb_sum[2];
            if possible {
                result_sum += game_num;
            }
        }
    }

    println!("Result sum: {result_sum}");
    println!("Power sum: {power_sum}");
}