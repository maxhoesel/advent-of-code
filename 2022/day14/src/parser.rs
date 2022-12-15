use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    character::complete::u32 as nomu32,
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{separated_pair, terminated},
    IResult,
};

use crate::cave::CavePos;

pub fn rock_formations(input: &str) -> IResult<&str, Vec<CavePos>> {
    let p = many0(formation);
    map(p, |formations| {
        formations.into_iter().flatten().unique().collect_vec()
    })(input)
}

fn formation(input: &str) -> IResult<&str, Vec<CavePos>> {
    let p = terminated(separated_list0(tag(" -> "), pos), newline);
    map(p, |coords| {
        coords
            .windows(2)
            .map(|windows| {
                let start = windows[0];
                let end = windows[1];
                if start.x.abs_diff(end.x) > 0 {
                    let range = start.x.min(end.x)..=start.x.max(end.x);
                    range
                        .into_iter()
                        .map(|x| CavePos { x, y: start.y })
                        .collect_vec()
                } else {
                    let range = start.y.min(end.y)..=start.y.max(end.y);
                    range
                        .into_iter()
                        .map(|y| CavePos { x: start.x, y })
                        .collect_vec()
                }
            })
            .flatten()
            .collect_vec()
    })(input)
}

fn pos(input: &str) -> IResult<&str, CavePos> {
    map(separated_pair(nomu32, tag(","), nomu32), |p| CavePos {
        x: p.0 as usize,
        y: p.1 as usize,
    })(input)
}
