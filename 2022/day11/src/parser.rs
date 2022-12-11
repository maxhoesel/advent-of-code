use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{newline, u32 as nom32};
use nom::combinator::{eof, map};
use nom::multi::{many0, many_till, separated_list0};
use nom::sequence::tuple;
use nom::{
    character::complete::{space0, space1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};

use crate::monkey::{InspectOp, InspectValue, ItemWorryLevel, Monkey};

pub fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    let p = many_till(monkey, eof);
    map(p, |a| a.0)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let p = tuple((
        monkey_header,
        monkey_items,
        monkey_op,
        monkey_test,
        monkey_worried_target,
        monkey_unworried_target,
        many0(newline),
    ));
    map(
        p,
        |(
            id,
            items,
            (inspect_op, inspect_value),
            worrytest_value,
            target_worried,
            target_unworried,
            _,
        )| {
            Monkey::new(
                id,
                items,
                inspect_op,
                inspect_value,
                worrytest_value,
                target_worried,
                target_unworried,
            )
        },
    )(input)
}

fn monkey_header(input: &str) -> IResult<&str, usize> {
    let p = delimited(tag("Monkey "), nom32, tag(":\n"));
    map(p, |id| id as usize)(input)
}

fn monkey_items(input: &str) -> IResult<&str, Vec<u32>> {
    fn _single(input: &str) -> IResult<&str, Vec<u32>> {
        let p = delimited(pair(space1, tag("Starting items: ")), nom32, newline);
        map(p, |single| vec![single])(input)
    }

    fn _list(input: &str) -> IResult<&str, Vec<u32>> {
        delimited(
            pair(space1, tag("Starting items: ")),
            separated_list0(tag(", "), nom32),
            newline,
        )(input)
    }

    alt((_single, _list))(input)
}

fn monkey_op(input: &str) -> IResult<&str, (InspectOp, InspectValue)> {
    let p = delimited::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        pair(space1, tag("Operation: new = old ")),
        separated_pair(take(1_u8), space0, monkey_op_value),
        newline,
    );

    map(p, |(op, val)| match op {
        "-" => (InspectOp::Sub, val),
        "+" => (InspectOp::Add, val),
        "*" => (InspectOp::Mul, val),
        "/" => (InspectOp::Div, val),
        _ => panic!(),
    })(input)
}

fn monkey_op_value(input: &str) -> IResult<&str, InspectValue> {
    fn _numeric(input: &str) -> IResult<&str, InspectValue> {
        map(nom32, InspectValue::Fixed)(input)
    }

    fn _old(input: &str) -> IResult<&str, InspectValue> {
        map(tag("old"), |_| InspectValue::Input)(input)
    }

    alt((_numeric, _old))(input)
}

fn monkey_test(input: &str) -> IResult<&str, u32> {
    delimited::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        pair(space1, tag("Test: divisible by ")),
        nom32,
        newline,
    )(input)
}

fn monkey_worried_target(input: &str) -> IResult<&str, usize> {
    let p = delimited::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        pair(space1, tag("If true: throw to monkey ")),
        nom32,
        newline,
    );

    map(p, |target| target as usize)(input)
}

fn monkey_unworried_target(input: &str) -> IResult<&str, usize> {
    let p = delimited::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        pair(space1, tag("If false: throw to monkey ")),
        nom32,
        newline,
    );
    map(p, |target| target as usize)(input)
}
