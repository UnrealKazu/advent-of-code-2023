use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn predict_prev_value(nums: Vec<i32>) -> i32 {
    let first = *nums.first().unwrap();
    let start_val = get_start_value(nums);

    first - start_val
}

fn get_start_value(nums: Vec<i32>) -> i32 {
    let mut consistent_diff = true;

    let mut diffs: Vec<i32> = Vec::new();

    let mut nums_it = nums.iter().peekable();

    while let Some(&num) = nums_it.next() {
        if let Some(&next) = nums_it.peek() {
            let diff = next - num;

            if !diffs.is_empty() && *diffs.first().unwrap() != diff {
                consistent_diff = false;
            }

            diffs.push(diff);
        }
    }

    if !consistent_diff {
        let first = *diffs.first().unwrap();
        let num = get_start_value(diffs);

        return first - num;
    } else {
        return *diffs.first().unwrap();
    }
}

fn main() {
    if let Ok(lines) = read_lines("./puzzle.input") {
        let mut sum = 0;

        for line in lines {
            sum += predict_prev_value(
                line.unwrap()
                    .split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect(),
            );
        }

        println!("Total sum is {}", sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_prediction() {
        let seq1 = vec![0, 3, 6, 9, 12, 15];
        let seq2 = vec![1, 3, 6, 10, 15, 21];
        let seq3 = vec![10, 13, 16, 21, 30, 45];

        let prev1 = predict_prev_value(seq1);
        let prev2 = predict_prev_value(seq2);
        let prev3 = predict_prev_value(seq3);

        assert_eq!(prev1, -3);
        assert_eq!(prev2, 0);
        assert_eq!(prev3, 5);
    }
}
