use std::io::{Read, BufReader, BufRead, Lines};

/// Convert any reader to a Lines iterator.
pub fn read_lines(read: impl Read) -> Lines<impl BufRead> {
    let reader = BufReader::new(read);
    reader.lines()
}

/// Create a BufReader over a string slice.
pub fn str2reader(string: &str) -> BufReader<&[u8]> {
    BufReader::new(string.as_bytes())
}