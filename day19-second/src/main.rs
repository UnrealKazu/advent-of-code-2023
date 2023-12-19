use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    vec,
};

use regex::Regex;

#[derive(Debug)]
struct Part {
    state: State,
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
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

fn parse_input(path: &str) -> HashMap<String, Workflow> {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    if let Ok(lines) = read_lines(path) {
        for raw_line in lines {
            let line = raw_line.unwrap();

            if line.is_empty() {
                break;
            }

            let wf = parse_workflow(line);
            workflows.insert(wf.name.clone(), wf);
        }
    }

    workflows
}

fn get_possible_distinct_combinations(workflows: HashMap<String, Workflow>) -> u64 {
    let start = Part {
        state: State::Passed(String::from("in")),
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let mut possible_parts = vec![start];
    let mut nrof_combinations: u64 = 0;

    // simple closure to clone a State
    let state_clone = |state: &State| -> State {
        match state {
            State::Accepted => State::Accepted,
            State::Rejected => State::Rejected,
            State::Passed(str_val) => State::Passed(str_val.clone()),
        }
    };

    // closure to check the operator and apply changes to the current part,
    // as well as to potentially return a new (split off) part
    let cond_check = |part_state: &mut State,
                      part_val: &mut (u32, u32),
                      rule: &Rule|
     -> Option<((u32, u32), State)> {
        if rule.op == Op::Gt {
            // op is >
            if part_val.0 > rule.value {
                // this part fully covers the boundary, so we can follow the move
                *part_state = state_clone(&rule.result);
                return None;
            } else if part_val.0 <= rule.value && part_val.1 > rule.value {
                // split the current part into two. We update the current one so that
                // it matches the part above the boundary, and create a new one that
                // covers the part below the boundary
                let new_part_val = (part_val.0, rule.value);
                let new_part_state = state_clone(part_state);

                // modify the remainder of the current part
                *part_val = (rule.value + 1, part_val.1);
                *part_state = state_clone(&rule.result);

                return Some((new_part_val, new_part_state));
            }
        } else {
            // op is <
            if part_val.1 < rule.value {
                // this part fully covers the boundary, so we can follow the move
                *part_state = state_clone(&rule.result);
            } else if part_val.0 < rule.value && part_val.1 >= rule.value {
                // split current part into two
                let new_part_val = (rule.value, part_val.1);
                let new_part_state = state_clone(part_state);

                // modify the remainder of the current part
                *part_val = (part_val.0, rule.value - 1);
                *part_state = state_clone(&rule.result);

                return Some((new_part_val, new_part_state));
            }
        }

        // no split off part was created
        None
    };

    while let Some(mut part) = possible_parts.pop() {
        if let State::Passed(workflow_state) = &part.state {
            let workflow = workflows
                .get(workflow_state)
                .expect("Expected a workflow, got nothing");

            let old_state = workflow_state.clone();

            for rule in &workflow.rules {
                match rule.prop {
                    'x' => {
                        if let Some(new_part) = cond_check(&mut part.state, &mut part.x, rule) {
                            // we have to create a new Part
                            possible_parts.push(Part {
                                state: new_part.1,
                                x: new_part.0,
                                m: part.m,
                                a: part.a,
                                s: part.s,
                            })
                        }
                    }
                    'm' => {
                        if let Some(new_part) = cond_check(&mut part.state, &mut part.m, rule) {
                            // we have to create a new Part
                            possible_parts.push(Part {
                                state: new_part.1,
                                x: part.x,
                                m: new_part.0,
                                a: part.a,
                                s: part.s,
                            })
                        }
                    }
                    'a' => {
                        if let Some(new_part) = cond_check(&mut part.state, &mut part.a, rule) {
                            // we have to create a new Part
                            possible_parts.push(Part {
                                state: new_part.1,
                                x: part.x,
                                m: part.m,
                                a: new_part.0,
                                s: part.s,
                            })
                        }
                    }
                    's' => {
                        if let Some(new_part) = cond_check(&mut part.state, &mut part.s, rule) {
                            // we have to create a new Part
                            possible_parts.push(Part {
                                state: new_part.1,
                                x: part.x,
                                m: part.m,
                                a: part.a,
                                s: new_part.0,
                            })
                        }
                    }
                    _ => panic!("Unexpected operator"),
                }
            }

            if let State::Passed(new_state) = &part.state {
                if old_state == *new_state {
                    // the state has not changed, so none of the rules applied,
                    // hence, we fall back to the default rule
                    part.state = state_clone(&workflow.default_state);
                }
            }

            // push the current part back into the vec
            possible_parts.push(part);
        } else if let State::Accepted = &part.state {
            //println!("{:#?}", part);
            // this is an accepted part, so we are done with it
            // count all the part combinations by multiplication
            let mut part_combination = (part.x.1 as u64 - part.x.0 as u64) + 1;
            part_combination *= (part.m.1 as u64 - part.m.0 as u64) + 1;
            part_combination *= (part.a.1 as u64 - part.a.0 as u64) + 1;
            part_combination *= (part.s.1 as u64 - part.s.0 as u64) + 1;

            if nrof_combinations == 0 {
                nrof_combinations = part_combination;
            } else {
                nrof_combinations += part_combination;
            }
        }
    }

    nrof_combinations
}

fn main() {
    let workflows = parse_input("./example.input");

    let nrof = get_possible_distinct_combinations(workflows);

    println!("Nrof distinct possible combinations is {}", nrof);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut flows: HashMap<String, Workflow> = HashMap::new();

        let wf = Workflow {
            name: String::from("in"),
            rules: vec![],
            default_state: State::Accepted,
        };

        flows.insert(String::from("in"), wf);

        let nrof = get_possible_distinct_combinations(flows);

        assert_eq!(nrof, u64::pow(4000, 4));
    }
}
