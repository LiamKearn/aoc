// Warning: Janky south.

use nom::{
    branch::alt,
    character::complete::{char, newline, satisfy},
    combinator::{map, opt},
    multi::{count, many0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn single_char(i: &[u8]) -> IResult<&[u8], char> {
    satisfy(char::is_alphabetic)(i)
}

fn parse_crate(i: &[u8]) -> IResult<&[u8], Option<char>> {
    let (i, char) = delimited(char('['), single_char, char(']'))(i)?;
    Ok((i, Some(char)))
}

fn parse_empty_crate(i: &[u8]) -> IResult<&[u8], Option<char>> {
    map(count(char(' '), 3), |_| None)(i)
}

fn parse_slot(i: &[u8]) -> IResult<&[u8], Option<char>> {
    alt((parse_crate, parse_empty_crate))(i)
}

// FIXME: Newline handling is janky.
fn parse_line(i: &[u8]) -> IResult<&[u8], Vec<Option<char>>> {
    let (i, (first_slot, mut all)) =
        tuple((parse_slot, many0(preceded(char(' '), parse_slot))))(i)?;
    let (i, _) = opt(newline)(i)?;

    all.insert(0, first_slot);

    Ok((i, all))
}

pub fn parse_lines(i: &[u8]) -> IResult<&[u8], Vec<Vec<Option<char>>>> {
    many1(parse_line)(i)
}

#[cfg(test)]
mod tests {
    use super::{parse_crate, parse_empty_crate, parse_line, parse_lines};
    use crate::EXAMPLE_INPUT;

    #[test]
    fn test_parse_valid_line() {
        let inp = r#"[D]                     [N] [F]    "#;
        let res = parse_line(inp.as_bytes());
        let crates = res.unwrap().1;
        assert_eq!(crates.first().unwrap().unwrap(), 'D');
        assert_eq!(crates.last().unwrap(), &None);
        assert_eq!(crates.len(), 9);
    }

    #[test]
    fn test_parse_leftover_line() {
        let inp = r#"[D] [N]
[s]   "#;
        let res = parse_line(inp.as_bytes());
        assert_eq!(std::str::from_utf8(res.unwrap().0).unwrap(), "[s]   ")
    }

    #[test]
    fn test_parse_valid_empty_crate() {
        parse_empty_crate("   ".as_bytes()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_empty_crate() {
        parse_empty_crate(" ".as_bytes()).unwrap();
    }

    #[test]
    fn test_parse_empty_crate_big() {
        let (remainder, krate) = parse_empty_crate("        ".as_bytes()).unwrap();
        assert!(krate.is_none());
        assert_eq!(remainder, "     ".as_bytes());
    }

    #[test]
    fn test_parse_two_crates() {
        assert_eq!(
            parse_line(r#"[a] [b]"#.as_bytes()).unwrap().1,
            vec![Some('a'), Some('b')]
        )
    }

    #[test]
    fn test_parse_valid_crate() {
        assert_eq!(parse_crate("[i]".as_bytes()).unwrap().1, Some('i'))
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_crate_to_many_chars() {
        parse_crate("[yy]".as_bytes()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_crate_double_open() {
        parse_crate("[[z]".as_bytes()).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_crate_double() {
        parse_crate("[[h]]".as_bytes()).unwrap();
    }

    #[test]
    fn test_example_input() {
        let res = parse_lines(EXAMPLE_INPUT.as_bytes());
        let crates = res.unwrap().1;
        assert_eq!(crates[0][0], None);
        let only_crates: Vec<Vec<char>> = crates
            .into_iter()
            .map(|row| row.into_iter().flatten().collect::<Vec<char>>())
            .collect();
        assert_eq!(only_crates[0].last(), Some(&'D'));
        assert_eq!(only_crates[1].first(), Some(&'N'));
        assert_eq!(only_crates[1].last(), Some(&'C'));
        assert_eq!(only_crates[2].first(), Some(&'Z'));
        assert_eq!(only_crates[2].last(), Some(&'P'));
    }

    #[test]
    fn test_parse_blob() {
        let inp = r#"[D]                     [N] [F]    
[H] [F]             [L] [J] [H]    
[R] [H]             [F] [V] [G] [H]
[Z] [Q]         [Z] [W] [L] [J] [B]
[S] [W] [H]     [B] [H] [D] [C] [M]
[P] [R] [S] [G] [J] [J] [W] [Z] [V]
[W] [B] [V] [F] [G] [T] [T] [T] [P]
[Q] [V] [C] [H] [P] [Q] [Z] [D] [W]
 1   2   3   4   5   6   7   8   9 "#;

        let res = parse_lines(inp.as_bytes());
        let out = res.unwrap().1;
        assert_eq!(out[0].first().unwrap().unwrap(), 'D');
        assert_eq!(out[0].last().unwrap(), &None);
        assert_eq!(out[3][2], None);
        assert_eq!(out[3][3], None);

        let only_crates: Vec<Vec<char>> = out
            .into_iter()
            .map(|row| row.into_iter().flatten().collect::<Vec<char>>())
            .collect();
        assert_eq!(only_crates[0].last(), Some(&'F'));
        assert_eq!(only_crates[1].first(), Some(&'H'));
        assert_eq!(only_crates[1].last(), Some(&'H'));
        assert_eq!(only_crates[4].first(), Some(&'S'));
        assert_eq!(only_crates[4].last(), Some(&'M'));
    }
}
