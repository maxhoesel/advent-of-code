use std::fmt::Display;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::map_res;
use nom::error::{Error, ErrorKind};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair, terminated};
use nom::{Finish, IResult};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass]
struct Game {
    num: u32,
    shown: Vec<CubeSet>,
}
#[pymethods]
impl Game {
    #[getter]
    fn shown(&self) -> PyResult<Vec<CubeSet>> {
        Ok(self.shown.clone())
    }
    #[getter]
    fn num(&self) -> PyResult<u32> {
        Ok(self.num)
    }
}
impl Game {
    fn parse_line(input: &str) -> Result<Game> {
        map_res(
            pair(
                delimited(
                    terminated(tag_no_case("Game"), space0),
                    map_res(digit1, str::parse::<u32>),
                    delimited(space0, char(':'), space0),
                ),
                terminated(
                    separated_list1(delimited(space0, char(';'), space0), CubeSet::parse),
                    multispace0,
                ),
            ),
            |res: (u32, Vec<CubeSet>)| {
                Ok::<_, ErrorKind>(Game {
                    num: res.0,
                    shown: res.1,
                })
            },
        )(input)
        .finish()
        .map(|r| r.1)
        .map_err(|e| anyhow!(e.to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass]
struct CubeSet {
    cubes: Vec<Cubes>,
}
#[pymethods]
impl CubeSet {
    #[getter]
    fn cubes(&self) -> PyResult<Vec<Cubes>> {
        Ok(self.cubes.clone())
    }
}
impl CubeSet {
    fn parse(input: &str) -> IResult<&str, CubeSet> {
        map_res(
            separated_list1(delimited(space0, char(','), space0), Cubes::parse),
            |r| Ok::<_, ErrorKind>(CubeSet { cubes: r }),
        )(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass]
struct Cubes {
    color: CubeColor,
    amount: u32,
}
#[pymethods]
impl Cubes {
    #[getter]
    fn color(&self) -> PyResult<CubeColor> {
        Ok(self.color)
    }
    #[getter]
    fn amount(&self) -> PyResult<u32> {
        Ok(self.amount)
    }
}
impl Cubes {
    fn parse(input: &str) -> IResult<&str, Cubes> {
        map_res(
            separated_pair(map_res(digit1, str::parse::<u32>), space1, CubeColor::parse),
            |res| {
                Ok::<_, ErrorKind>(Cubes {
                    color: res.1,
                    amount: res.0,
                })
            },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass]
enum CubeColor {
    GREEN,
    BLUE,
    RED,
}
impl CubeColor {
    fn parse(input: &str) -> IResult<&str, CubeColor> {
        map_res(
            alt((
                tag_no_case("green"),
                tag_no_case("blue"),
                tag_no_case("red"),
            )),
            str::parse,
        )(input)
    }
}
impl FromStr for CubeColor {
    type Err = Error<String>;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "green" => Ok(CubeColor::GREEN),
            "blue" => Ok(CubeColor::BLUE),
            "red" => Ok(CubeColor::RED),
            _ => Err(Error {
                input: s.to_string(),
                code: ErrorKind::Fail,
            }),
        }
    }
}

#[pyfunction]
fn parse_games(games: &str) -> PyResult<Vec<Game>> {
    let res: Result<Vec<Game>, _> = games.lines().map(Game::parse_line).collect();
    res.map_err(|e| PyTypeError::new_err(e.to_string()))
}

#[pymodule]
fn day02_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CubeSet>()?;
    m.add_class::<CubeColor>()?;
    m.add_class::<Cubes>()?;
    m.add_class::<Game>()?;

    m.add_function(wrap_pyfunction!(parse_games, m)?)?;

    Ok(())
}
