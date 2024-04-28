use crate::Parser;
use crate::string::one_or_more_chr;
use crate::combinator::{and_then, map};

pub fn integer<'a, T: std::str::FromStr>() -> impl Parser<'a, T> {
    and_then(
        one_or_more_chr(|c| c.is_numeric()),
        |digits| digits.parse::<T>()
    )
}
