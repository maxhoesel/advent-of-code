use std::ops::{Range, RangeInclusive};

use miette::GraphicalReportHandler;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, opt},
    error::ParseError,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, GenericErrorTree},
    final_parser::final_parser,
};
use thiserror::Error;

use crate::grid::GridCoord;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
pub enum SensorError {
    #[error("Failed to parse Sensor Data")]
    ParseError,
}

#[derive(Error, Debug, miette::Diagnostic)]
#[error("Bad Sensor input")]
struct BadSensorInput {
    #[source_code]
    src: &'static str,

    #[label("{kind}")]
    bad_bit: miette::SourceSpan,

    kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
}

pub trait ManhattanGeometry {
    type Distance;

    fn taxicab_distance(&self, other: &Self) -> Self::Distance;
}
impl ManhattanGeometry for GridCoord {
    type Distance = usize;

    fn taxicab_distance(&self, other: &Self) -> Self::Distance {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Sensor {
    pub pos: GridCoord,
    pub nearest_beacon: GridCoord,
}
impl Sensor {
    pub fn from_line(line: &'static str) -> Result<Self, SensorError> {
        let input = Span::new(line);
        let res: Result<_, ErrorTree<Span>> =
            final_parser(Sensor::_parse_line::<ErrorTree<Span>>)(input);
        let sensor = match res {
            Ok(sensor) => sensor,
            Err(e) => {
                match e {
                    GenericErrorTree::Base { location, kind } => {
                        let offset = location.location_offset().into();
                        let err = BadSensorInput {
                            src: line,
                            bad_bit: miette::SourceSpan::new(offset, 0.into()),
                            kind,
                        };
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .render_report(&mut s, &err)
                            .unwrap();
                        println!("{s}");
                    }
                    GenericErrorTree::Stack { .. } => todo!("stack"),
                    GenericErrorTree::Alt(_) => todo!("alt"),
                };
                return Err(SensorError::ParseError);
            }
        };
        Ok(sensor)
    }
    pub fn nearest_beacon_distance(&self) -> usize {
        self.pos.taxicab_distance(&self.nearest_beacon)
    }

    pub fn coverage_by_row(&self, row: isize) -> Option<RangeInclusive<isize>> {
        if row > self.pos.y + self.nearest_beacon_distance() as isize
            || row < self.pos.y - self.nearest_beacon_distance() as isize
        {
            return None;
        }
        let x_min = self.pos.x
            - (self.nearest_beacon_distance() as isize - row.abs_diff(self.pos.y) as isize)
                as isize;
        let x_max = self.pos.x
            + (self.nearest_beacon_distance() as isize - row.abs_diff(self.pos.y) as isize)
                as isize;
        Some(x_min..=x_max)
    }

    pub fn coverage_by_column(&self, col: isize) -> Option<RangeInclusive<isize>> {
        if col > self.pos.x + self.nearest_beacon_distance() as isize
            || col < self.pos.x - self.nearest_beacon_distance() as isize
        {
            return None;
        }
        let y_min =
            self.pos.y - (self.nearest_beacon_distance() - col.abs_diff(self.pos.x)) as isize;
        let y_max =
            self.pos.y + (self.nearest_beacon_distance() - col.abs_diff(self.pos.x)) as isize;
        Some(y_min..=y_max)
    }

    fn _parse_line<'a, E: ParseError<Span<'a>>>(input: Span<'a>) -> IResult<Span<'a>, Sensor, E> {
        let p = tuple((
            delimited(
                tag("Sensor at "),
                separated_pair(
                    preceded(tag("x="), nom::character::complete::i32),
                    tag(", "),
                    preceded(tag("y="), nom::character::complete::i32),
                ),
                tag(": "),
            ),
            preceded(
                tag("closest beacon is at "),
                separated_pair(
                    preceded(tag("x="), nom::character::complete::i32),
                    tag(", "),
                    preceded(tag("y="), nom::character::complete::i32),
                ),
            ),
            opt(newline),
        ));
        map(p, |(sensor_coords, beacon_coords, _)| Sensor {
            pos: GridCoord {
                x: isize::try_from(sensor_coords.0).unwrap(),
                y: isize::try_from(sensor_coords.1).unwrap(),
            },
            nearest_beacon: GridCoord {
                x: isize::try_from(beacon_coords.0).unwrap(),
                y: isize::try_from(beacon_coords.1).unwrap(),
            },
        })(input)
    }

    fn _parse_coord_pair<'a, E: ParseError<Span<'a>>>(
        input: Span<'a>,
    ) -> IResult<Span<'a>, (i32, i32)> {
        separated_pair(
            preceded(tag("x="), nom::character::complete::i32),
            tag(", "),
            preceded(tag("y="), nom::character::complete::i32),
        )(input)
    }
}
