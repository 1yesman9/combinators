//generators of Parser<&str>
use crate::{chr_match, Parser};
use crate::{str, or, combinator::{string, or}};
use crate::combinator::zero_or_more;

pub fn chr_str_match<'a>(predicate: impl Fn(char) -> bool) -> impl Parser<'a, &'a str> {
    let chr_parser = chr_match(predicate);
    move |input: &'a str| {
        chr_parser.parse(input)
            .map(|(c, remaining)| (&input[0..c.len_utf8()], remaining))
    }
}

pub fn chr_str<'a>(chr: char) -> impl Parser<'a, &'a str> {
    chr_str_match(move |c| c == chr)
}

//this is an infallable parser
pub fn zero_or_more_chr<'a>(predicate: impl Fn(char) -> bool) -> impl Parser<'a, &'a str> {
    let parser = chr_match(predicate);
    move |original: &'a str| {
        let mut input = original;
        let mut total_len = 0;
        while let Ok((char, remaining)) = parser.parse(input) {
            total_len += char.len_utf8();
            input = remaining;
        }
        Ok((&original[0..total_len], input))
    }
}

pub fn one_or_more_chr<'a>(predicate: impl Fn(char) -> bool) -> impl Parser<'a, &'a str> {
    let mut parser = zero_or_more_chr(predicate);
    move |input: &'a str| {
        let (result, remaining) = parser.parse(input).unwrap(); //safety: zero_or_more_is infallible
        if remaining != input { Ok((result, remaining)) } else { Err(input) }
    }
}

pub fn literal<'a>(literal: &str) -> impl Parser<'a, &'a str> + '_ {
    move |input: &'a str| {
        input.get(0..literal.len())
            .filter(|prefix| *prefix == literal)
            .map(|prefix| (prefix, &input[prefix.len()..]))
            .ok_or(input)
    }
}

pub fn digits<'a>() -> impl Parser<'a, &'a str> {
    one_or_more_chr(|c| c.is_numeric())
}

pub fn identifier<'a>() -> impl Parser<'a, &'a str> {
    str!(chr_str_match(|c| c.is_alphabetic() || c == '_'), zero_or_more_chr(|c| c.is_alphanumeric() || c == '_'))
}

pub fn alphabetic<'a>() -> impl Parser<'a, &'a str> {
    one_or_more_chr(|c| c.is_alphabetic())
}

pub fn whitespace<'a>() -> impl Parser<'a, &'a str> {
    one_or_more_chr(|c| c.is_whitespace())
}

pub fn quoted<'a>() -> impl Parser<'a, &'a str> {
    or!(
        str!(chr_str('"'), zero_or_more_chr(|c| c != '"'), chr_str('"')),
        str!(chr_str('\''), zero_or_more_chr(|c| c != '\''), chr_str('\''))
    )
}