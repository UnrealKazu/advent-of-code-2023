use {regex::Regex, std::fs};

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let input = read_lines("./puzzle.input");

    let decimals_regex = Regex::new(r"\d").expect("failed to compile regex");

    let mut sum = 0;
    for line in input {
        let matches: Vec<_> = decimals_regex
            .find_iter(&line)
            .map(|m| m.as_str())
            .collect();
        let num_str = format!("{}{}", matches[0], matches[matches.len() - 1]);
        let num: i32 = num_str.parse().unwrap();

        sum += num;
    }

    println!("The calibration result is {}", sum);
}
