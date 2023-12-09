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

fn predict_next_value(nums: Vec<i32>) -> i32 {
    let last = *nums.last().unwrap();
    let end_val = get_end_value(nums);

    last + end_val
}

fn get_end_value(nums: Vec<i32>) -> i32 {
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
        let last = *diffs.last().unwrap();
        let num = get_end_value(diffs);

        return last + num;
    } else {
        return *diffs.first().unwrap();
    }
}

fn main() {
    if let Ok(lines) = read_lines("./puzzle.input") {
        let mut sum = 0;

        for line in lines {
            sum += predict_next_value(
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

        let next1 = predict_next_value(seq1);
        let next2 = predict_next_value(seq2);
        let next3 = predict_next_value(seq3);

        assert_eq!(next1, 18);
        assert_eq!(next2, 28);
        assert_eq!(next3, 68);
    }

    #[test]
    fn test_correct_puzzle_prediction() {
        let seq1 = vec![
            2, 0, -2, -4, -6, -8, -10, -12, -14, -16, -18, -20, -22, -24, -26, -28, -30, -32, -34,
            -36, -38,
        ];

        let next1 = predict_next_value(seq1);

        assert_eq!(next1, -40);
    }
}
