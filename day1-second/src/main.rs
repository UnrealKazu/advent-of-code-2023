use std::{fs, time::Instant};

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn match_number(substr: &str) -> &str {
    if substr.contains('1') || substr.contains("one") {
        return "1";
    } else if substr.contains('2') || substr.contains("two") {
        return "2";
    } else if substr.contains('3') || substr.contains("three") {
        return "3";
    } else if substr.contains('4') || substr.contains("four") {
        return "4";
    } else if substr.contains('5') || substr.contains("five") {
        return "5";
    } else if substr.contains('6') || substr.contains("six") {
        return "6";
    } else if substr.contains('7') || substr.contains("seven") {
        return "7";
    } else if substr.contains('8') || substr.contains("eight") {
        return "8";
    } else if substr.contains('9') || substr.contains("nine") {
        return "9";
    }

    "0"
}

fn main() {
    let input = read_lines("./puzzle.input");

    let now = Instant::now();

    let mut sum = 0;
    for line in input {
        let mut first_num = "";
        let mut second_num = "";
        for sub1 in 0..line.len() {
            let num = match_number(&line[0..sub1 + 1]);

            if num != "0" {
                first_num = num;
                break;
            }
        }

        for sub2 in (0..line.len() + 1).rev() {
            let num = match_number(&line[sub2..line.len()]);

            if num != "0" {
                second_num = num;
                break;
            }
        }

        let num_str = format!("{}{}", first_num, second_num);
        let num: i32 = num_str.parse().unwrap();

        //println!("Original: {}\nNumber: {}", line, num);
        sum += num;
    }

    println!("Duration: {}", now.elapsed().as_millis());

    println!("The calibration result is {}", sum);
}
