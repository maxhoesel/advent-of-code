use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Display,
    fs::read_to_string,
    vec,
};

use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use itertools::Itertools;
use log::info;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{delimited, separated_pair, terminated},
    Finish, IResult,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Clone, PartialEq, Hash, Debug, Eq)]
struct Packet(Element);
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Element {
    List(Vec<Element>),
    Int(i32),
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::List(l) => {
                write!(f, "[")?;
                for e in l {
                    write!(f, "{}", e)?;
                }
                write!(f, "]")
            }
            Element::Int(i) => write!(f, "{}", i),
        }
    }
}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Element::List(l), Element::List(r)) => Element::_cmp_list(l, r),
            (Element::List(l), Element::Int(r)) => {
                info!("Left is list but Right is Integer, converting");
                Element::_cmp_list(l, &[Element::Int(*r)])
            }
            (Element::Int(l), Element::List(r)) => {
                info!("Right is list but Left is integer, converting");
                Element::_cmp_list(&[Element::Int(*l)], r)
            }
            (Element::Int(l), Element::Int(r)) => {
                if l < r {
                    info!(
                        "Left ({}) is smaller than Right ({}), order is correct",
                        l, r
                    );
                    Less
                } else if l > r {
                    info!(
                        "Left ({}) is larger than Right ({}), order is incorrect",
                        l, r
                    );
                    Greater
                } else {
                    info!(
                        "Left ({}) is the same value as Right ({}), continuing",
                        l, r
                    );
                    Equal
                }
            }
        }
    }
}

impl Element {
    fn _cmp_list(l: &[Element], r: &[Element]) -> std::cmp::Ordering {
        for i in 0..l.len().min(r.len()) {
            let e_left = l.get(i).unwrap();
            let e_right = r.get(i).unwrap();
            info!("Comparing Left ({}) and Right ({})", e_left, e_right);
            match e_left.cmp(e_right) {
                Less => return Less,
                Equal => continue,
                Greater => return Greater,
            }
        }
        if l.len() < r.len() {
            Less
        } else if l.len() > r.len() {
            Greater
        } else {
            Equal
        }
    }
}

fn packet_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    let p = terminated(
        separated_list0(tag("\n\n"), separated_pair(packet, tag("\n"), packet)),
        newline,
    );
    map(p, |r| {
        r.into_iter()
            .map(|(l, r)| Pair { left: l, right: r })
            .collect()
    })(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    map(list, |e| Packet(e))(input)
}

fn list(input: &str) -> IResult<&str, Element> {
    let p = delimited(tag("["), elements, tag("]"));
    map(p, |e| Element::List(e))(input)
}

fn elements(input: &str) -> IResult<&str, Vec<Element>> {
    separated_list0(tag(","), alt((list, integer)))(input)
}

fn integer(input: &str) -> IResult<&str, Element> {
    map(nom::character::complete::i32, |e| Element::Int(e))(input)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = read_to_string("input.txt").wrap_err("Reading input.txt")?;
    let pairs = match all_consuming(packet_pairs)(input.as_str()).finish() {
        Ok((_, pairs)) => pairs,
        Err(e) => return Err(eyre!("Parsing Error: {}", e)),
    };

    let right_sum: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, p)| match p.left.cmp(&p.right) {
            Less => Some(i + 1),
            Equal => panic!(),
            Greater => None,
        })
        .sum();
    println!("Sum of correct pair indices: {}", right_sum);

    let mut sorted = pairs
        .iter()
        .cloned()
        .map(|p| vec![p.left, p.right])
        .flatten()
        .collect_vec();
    let div1 = Packet(Element::List(vec![Element::Int(2)]));
    let div2 = Packet(Element::List(vec![Element::Int(6)]));
    sorted.push(div1.clone());
    sorted.push(div2.clone());
    sorted.sort();

    let decoder_key =
        (sorted.binary_search(&div1).unwrap() + 1) * (sorted.binary_search(&div2).unwrap() + 1);
    println!("Decoder Key: {}", decoder_key);

    Ok(())
}
