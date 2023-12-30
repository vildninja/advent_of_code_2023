use std::collections::HashMap;
use std::ops::Range;
use itertools::{Itertools};
use regex::Regex;
use crate::Rule::{Always, Less, More};
use crate::Target::{Accept, Reject, Goto};

#[derive(Copy, Clone, Debug)]
enum Rule {
    More(usize, usize),
    Less(usize, usize),
    Always,
}

#[derive(Copy, Clone, Debug)]
enum Target {
    Accept,
    Reject,
    Goto(usize),
}

#[derive(Clone, Debug)]
struct Workflow {
    name: String,
    rules: Vec<(Rule, Target)>,
}

fn main() {

    let _debug_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";


    let _raw_input = include_str!("d19_input.txt");

    let input = _raw_input;

    let workflow_pattern = Regex::new(r"([a-z]+)\{(.+)}").unwrap();
    let rule_pattern = Regex::new(r"([a-z])([<>])([0-9]+):([a-zA-Z]+)").unwrap();

    let byte_to_xmas = |id: u8| {
        match id {
            b'x' => 0,
            b'm' => 1,
            b'a' => 2,
            b's' => 3,
            _ => panic!()
        }
    };

    let workflow_id = input.lines()
        .take_while(|line| line.len() > 0 && line.as_bytes().first().unwrap().is_ascii_alphabetic())
        .enumerate().map(|(i, line)| {
            let name = line.split("{").next().unwrap().to_owned();
            (name, i)
        }).collect::<HashMap<_, _>>();

    let workflows = input.lines()
        .take_while(|line| line.len() > 0 && line.as_bytes().first().unwrap().is_ascii_alphabetic())
        .map(|line| {
            let captures = workflow_pattern.captures(line).unwrap();
            let name = captures[1].to_owned();
            let rules = captures[2].split(',')
                .map(|rule| {
                    if rule == "A" { (Always, Accept) }
                    else if rule == "R" { (Always, Reject) }
                    else if let Some(rule_cap) = rule_pattern.captures(rule) {
                        if &rule_cap[2] == ">" {
                            (
                                More(byte_to_xmas(rule_cap[1].as_bytes()[0]), rule_cap[3].parse::<usize>().unwrap()),
                                if &rule_cap[4] == "A" { Accept }
                                else if &rule_cap[4] == "R" { Reject }
                                else { Goto(workflow_id[&rule_cap[4]]) }
                            )
                        } else {
                            (
                                Less(byte_to_xmas(rule_cap[1].as_bytes()[0]), rule_cap[3].parse::<usize>().unwrap()),
                                if &rule_cap[4] == "A" { Accept }
                                else if &rule_cap[4] == "R" { Reject }
                                else { Goto(workflow_id[&rule_cap[4]]) }
                            )
                        }
                    } else {
                        // println!("unmatched rule: {rule}");
                        (Always, Goto(workflow_id[rule]))
                    }
                }).collect_vec();

            Workflow {
                name,
                rules,
            }
        }).collect_vec();

    assert_eq!(workflow_id.len(), workflows.len());

    let part_pattern = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)").unwrap();

    let mut parts = input.lines()
        .filter(|line| line.starts_with('{'))
        .map(|line| {
            let captures = part_pattern.captures(line).unwrap();
            (
                workflow_id["in"],
                [
                    captures[1].parse::<usize>().unwrap(),
                    captures[2].parse::<usize>().unwrap(),
                    captures[3].parse::<usize>().unwrap(),
                    captures[4].parse::<usize>().unwrap(),
                ]
            )
        }).collect::<Vec<_>>();

    let mut accepted = Vec::new();

    while !parts.is_empty() {
        let (wf, part) = parts.pop().unwrap();

        for &(rule, target) in &workflows[wf].rules {
            if match rule {
                More(id, val) => part[id] > val,
                Less(id, val) => part[id] < val,
                Always => true,
            } {
                match target {
                    Accept => { accepted.push(part); }
                    Reject => {}
                    Goto(next) => { parts.push((next, part)); }
                }
                break;
            }
        }
    }

    println!("Accepted parts: {accepted:?}");

    let sum = accepted.iter().flatten().sum::<usize>();

    println!("Accepted sum: {sum}");

    // part 2

    let mut accepted_ranges = Vec::new();
    let mut _rejected_ranges = Vec::new();
    let mut part_ranges = Vec::new();
    part_ranges.push((workflow_id["in"], [1usize..4001, 1usize..4001, 1usize..4001, 1usize..4001]));

    let split_range = |range: &Range<usize>, split| {
        let above = usize::max(range.start, split)..range.end;
        let below = range.start..usize::min(range.end, split);
        (below, above)
    };

    while !part_ranges.is_empty() {
        let (wf, part) = part_ranges.pop().unwrap();

        let _ = workflows[wf].rules.iter().fold(part.clone(), |mut remainder, &(rule, target)| {
            let mut accepted = remainder.clone();
            match rule {
                More(id, val) => {
                    let (failed, passed) = split_range(&remainder[id], val + 1);
                    accepted[id] = passed;
                    remainder[id] = failed;
                },
                Less(id, val) => {
                    let (passed, failed) = split_range(&remainder[id], val);
                    accepted[id] = passed;
                    remainder[id] = failed;
                },
                Always => {
                    accepted = remainder;
                    remainder = [0usize..0, 0usize..0, 0usize..0, 0usize..0];
                },
            };

            if accepted.iter().all(|range| range.len() > 0) {
                match target {
                    Accept => { accepted_ranges.push(accepted); }
                    Reject => { _rejected_ranges.push(accepted); }
                    Goto(next) => { part_ranges.push((next, accepted)); }
                }
            }

            remainder
        });

    }



    let combinations = accepted_ranges.iter()
        .map(|ranges| ranges.iter()
            .map(|range| range.len()).product::<usize>())
        .sum::<usize>();

    println!("Accepted ranges {accepted_ranges:?}");
    println!("Accepted ranges {}", accepted_ranges.len());
    println!("Accepted combinations {combinations}");


}