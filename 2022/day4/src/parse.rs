use color_eyre::{eyre::eyre, Result};

pub fn range_from_str(s: &str) -> Result<(i32, i32)> {
    let Some((minstr, maxstr)) = s.split_once('-') else {
        return Err(eyre!("Invalid input: invalid range: {}", s));
    };
    Ok((minstr.parse::<i32>()?, maxstr.parse::<i32>()?))
}
