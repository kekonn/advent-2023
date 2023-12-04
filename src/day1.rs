use std::{ collections::HashMap, ops::Range };

use tokio::io::{ AsyncRead, AsyncBufReadExt, BufReader };

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

pub async fn day1(data: impl AsyncRead + Unpin) -> u32 {
    // Convert file to buffered line by line access
    let mut lines = {
        let reader = BufReader::new(data);
        reader.lines()
    };

    let mut nums = Vec::<u32>::new(); // all numbers found on each line
    let dictionary = HashMap::from(NUM_MAP); // a dictionary that helps convert spelled numbers to digits

    while let Ok(Some(line)) = lines.next_line().await {
        // line per line handling
        let line = line.trim().to_owned();
        let mut output_line = line.clone();

        // find all spelled digits and their location
        let mut spelled_found = dictionary
            .keys()
            .filter_map(|k| line.find(*k).map(|s| (s..s + k.len(), *k)))
            .collect::<Vec<(Range<usize>, &str)>>();
        spelled_found.sort_by(|(i1, _), (i2, _)| i1.start.cmp(&i2.start));

        while let Some((location, word)) = spelled_found.pop() {
            let overlapping: Vec<&(Range<usize>, &str)> = spelled_found
                .iter()
                .filter(|(loc, _)| (location.contains(&loc.start) || location.contains(&loc.end)))
                .collect();

            let mut location = location;
            for (overlap, _) in overlapping {
                location = match (location.contains(&overlap.start), location.contains(&overlap.end)) {
                    (true, false) => location.start..overlap.start,
                    (false, true) => overlap.end..location.end,
                    _ => location,
                };
            }

            // We now have the minimum range to replace
            output_line.replace_range(location, dictionary[word]);
        }

        // select all characters that represent digits
        let line_digits: Vec<char> = output_line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
        // make string with first and last digit of current line
        let num_digits = format!(
            "{}{}",
            line_digits.first().unwrap_or(&'0'),
            line_digits.last().unwrap_or(&'0')
        );

        nums.push(num_digits.parse::<u32>().expect("num_digits should represent a valid number"));
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

    const DAY1_PT2_TEST: &str = "sevenineight";

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

        assert_eq!(78, result);
    }
}
