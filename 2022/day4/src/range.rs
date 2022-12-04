use std::ops::RangeInclusive;

use color_eyre::{eyre::eyre, Result};

pub fn range_tuple(s: &str) -> Result<(i32, i32)> {
    let Some((minstr, maxstr)) = s.split_once('-') else {
        return Err(eyre!("Invalid input: invalid range: {}", s));
    };
    Ok((minstr.parse::<i32>()?, maxstr.parse::<i32>()?))
}

pub fn range_from_str(s: &str) -> Result<RangeInclusive<u32>> {
    let Some((minstr, maxstr)) = s.split_once('-') else {
        return Err(eyre!("Invalid input: invalid range: {}", s));
    };
    Ok(minstr.parse::<u32>()?..=maxstr.parse::<u32>()?)
}

pub trait RangeInclusiveExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn intersects_range(&self, other: &Self) -> bool;
}

impl<T> RangeInclusiveExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn intersects_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}
