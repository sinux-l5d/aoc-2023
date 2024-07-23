use std::io;

/// Function that process a file against day-01 challenge
pub fn process(f: impl io::BufRead) -> usize {
    f.lines()
        .map(|line| nb_from_line(line.unwrap_or("".into()).as_str()))
        .sum()
}

fn nb_from_line(l: &str) -> usize {
    let mut it = l.bytes().filter_map(|b| match b {
        b'0'..=b'9' => Some(b - b'0'),
        _ => None,
    });

    if let Some(first) = it.next() {
        let last = it.last().unwrap_or(first);
        (10 * first + last).into()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let file = "1abc2\n\
                    pqr3stu8vwx\n\
                    a1b2c3d4e5f\n\
                    treb7uchet"
            .as_bytes();
        let reader = io::BufReader::new(file);
        assert_eq!(process(reader), 142);
    }

    #[test]
    fn parse_3_digits() {
        assert_eq!(nb_from_line("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn parse_2_digits() {
        assert_eq!(nb_from_line("1abc2"), 12);
    }

    #[test]
    fn parse_1_digit() {
        assert_eq!(nb_from_line("1abc"), 11);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(nb_from_line(""), 0);
    }
}
