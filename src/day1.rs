use std::collections::HashMap;

use anyhow::Result;
use tokio::io::{ AsyncBufRead, AsyncRead, AsyncBufReadExt, BufReader };
use regex::Regex;

static NUM_MAP: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];
const REGEX: &str = "(one|two|three|four|five|six|seven|eight|nine)";

pub async fn day1(data: impl AsyncRead + Unpin) -> u32 {
    // Convert file to buffered line by line access
    let mut lines = {
        let reader = BufReader::new(data);
        reader.lines()
    };
    let mut nums = Vec::<u32>::new();
    let regex = Regex::new(REGEX).unwrap();
    let dictionary = HashMap::from(NUM_MAP);

    while let Ok(Some(line)) = lines.next_line().await {
        let line = {
            let line = line;let regex = &regex;let dictionary = &dictionary;
            let line = line.trim().to_owned();
            let mut line_output = line.clone();

            let matches = regex.find_iter(&line);
            for rmatch in matches {
                let matched_str = rmatch.as_str();
                line_output = line_output.replace(matched_str, dictionary[matched_str]);
            }

            line_output
        };
        nums.push({
            let line: &str = &line;
            let line_digits: Vec<char> = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect();

            let num_digits = format!(
                "{}{}",
                line_digits.first().unwrap_or(&'0'),
                line_digits.last().unwrap_or(&'0')
            );

            Ok(num_digits.parse::<u32>()?)
        }.unwrap());
    }

    nums.iter().sum()
}

#[cfg(test)]
mod test {
    use tokio::io::BufReader;

    const DAY1_SAMPLE_INPUT: &str = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

    const DAY1_PT2_INPUT: &str =
        r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    "#;

    const DAY1_PT2_TEST: &str = "nineight";

    /// result should be 142
    #[tokio::test]
    async fn day1() {
        let str_reader = BufReader::new(DAY1_SAMPLE_INPUT.as_bytes());

        let result = super::day1(str_reader).await;

        assert_eq!(142, result);
    }

    #[tokio::test]
    async fn day1_pt2() {
        let str_reader = BufReader::new(DAY1_PT2_INPUT.as_bytes());

        let result = super::day1(str_reader).await;

        assert_eq!(281, result);
    }

    #[tokio::test]
    async fn day1_pt2bis() {
        let str_reader = BufReader::new(DAY1_PT2_TEST.as_bytes());

        let result = super::day1(str_reader).await;

        assert_eq!(98, result);
    }
}
