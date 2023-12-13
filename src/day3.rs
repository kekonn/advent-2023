use std::{io::Read, ops::Range};

use regex::Regex;

use crate::util;

#[derive(Debug, Clone)]
struct LineNumber {
    pub num: u16,
    pub position: Range<usize>,
}

const NUM_PATTERN: &str = r"(\d)*";
const SYMBOL_PATTERN: &str = r"[^\d\.]";

/// keep the last line, go to next line
/// scan current line for symbols, save position of symbol
/// check same position on line above for numbers, extract to list
/// save positions of symbols for scan of next line as well
pub fn day3(file: impl Read) {
    let mut lines = util::read_lines(file);
    let num_regex = Regex::new(NUM_PATTERN).unwrap();
    let sym_regex = Regex::new(SYMBOL_PATTERN).unwrap();
    let mut part_numbers: Vec<u16> = vec![];
    let mut previous_nums: Vec<LineNumber> = vec![];

    while let Some(Ok(current_line)) = lines.next() {
        let current_line = current_line.trim();
        // parse all numbers from current line
        let num_matches: Vec<LineNumber> = num_regex.find_iter(current_line)
            .map(|m| LineNumber { num: m.as_str().parse::<u16>().unwrap(), position: m.range()})
            .collect();


        let symbol_locations = sym_regex.find_iter(current_line).map(|m| m.range());

        // if previous line contained none, no sense in matching previous line
        if previous_nums.is_empty() {
            previous_nums = num_matches.clone();
        } else {
            // somehow match both lines at the same time?
        }

        // search for symbols on the current line
        for sym_loc in symbol_locations {
            let near_nums = num_matches.iter().filter(|&ln| ranges_overlap(&ln.position, &sym_loc)).collect::<Vec<&LineNumber>>();
        }
    }
}

fn ranges_overlap<T: PartialOrd>(range1: &Range<T>, range2: &Range<T>) -> bool {
    range1.start <= range2.end && range1.end >= range2.start
}


#[cfg(test)]
mod tests {
    use crate::util;
    use super::ranges_overlap;

    const DAY3_INPUT: &str = r#"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;

    #[test]
    fn day3() {
        let input = util::str2reader(DAY3_INPUT);
    }

    #[test]
    fn overlapping_test() {
        let range1 = 1..5;
        let range2 = 3..7;
        let range3 = 8..10;

        assert!(ranges_overlap(&range1, &range2));
        assert!(!ranges_overlap(&range2, &range3));
        assert!(!ranges_overlap(&range1, &range3))
    }
}