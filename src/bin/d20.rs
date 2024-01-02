use std::collections::{HashMap, VecDeque};
use itertools::{Itertools};
use crate::ModuleKind::{Broadcast, Conjunction, FlipFlop};

#[derive(Debug)]
enum ModuleKind {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}

#[derive(Debug)]
struct Module {
    destinations: Vec<String>,
    kind: ModuleKind,
}

fn main() {

    let _debug_input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    let _debug_input_2 = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";


    let _raw_input = include_str!("d20_input.txt");

    let input = _raw_input;

    let mut reverse = HashMap::<String, Vec<String>>::new();

    let mut modules = input.lines().map(|line| {
        let kind = if line.starts_with('%') {
            FlipFlop(false)
        } else if line.starts_with('&') {
            Conjunction(HashMap::new())
        } else {
            Broadcast
        };

        let name = line.chars()
            .skip_while(|c| !c.is_ascii_alphabetic())
            .take_while(|c| c.is_ascii_alphabetic())
            .collect::<String>();

        let destinations = line.split_once("->").unwrap().1
            .split(',').map(|dest| dest.trim().to_string()).collect_vec();

        destinations.iter().for_each(|dest| {
            let from = reverse.entry(dest.clone()).or_default();
            from.push(name.clone());
        });

        (
            name,
            Module {
                destinations,
                kind,
            }
        )
    }).collect::<HashMap<_, _>>();

    modules.iter_mut().for_each(|(name, module)| {
        if let Conjunction(input_map) = &mut module.kind {
            if let Some(mut inputs) = reverse.remove(name) {
                input_map.extend(inputs.drain(..).map(|input| (input, false)));
            }
        }
    });

    println!("Modules: {modules:?}");

    let mut part_two_count = 0usize;
    let mut part_two_last_toggle = HashMap::new();
    const PART_ONE_COUNT: usize = 1000;
    const PART_TWO_COUNT: usize = 10000;

    let (_low_count, _high_count) = [("broadcaster".to_string(), "button".to_string(), false)]
        .iter().cycle().cloned().take(PART_TWO_COUNT)
        .fold((0usize, 0usize), |(mut low_count, mut high_count), first_pulse| {

            let mut pulses = VecDeque::new();
            pulses.push_back(first_pulse);

            part_two_count += 1;

            while let Some((to, from, pulse)) = pulses.pop_front() {

                if !pulse && to == "rx" {
                    println!("rx received low after {part_two_count} button presses");
                }

                match pulse {
                    true => { high_count += 1; },
                    false => { low_count += 1; },
                }

                if high_count + low_count < 50 {
                    println!("{:4} {from} -{pulse}-> {to}", high_count + low_count);
                }

                if let Some(module) = modules.get_mut(&to) {
                    if let Some(new_pulse) = match &mut module.kind {
                        FlipFlop(state) if pulse == false => {
                            *state = !*state;
                            Some(*state)
                        },
                        Conjunction(inputs) if to == "tg" => {
                            let old = inputs.insert(from.clone(), pulse).unwrap();
                            if old != pulse {
                                if let Some(last_count) = part_two_last_toggle
                                    .insert(from.clone(), part_two_count) {
                                    println!("rx received {pulse} from {from} after {} #{part_two_count}",
                                             part_two_count - last_count);
                                }
                            }
                            // high if all low
                            Some(inputs.values().any(|pulse| !pulse))
                        },
                        Conjunction(inputs) => {
                            inputs.insert(from, pulse);
                            // high if all low
                            Some(inputs.values().any(|pulse| !pulse))
                        },
                        Broadcast => Some(pulse),
                        _ => None,
                    } {
                        pulses.extend(module.destinations.iter()
                            .map(|dest| (dest.clone(), to.clone(), new_pulse)));
                    }
                }
            }

            if part_two_count == PART_ONE_COUNT {
                println!("Part one final pulse count low {low_count}, high {high_count}, product {}",
                         low_count * high_count);
            }

            (low_count, high_count)
        });


    // Part two
    // f it, the log shows inputs to tg loops
    // tf after 3923
    // db after 3929
    // vq after 4007
    // ln after 4091
    // all primes, all loops start at 0
    println!("Part two rx receives low after {}", 3923usize * 3929 * 4007 * 4091);
}