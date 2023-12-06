use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use nom::bytes::complete::*;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::{character::complete::*, multi::many1, sequence::*};
use nom::{Finish, IResult};
use std::collections::hash_set::Iter;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

#[derive(Debug, Clone)]
pub struct SeedRangeAlmanac {
    seed_ranges: Vec<Range<u64>>,
    maps: Vec<Map>,
}
impl SeedRangeAlmanac {
    pub fn has_seed(&self, seed: &u64) -> bool {
        self.seed_ranges.iter().any(|range| range.contains(seed))
    }
    pub fn seed_location(&self, seed: u64) -> u64 {
        let mut next = seed;
        for map in &self.maps {
            //print!("{} -> {}: {}", map.from, map.to, next);
            next = map.find(next);
            //println!("-> {}", next);
        }
        next
    }
    pub fn lowest_seed(&self) -> u64 {
        let mut current_target = 0;
        loop {
            //dbg!(current_target);
            let mut current_vals = vec![current_target];
            for map in self.maps.iter().rev() {
                current_vals = current_vals
                    .into_iter()
                    .flat_map(|out| map.inputs_for_output(out))
                    .collect_vec();
            }
            // at this point the vals wil be possible input seeds, see if any exist in the almanac
            if let Some(seed) = current_vals.iter().find(|seed| self.has_seed(seed)) {
                return *seed;
            }
            current_target += 1;
        }
    }
    pub fn parse(input: &str) -> Result<SeedRangeAlmanac> {
        let seed_parser = preceded(
            tag("seeds: "),
            separated_list1(
                space1,
                map_res(
                    separated_pair(
                        map_res(digit1, str::parse::<u64>),
                        space1,
                        map_res(digit1, str::parse::<u64>),
                    ),
                    |res| Ok::<_, ErrorKind>(res.0..res.0 + res.1),
                ),
            ),
        );
        let maps_parser = separated_list1(multispace1, Map::parse);
        let parser = terminated(
            separated_pair(seed_parser, multispace1, maps_parser),
            multispace0,
        );

        map_res(parser, |res| {
            Ok::<_, ErrorKind>(SeedRangeAlmanac {
                maps: res.1,
                seed_ranges: res.0,
            })
        })(input)
        .finish()
        .map(|ok| ok.1)
        .map_err(|e| anyhow!("Error while reading almanac: {e}"))
    }
}

#[derive(Debug, Clone)]
pub struct Almanac {
    maps: Vec<Map>,
    seeds: HashSet<u64>,
}
impl Almanac {
    pub fn seeds(&self) -> Iter<u64> {
        self.seeds.iter()
    }
    pub fn seed_location(&self, seed: u64) -> u64 {
        let mut next = seed;
        for map in &self.maps {
            //print!("{} -> {}: {}", map.from, map.to, next);
            next = map.find(next);
            //println!("-> {}", next);
        }
        next
    }
    pub fn parse(input: &str) -> Result<Almanac> {
        let seed_parser = preceded(
            tag("seeds: "),
            separated_list1(space1, map_res(digit1, str::parse::<u64>)),
        );
        let maps_parser = separated_list1(multispace1, Map::parse);
        let parser = terminated(
            separated_pair(seed_parser, multispace1, maps_parser),
            multispace0,
        );

        map_res(parser, |res| {
            Ok::<_, ErrorKind>(Almanac {
                maps: res.1,
                seeds: HashSet::from_iter(res.0.into_iter().collect::<HashSet<_>>()),
            })
        })(input)
        .finish()
        .map(|ok| ok.1)
        .map_err(|e| anyhow!("Error while reading almanac: {e}"))
    }
}

#[derive(Debug, Clone)]
struct Map {
    from: String,
    to: String,
    mappings: Vec<(Range<u64>, Range<u64>)>,
}
impl Map {
    fn parse(input: &str) -> IResult<&str, Map> {
        let header_parser = terminated(
            separated_pair(alpha1, tag("-to-"), alpha1),
            tuple((space1, tag("map:"), multispace1)),
        );
        let mapping_parser = separated_pair(
            map_res(digit1, str::parse::<u64>),
            space1,
            separated_pair(
                map_res(digit1, str::parse::<u64>),
                space1,
                map_res(digit1, str::parse::<u64>),
            ),
        );
        let parser = pair(header_parser, separated_list1(multispace1, mapping_parser));
        map_res(parser, |res| {
            Ok::<_, ErrorKind>(Map {
                from: res.0 .0.to_string(),
                to: res.0 .1.to_string(),
                mappings: res
                    .1
                    .iter()
                    .map(|map| (map.1 .0..(map.1 .0 + map.1 .1), (map.0..(map.0 + map.1 .1))))
                    .collect(),
            })
        })(input)
    }
    fn find(&self, input: u64) -> u64 {
        if let Some((in_range, out_range)) = self
            .mappings
            .iter()
            .find(|(range, _)| range.contains(&input))
        {
            out_range.start + (input - in_range.start)
        } else {
            input
        }
    }
    fn inputs_for_output(&self, output: u64) -> Vec<u64> {
        let mut inputs = vec![];
        if self.find(output) == output {
            inputs.push(output);
        }

        for (in_range, out_range) in &self.mappings {
            if out_range.contains(&output) {
                inputs.push(in_range.start + output - out_range.start)
            }
        }
        inputs
    }
}
