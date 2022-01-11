use nom::{combinator::map, IResult};

/// Parse an [u16]
///
/// # Definition
///
/// ```txt
/// raw_value = [A-Fa-f0-9]{1,4}
/// ```
///
/// # Examples
///
/// ```
/// # use gb_breakpoint::parser::raw_value;
/// assert_eq!(raw_value("42"), Ok(("", 0x42)));
/// ```
pub fn value(input: &str) -> IResult<&str, u16> {
    use nom::bytes::complete::take_while_m_n;

    map(take_while_m_n(1, 4, |c: char| c.is_ascii_hexdigit()), |s| {
        u16::from_str_radix(s, 16).unwrap()
    })(input)
}
