// Warning: Janky north.

use crate::Move;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_opt,
    sequence::{separated_pair, tuple},
    IResult,
};

#[allow(clippy::missing_errors_doc)]
pub fn parse_instruction(i: &str) -> IResult<&str, Move> {
    let (i, ((operation, qty), _, (_, from), _, (_, to))) = tuple((
        parse_specifier_numerator_pair,
        space1,
        parse_specifier_numerator_pair,
        space1,
        parse_specifier_numerator_pair,
    ))(i)?;

    // LEARN: Is something like the following but limited better given the vector collection?
    // let ... = separated_list1(space1, parse_specifier_numerator_pair)(i)?;

    match operation {
        "move" => Ok((i, Move { qty, from, to })),
        _ => unimplemented!(),
    }
}

fn parse_specifier_numerator_pair(i: &str) -> IResult<&str, (&str, usize)> {
    separated_pair(
        parse_specifier_keyword,
        space1,
        map_opt(digit1, |s: &str| s.parse::<usize>().ok()),
    )(i)
}

fn parse_specifier_keyword(i: &str) -> IResult<&str, &str> {
    alt((tag("move"), tag("from"), tag("to")))(i)
}

#[cfg(test)]
mod tests {
    use super::parse_instruction;
    use crate::Move;

    #[test]
    fn test_parse_move() {
        assert_eq!(
            parse_instruction("move 1 from 2 to 1").unwrap().1,
            Move {
                qty: 1,
                from: 2,
                to: 1,
            }
        );
    }

    #[test]
    fn test_big_digit_move() {
        assert_eq!(
            parse_instruction("move 15 from 243 to 1344").unwrap().1,
            Move {
                qty: 15,
                from: 243,
                to: 1344,
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_operation() {
        parse_instruction("asd 1 from 2 to 1").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_no_seperators() {
        parse_instruction("move1from2to1").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_digit() {
        parse_instruction("move 13s33 from 2 to 1").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_from() {
        parse_instruction("move 56 fromm 2 to 4").unwrap();
    }
}
