use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

use regex::Regex;

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

#[derive(PartialEq, Debug)]
enum Op {
    Lt,
    Gt,
}

#[derive(PartialEq, Debug, Clone)]
enum State {
    Accepted,
    Rejected,
    Passed(String),
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_state: State,
}

struct Rule {
    prop: char,
    op: Op,
    value: u32,
    result: State,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_workflow(flow_raw: String) -> Workflow {
    let name_index = flow_raw.find('{').unwrap();
    let name = &flow_raw[0..name_index];

    let mut workflows = Workflow {
        name: name.to_string(),
        rules: vec![],
        default_state: State::Accepted,
    };

    let r = Regex::new(r"([xmas])(<|>)([0-9]+):([a-zA-Z]+)").unwrap();

    let mut flows_it = flow_raw.split(',').peekable();
    while let Some(flow) = flows_it.next() {
        if flows_it.peek().is_none() {
            // final one, which is always only a State, so this becomes the default
            workflows.default_state = match &flow[0..flow.len() - 1] {
                "A" => State::Accepted,
                "R" => State::Rejected,
                v => State::Passed(String::from(v)),
            };
            break;
        }

        let caps = r.captures(flow).unwrap();

        let prop = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let op = match caps.get(2).unwrap().as_str().chars().next().unwrap() {
            '<' => Op::Lt,
            '>' => Op::Gt,
            _ => panic!("Unexpected operation"),
        };
        let value: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let result = match caps.get(4).unwrap().as_str() {
            "A" => State::Accepted,
            "R" => State::Rejected,
            v => State::Passed(String::from(v)),
        };

        workflows.rules.push(Rule {
            prop,
            op,
            value,
            result,
        })
    }

    workflows
}

fn parse_part(part_raw: String) -> Part {
    let r = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();

    let caps = r.captures(&part_raw).unwrap();

    Part {
        x: caps.get(1).unwrap().as_str().parse().unwrap(),
        m: caps.get(2).unwrap().as_str().parse().unwrap(),
        a: caps.get(3).unwrap().as_str().parse().unwrap(),
        s: caps.get(4).unwrap().as_str().parse().unwrap(),
    }
}

fn parse_input(path: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        let mut workflow_mode = true;
        for raw_line in lines {
            let line = raw_line.unwrap();

            if line.is_empty() {
                // with the empty line we switch from parsing workflows
                // to parsing parts
                workflow_mode = false;
                continue;
            }

            if workflow_mode {
                let wf = parse_workflow(line);
                workflows.insert(wf.name.clone(), wf);
            } else {
                parts.push(parse_part(line));
            }
        }
    }

    (workflows, parts)
}

fn get_accepted_parts_rating(workflows: HashMap<String, Workflow>, parts: Vec<Part>) -> u32 {
    let mut sum = 0;

    for part in parts {
        let mut result = State::Passed(String::from("in"));

        // simple closure to copy a State
        let result_clone = |result: &State| -> State {
            match result {
                State::Accepted => State::Accepted,
                State::Rejected => State::Rejected,
                State::Passed(str_val) => State::Passed(str_val.clone()),
            }
        };

        // closure to do the actual operation on the given value
        let cond_check = |part_val: u32, rule: &Rule| -> Result<State, bool> {
            if rule.op == Op::Gt {
                if part_val > rule.value {
                    return Ok(result_clone(&rule.result));
                }
            } else if part_val < rule.value {
                return Ok(result_clone(&rule.result));
            }

            Err(false)
        };

        // as long as the part has not reached either Accepted or Rejected state,
        // we continue following the workflows
        while let State::Passed(ref n) = &result {
            let workflow = workflows.get(n).expect("Expected a workflow, got nothing");

            let mut hit = false;

            for rule in &workflow.rules {
                if let Ok(new_result) = match rule.prop {
                    'x' => cond_check(part.x, rule),
                    'm' => cond_check(part.m, rule),
                    'a' => cond_check(part.a, rule),
                    's' => cond_check(part.s, rule),
                    _ => panic!("Unexpected operation"),
                } {
                    result = new_result;
                    hit = true;
                    break;
                }
            }

            if !hit {
                // no rules matched, so we fall back to the default rule
                result = result_clone(&workflow.default_state);
            }
        }

        // only for accepted parts do we sum all its internal values
        if result == State::Accepted {
            sum += part.x + part.m + part.a + part.s;
        }
    }

    sum
}

fn main() {
    let (workflows, parts) = parse_input("./puzzle.input");

    let sum = get_accepted_parts_rating(workflows, parts);

    println!("Sum of accepted parts is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_should_generate_rule() {
        let flow_raw = String::from("px{a<2006:qkq,m>2090:A,rfg}");

        let flow = parse_workflow(flow_raw);

        assert_eq!(flow.name, "px");
        assert_eq!(flow.rules.len(), 2);

        let r1 = flow.rules.get(0).unwrap();
        let r2 = flow.rules.get(1).unwrap();

        assert_eq!(r1.prop, 'a');
        assert_eq!(r1.op, Op::Lt);
        assert_eq!(r1.value, 2006);
        assert_eq!(r1.result, State::Passed(String::from("qkq")));

        assert_eq!(r2.prop, 'm');
        assert_eq!(r2.op, Op::Gt);
        assert_eq!(r2.value, 2090);
        assert_eq!(r2.result, State::Accepted);

        assert_eq!(flow.default_state, State::Passed(String::from("rfg")));
    }

    #[test]
    fn test_parse_part_should_generate_part() {
        let part_raw = String::from("{x=787,m=2655,a=1222,s=2876}");

        let part = parse_part(part_raw);

        assert_eq!(part.x, 787);
        assert_eq!(part.m, 2655);
        assert_eq!(part.a, 1222);
        assert_eq!(part.s, 2876);
    }
}
