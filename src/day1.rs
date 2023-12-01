use anyhow::Result;
use tokio::io::{AsyncBufRead, AsyncRead, AsyncBufReadExt, BufReader};

static NUM_MAP: [(&str, u32); 9] = [("one", 1), ("two", 2),("three", 3),("four",4),("five",5),("six",6),("seven",7),("eight",8),("nine",9)];

pub async fn day1(data: impl AsyncRead + Unpin) -> u32 {
    // Convert file to buffered line by line access
    let mut lines = to_lines(data);
    let mut nums = Vec::<u32>::new();

    while let Ok(Some(line)) = lines.next_line().await {
        nums.push(line_to_num(&line).unwrap());
    }

    nums.iter().sum()
}

fn to_lines(data: impl AsyncRead + Unpin) -> tokio::io::Lines<impl AsyncBufRead + Unpin> {
    let reader = BufReader::new(data);
    reader.lines()
}

fn line_to_num(line: &str) -> Result<u32> {
    let line_digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

    let num_digits = format!("{}{}", line_digits.first().unwrap_or(&'0'), line_digits.last().unwrap_or(&'0'));

    Ok(num_digits.parse::<u32>()?)
}

#[cfg(test)]
mod test {
    use tokio::io::BufReader;

    const DAY1_SAMPLE_INPUT: &str = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

    /// result should be 142
    #[tokio::test]
    async fn day1() {
        let str_reader = BufReader::new(DAY1_SAMPLE_INPUT.as_bytes());

        let result = super::day1(str_reader).await;

        assert_eq!(142, result);
    }
}