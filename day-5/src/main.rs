#![allow(clippy::missing_errors_doc)]

use crate::{instruction_parser::parse_instruction, stacks_parser::parse_lines};
use std::{
    fmt::{Debug, Display, Write},
    ops::{Index, IndexMut},
};

pub mod instruction_parser;
pub mod stacks_parser;

#[derive(Debug, PartialEq, Default)]
pub struct Crate(char);

impl From<char> for Crate {
    fn from(value: char) -> Self {
        Crate(value)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        f.write_char(self.0)?;
        f.write_char(']')?;

        Ok(())
    }
}

#[derive(Default)]
pub struct Stacks<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Stacks<T> {
    fn height(&self) -> usize {
        self.inner.iter().map(Vec::len).max().unwrap()
    }

    fn width(&self) -> usize {
        self.inner.len()
    }

    fn pop_from_column(&mut self, column: usize) -> Option<T> {
        self.inner.index_mut(column).pop()
    }

    fn push_to_column(&mut self, column: usize, element: T) {
        if let Some(existing) = self.inner.get_mut(column) {
            existing.push(element);
        } else {
            self.inner.insert(column, Vec::default());
            self.push_to_column(column, element);
        }
    }

    fn index_column(&self, column: usize, row: usize) -> Option<&T> {
        self.inner.index(column).get(row)
    }
}

impl Stacks<Crate> {
    fn get_message(&self) -> String {
        self.inner
            .iter()
            // TODO: unwrap here will give wrong message.
            .map(|col| col.last().unwrap().0)
            .collect::<String>()
    }

    fn parse(input: &str) -> Stacks<Crate> {
        // The last line, this should be stack indices, we can be remove this and use it to assert parse correctness.
        let stack_indices = input.lines().last().unwrap();
        let suposed_stack_count: usize = stack_indices.chars().filter(char::is_ascii_digit).count();

        // Parse the stack minus the indices.
        let (to_stack, _) = input.split_once(stack_indices).unwrap();
        let parsed_stack = parse_lines(to_stack.as_bytes()).unwrap();
        let stack_rows = transpose(parsed_stack.1);

        // Assert that indices match the width of our parsed crates.
        assert_eq!(
            stack_rows.len(),
            suposed_stack_count,
            "The parsed indicies don't align with the crates we parsed"
        );

        let mut stacks: Stacks<Crate> = Stacks::default();
        for (column_idx, column) in stack_rows.iter().enumerate() {
            let column_items = column
                .iter()
                // We want to discard empty crates, if we ignore empty crates we don't have
                // to deal with them after parsing.
                .flatten()
                .rev();

            for krate in column_items {
                stacks.push_to_column(column_idx, (*krate).into());
            }
        }

        stacks
    }
}

impl Display for Stacks<Crate> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Render crates.
        for row in (0..=self.height()).rev() {
            for column in 0..self.width() {
                if let Some(krate) = self.index_column(column, row) {
                    f.write_fmt(format_args!("{krate}"))?;
                } else {
                    f.write_str("   ")?;
                }
                if column != self.width() - 1 {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }

        // Label crate columns.
        for column in 0..self.width() {
            f.write_char(' ')?;
            f.write_str((column + 1).to_string().as_str())?;
            f.write_char(' ')?;
            if column != self.width() - 1 {
                f.write_char(' ')?;
            }
        }

        Ok(())
    }
}

pub trait OP<T> {
    fn enact(&self, target: &mut Stacks<T>, animation_duration: u64) -> Option<()>;
}

#[derive(Debug, PartialEq)]
pub struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Move {qty} from {from} to {to}",
            qty = self.qty,
            from = self.from,
            to = self.to,
        )
    }
}

impl OP<Crate> for Move {
    fn enact(&self, target: &mut Stacks<Crate>, animation_duration: u64) -> Option<()> {
        #[cfg(debug_assertions)]
        {
            println!("\nApplied `{self}`:");
        }

        for _ in 1..=self.qty {
            let krate = target.pop_from_column(self.from - 1)?;
            target.push_to_column(self.to - 1, krate);
        }

        #[cfg(debug_assertions)]
        {
            println!("{target}");
        }
        if animation_duration > 0 {
            println!("{target}");
            println!("Applied `{self}`.");
        }

        Some(())
    }
}

fn solve_part_1(input: &str, animation_duration: u64) -> String {
    let (raw_stacks, raw_instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = Stacks::parse(raw_stacks);

    #[cfg(debug_assertions)]
    {
        println!("Initial stack:");
        println!("{stacks}");
    }

    if animation_duration > 0 {
        // CBA using crossterm lol.
        print!("{}", "\n".repeat(100));
        println!("{stacks}");
    }

    for instruction in raw_instructions.lines() {
        let moove = parse_instruction(instruction).unwrap().1;
        if animation_duration > 0 {
            std::thread::sleep(std::time::Duration::from_millis(animation_duration));
            print!("{}", "\n".repeat(100));
            print!("{}[2J", 27 as char);
        }
        moove.enact(&mut stacks, animation_duration);
    }

    stacks.get_message()
}

fn solve_part_2(input: &str, animation_duration: u64) -> String {
    todo!()
}

trait DisplayMarker {}

enum Part {
    One(Stage),
    Two(Stage),
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One(stage) => write!(f, "One > {stage}:"),
            Part::Two(stage) => write!(f, "Two > {stage}:"),
        }
    }
}

enum Stage {
    Example,
    Actual,
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stage::Example => write!(f, "Example"),
            Stage::Actual => write!(f, "Actual"),
        }
    }
}

fn solve_animated<F>(part: Part, solver: F, input: &str, animation_duration: u64)
where
    F: Fn(&str, u64) -> String,
{
    if animation_duration > 0 {
        for _ in 0..10 {
            print!("{}[2J", 27 as char);
        }
    }

    let res = solver(input, animation_duration);
    println!("Solved {part} {res}");
    if animation_duration < 1 {
        return;
    }

    std::thread::sleep(std::time::Duration::from_millis(animation_duration));
}

fn main() {
    // Any argument enables animate.
    let mut animation_duration = 0;
    let mut args = std::env::args();
    if args.nth(1).is_some() {
        animation_duration = args.next().expect("Supply a frame duration in milis").parse::<u64>().unwrap();
    }
    solve_animated(Part::One(Stage::Example), solve_part_1, EXAMPLE_INPUT, animation_duration);
    solve_animated(Part::One(Stage::Actual), solve_part_1, ACTUAL_INPUT, animation_duration);
    solve_animated(Part::Two(Stage::Example), solve_part_1, EXAMPLE_INPUT, animation_duration);
    solve_animated(Part::Two(Stage::Actual), solve_part_1, ACTUAL_INPUT, animation_duration);
}

#[cfg(test)]
mod tests {
    use crate::{solve_part_1, ACTUAL_INPUT, EXAMPLE_INPUT};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(EXAMPLE_INPUT, 0), "CMZ".to_string());
        assert_eq!(solve_part_1(ACTUAL_INPUT, 0), "ZWHVFWQWW".to_string());
    }
}

// https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v
        .into_iter()
        .map(std::iter::IntoIterator::into_iter)
        .collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

const EXAMPLE_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

const ACTUAL_INPUT: &str = r#"[D]                     [N] [F]    
[H] [F]             [L] [J] [H]    
[R] [H]             [F] [V] [G] [H]
[Z] [Q]         [Z] [W] [L] [J] [B]
[S] [W] [H]     [B] [H] [D] [C] [M]
[P] [R] [S] [G] [J] [J] [W] [Z] [V]
[W] [B] [V] [F] [G] [T] [T] [T] [P]
[Q] [V] [C] [H] [P] [Q] [Z] [D] [W]
 1   2   3   4   5   6   7   8   9 

move 1 from 3 to 9
move 2 from 2 to 1
move 3 from 5 to 4
move 1 from 1 to 8
move 1 from 3 to 9
move 1 from 5 to 7
move 1 from 5 to 3
move 4 from 4 to 2
move 2 from 3 to 4
move 1 from 3 to 2
move 6 from 1 to 5
move 1 from 4 to 3
move 1 from 3 to 9
move 4 from 2 to 4
move 4 from 8 to 7
move 3 from 2 to 6
move 1 from 2 to 7
move 5 from 5 to 6
move 1 from 5 to 8
move 5 from 8 to 7
move 7 from 4 to 6
move 15 from 6 to 4
move 1 from 8 to 7
move 1 from 1 to 5
move 1 from 2 to 4
move 2 from 4 to 8
move 1 from 5 to 2
move 5 from 6 to 4
move 2 from 2 to 1
move 1 from 9 to 4
move 1 from 6 to 9
move 3 from 9 to 3
move 3 from 4 to 3
move 1 from 6 to 1
move 5 from 3 to 4
move 2 from 8 to 5
move 1 from 3 to 6
move 1 from 6 to 2
move 1 from 2 to 8
move 6 from 4 to 2
move 1 from 2 to 7
move 1 from 5 to 3
move 4 from 9 to 3
move 1 from 9 to 1
move 3 from 1 to 6
move 1 from 9 to 7
move 14 from 7 to 6
move 1 from 8 to 3
move 4 from 2 to 6
move 3 from 3 to 8
move 9 from 4 to 9
move 1 from 1 to 5
move 2 from 5 to 8
move 3 from 8 to 2
move 4 from 2 to 6
move 1 from 3 to 9
move 10 from 6 to 1
move 5 from 9 to 8
move 1 from 9 to 3
move 6 from 1 to 8
move 3 from 7 to 4
move 2 from 4 to 5
move 2 from 9 to 8
move 15 from 8 to 3
move 3 from 7 to 9
move 8 from 4 to 3
move 2 from 5 to 9
move 6 from 6 to 5
move 6 from 5 to 8
move 1 from 7 to 8
move 6 from 9 to 2
move 5 from 2 to 4
move 6 from 3 to 5
move 5 from 5 to 8
move 1 from 5 to 7
move 1 from 9 to 7
move 2 from 6 to 4
move 12 from 8 to 2
move 7 from 2 to 4
move 3 from 7 to 5
move 3 from 5 to 7
move 3 from 7 to 9
move 2 from 9 to 7
move 1 from 9 to 3
move 2 from 7 to 4
move 3 from 1 to 9
move 4 from 6 to 5
move 6 from 2 to 8
move 14 from 4 to 9
move 7 from 9 to 6
move 9 from 9 to 2
move 1 from 5 to 8
move 5 from 6 to 3
move 3 from 1 to 9
move 3 from 8 to 9
move 1 from 8 to 3
move 5 from 2 to 5
move 1 from 4 to 9
move 2 from 6 to 1
move 2 from 3 to 6
move 3 from 8 to 3
move 2 from 6 to 3
move 1 from 4 to 9
move 4 from 3 to 6
move 7 from 6 to 9
move 10 from 9 to 2
move 10 from 3 to 2
move 7 from 2 to 8
move 2 from 1 to 7
move 13 from 3 to 7
move 7 from 5 to 1
move 1 from 9 to 6
move 4 from 8 to 4
move 2 from 3 to 2
move 4 from 4 to 6
move 1 from 3 to 4
move 5 from 6 to 5
move 3 from 5 to 7
move 12 from 2 to 5
move 7 from 5 to 6
move 2 from 8 to 3
move 7 from 6 to 2
move 3 from 9 to 6
move 1 from 6 to 7
move 1 from 4 to 9
move 2 from 7 to 6
move 13 from 7 to 4
move 3 from 7 to 5
move 1 from 9 to 6
move 12 from 4 to 3
move 1 from 8 to 1
move 2 from 6 to 4
move 1 from 7 to 9
move 2 from 9 to 8
move 12 from 3 to 5
move 1 from 8 to 2
move 15 from 5 to 6
move 2 from 4 to 6
move 1 from 9 to 6
move 5 from 5 to 4
move 4 from 4 to 2
move 2 from 1 to 5
move 4 from 1 to 5
move 1 from 8 to 6
move 7 from 5 to 2
move 22 from 2 to 3
move 9 from 6 to 3
move 1 from 1 to 8
move 1 from 8 to 7
move 23 from 3 to 6
move 2 from 2 to 4
move 1 from 7 to 8
move 1 from 8 to 2
move 19 from 6 to 9
move 2 from 2 to 4
move 4 from 4 to 6
move 13 from 6 to 8
move 12 from 9 to 1
move 2 from 5 to 9
move 2 from 4 to 8
move 1 from 2 to 7
move 1 from 7 to 1
move 4 from 6 to 2
move 10 from 1 to 9
move 1 from 6 to 7
move 11 from 8 to 2
move 6 from 3 to 6
move 1 from 7 to 2
move 1 from 1 to 8
move 2 from 6 to 7
move 7 from 6 to 3
move 9 from 3 to 1
move 7 from 9 to 6
move 1 from 8 to 7
move 4 from 2 to 6
move 1 from 8 to 3
move 6 from 6 to 5
move 9 from 9 to 3
move 5 from 6 to 1
move 1 from 7 to 8
move 2 from 8 to 4
move 1 from 4 to 2
move 1 from 4 to 5
move 2 from 5 to 6
move 1 from 6 to 9
move 9 from 1 to 4
move 4 from 4 to 6
move 2 from 4 to 7
move 7 from 2 to 8
move 5 from 6 to 7
move 6 from 3 to 8
move 8 from 1 to 9
move 3 from 5 to 2
move 2 from 3 to 9
move 3 from 9 to 4
move 7 from 2 to 3
move 1 from 7 to 2
move 10 from 3 to 2
move 6 from 9 to 4
move 1 from 3 to 1
move 1 from 1 to 8
move 4 from 8 to 5
move 10 from 8 to 4
move 2 from 8 to 9
move 7 from 4 to 9
move 6 from 2 to 6
move 3 from 6 to 5
move 4 from 4 to 9
move 8 from 7 to 5
move 1 from 9 to 2
move 7 from 2 to 1
move 4 from 9 to 8
move 2 from 6 to 3
move 2 from 3 to 2
move 13 from 5 to 7
move 5 from 4 to 9
move 5 from 1 to 7
move 3 from 5 to 8
move 17 from 7 to 2
move 15 from 2 to 6
move 15 from 9 to 5
move 1 from 9 to 5
move 4 from 8 to 6
move 1 from 4 to 6
move 5 from 4 to 7
move 5 from 2 to 7
move 18 from 6 to 2
move 2 from 7 to 6
move 10 from 2 to 8
move 2 from 2 to 3
move 11 from 8 to 7
move 7 from 7 to 5
move 9 from 7 to 5
move 3 from 7 to 5
move 2 from 1 to 7
move 4 from 2 to 1
move 30 from 5 to 1
move 1 from 3 to 1
move 35 from 1 to 9
move 2 from 2 to 5
move 2 from 8 to 3
move 20 from 9 to 2
move 3 from 7 to 9
move 1 from 3 to 6
move 5 from 5 to 3
move 18 from 2 to 5
move 4 from 5 to 8
move 7 from 9 to 7
move 1 from 6 to 2
move 3 from 8 to 5
move 6 from 3 to 5
move 3 from 7 to 4
move 2 from 2 to 3
move 1 from 4 to 5
move 2 from 4 to 5
move 4 from 7 to 2
move 26 from 5 to 6
move 2 from 2 to 7
move 1 from 2 to 9
move 1 from 7 to 8
move 1 from 5 to 3
move 2 from 8 to 3
move 11 from 9 to 3
move 6 from 3 to 4
move 27 from 6 to 4
move 33 from 4 to 3
move 4 from 6 to 8
move 1 from 2 to 8
move 1 from 7 to 3
move 4 from 8 to 9
move 1 from 8 to 6
move 34 from 3 to 8
move 1 from 8 to 5
move 1 from 2 to 9
move 8 from 3 to 9
move 3 from 5 to 4
move 1 from 6 to 5
move 27 from 8 to 9
move 1 from 3 to 4
move 1 from 5 to 7
move 3 from 8 to 1
move 11 from 9 to 1
move 1 from 7 to 5
move 11 from 9 to 3
move 1 from 5 to 1
move 1 from 8 to 7
move 2 from 9 to 2
move 1 from 2 to 1
move 1 from 2 to 7
move 2 from 8 to 2
move 6 from 3 to 8
move 1 from 4 to 2
move 7 from 1 to 2
move 1 from 7 to 1
move 19 from 9 to 1
move 3 from 2 to 9
move 10 from 1 to 4
move 2 from 9 to 1
move 1 from 7 to 9
move 7 from 1 to 6
move 10 from 4 to 3
move 14 from 1 to 7
move 2 from 9 to 1
move 3 from 4 to 6
move 9 from 7 to 6
move 1 from 3 to 5
move 4 from 8 to 5
move 10 from 6 to 8
move 3 from 5 to 6
move 10 from 3 to 4
move 4 from 3 to 7
move 1 from 5 to 9
move 2 from 7 to 9
move 1 from 1 to 9
move 6 from 2 to 4
move 1 from 5 to 3
move 11 from 4 to 9
move 3 from 4 to 9
move 1 from 2 to 7
move 2 from 3 to 5
move 1 from 3 to 2
move 7 from 7 to 2
move 2 from 5 to 8
move 8 from 2 to 1
move 2 from 6 to 8
move 9 from 6 to 8
move 3 from 8 to 2
move 3 from 2 to 6
move 9 from 9 to 5
move 3 from 5 to 8
move 5 from 9 to 4
move 3 from 6 to 4
move 1 from 6 to 3
move 3 from 1 to 6
move 3 from 6 to 9
move 17 from 8 to 5
move 12 from 5 to 4
move 21 from 4 to 3
move 1 from 4 to 9
move 7 from 5 to 4
move 22 from 3 to 7
move 3 from 1 to 8
move 3 from 9 to 1
move 4 from 4 to 6
move 1 from 6 to 2
move 3 from 4 to 1
move 1 from 6 to 7
move 4 from 9 to 3
move 2 from 5 to 7
move 1 from 9 to 6
move 2 from 6 to 9
move 8 from 7 to 9
move 1 from 6 to 2
move 1 from 9 to 3
move 4 from 3 to 4
move 14 from 7 to 4
move 1 from 3 to 2
move 3 from 7 to 8
move 12 from 8 to 9
move 8 from 4 to 1
move 1 from 7 to 4
move 2 from 5 to 1
move 3 from 2 to 9
move 17 from 9 to 3
move 6 from 9 to 1
move 1 from 9 to 2
move 13 from 3 to 9
move 4 from 3 to 1
move 3 from 9 to 1
move 22 from 1 to 9
move 1 from 8 to 1
move 6 from 9 to 5
move 4 from 1 to 9
move 3 from 1 to 9
move 4 from 4 to 8
move 4 from 4 to 2
move 1 from 4 to 3
move 3 from 8 to 9
move 1 from 3 to 4
move 1 from 1 to 3
move 1 from 8 to 2
move 1 from 5 to 8
move 4 from 2 to 1
move 1 from 8 to 7
move 10 from 9 to 6
move 1 from 7 to 9
move 1 from 2 to 3
move 1 from 6 to 1
move 3 from 5 to 7
move 1 from 8 to 7
move 1 from 6 to 1
move 1 from 2 to 4
move 1 from 5 to 2
move 19 from 9 to 2
move 1 from 4 to 7
move 1 from 3 to 7
move 3 from 7 to 9
move 4 from 1 to 2
move 10 from 9 to 4
move 1 from 5 to 8
move 3 from 6 to 4
move 1 from 3 to 4
move 10 from 2 to 8
move 12 from 2 to 5
move 3 from 5 to 9
move 5 from 6 to 5
move 5 from 1 to 4
move 22 from 4 to 3
move 3 from 8 to 7
move 1 from 7 to 2
move 3 from 2 to 9
move 19 from 3 to 5
move 2 from 7 to 8
move 7 from 5 to 6
move 5 from 9 to 6
move 1 from 9 to 3
move 16 from 5 to 1
move 2 from 3 to 1
move 3 from 7 to 3
move 7 from 8 to 4
move 2 from 8 to 1
move 5 from 5 to 9
move 1 from 5 to 2
move 1 from 2 to 3
move 1 from 8 to 5
move 4 from 5 to 7
move 2 from 3 to 8
move 2 from 1 to 5
move 4 from 7 to 6
move 6 from 4 to 7
move 4 from 9 to 8
move 14 from 6 to 7
move 8 from 1 to 7
move 7 from 1 to 3
move 3 from 5 to 9
move 28 from 7 to 5
move 1 from 1 to 8
move 4 from 8 to 3
move 9 from 3 to 1
move 1 from 9 to 5
move 6 from 3 to 2
move 10 from 1 to 6
move 1 from 1 to 9
move 5 from 9 to 7
move 14 from 5 to 3
move 1 from 4 to 1
move 1 from 7 to 2
move 1 from 7 to 1
move 1 from 1 to 7
move 3 from 8 to 5
move 4 from 6 to 3
move 3 from 7 to 2
move 15 from 3 to 6
move 16 from 5 to 7
move 4 from 2 to 8
move 1 from 3 to 1
move 5 from 7 to 3
move 12 from 6 to 4
move 4 from 8 to 5
move 1 from 4 to 2
move 2 from 5 to 3
move 8 from 6 to 3
move 7 from 4 to 5
move 9 from 7 to 6
move 1 from 7 to 9
move 1 from 1 to 9
move 1 from 1 to 9
move 5 from 2 to 8
move 5 from 8 to 2
move 11 from 5 to 9
move 1 from 4 to 2
move 4 from 9 to 6
move 12 from 3 to 7
move 3 from 4 to 9
move 14 from 6 to 2
move 2 from 2 to 4
move 2 from 3 to 5
move 10 from 7 to 2
move 1 from 4 to 8
move 1 from 2 to 7
move 28 from 2 to 9
move 4 from 7 to 5
move 1 from 2 to 4
move 6 from 5 to 1
move 2 from 4 to 3
move 1 from 8 to 1
move 40 from 9 to 1
move 10 from 1 to 6
move 5 from 3 to 5
move 1 from 9 to 8
move 3 from 6 to 7
move 11 from 1 to 2
move 9 from 2 to 3
move 3 from 5 to 1
move 4 from 7 to 1
move 2 from 2 to 4
move 2 from 5 to 8
move 19 from 1 to 7
move 8 from 3 to 2
move 14 from 1 to 8
move 14 from 7 to 1
move 4 from 6 to 5
move 1 from 1 to 9"#;
