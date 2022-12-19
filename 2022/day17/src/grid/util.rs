pub fn display_top_bot_line(
    width: isize,
    indent: usize,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for _ in 0..indent {
        write!(f, " ")?;
    }
    write!(f, "+")?;
    for _ in 0..width {
        write!(f, "-")?;
    }
    writeln!(f, "+")
}
