#![feature(macro_metavar_expr)]
pub mod combinator;
pub mod string;
pub mod number;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> Result<(Output, &'a str), &'a str>;
}

impl<'a, Output, F> Parser<'a, Output> for F
    where
        F: Fn(&'a str) -> Result<(Output, &'a str), &'a str>
{
    fn parse(&self, input: &'a str) -> Result<(Output, &'a str), &'a str> {
        self(input)
    }
}

//generators
pub fn chr_match<'a>(predicate: impl Fn(char) -> bool) -> impl Parser<'a, char> {
    move |input: &'a str| {
        input.chars().next()
            .filter(|c| predicate(*c))
            .map(|c| (c, &input[c.len_utf8()..]))
            .ok_or(input)
    }
}

pub fn chr<'a>(chr: char) -> impl Parser<'a, char> {
    chr_match(move |c| c == chr)
}

pub fn or_trie<'a, Output>(elements: &[(&str, Output)]) {//-> impl Parser<'a, Output> {
    /*
        build trie
        return parser that finds the largest prefix of input in the trie and the associated value
        err if it doesn't exist
    */
    unimplemented!()
}


macro_rules! tuple_inner {
    ($( $parser: ident $output: ident $name: ident),+) => {
        impl<'a, $($output, $parser: Parser<'a, $output>),+> Parser<'a, ($($output),+)> for ($($parser),+) {
            fn parse(&self, old_input: &'a str) -> Result<(($($output),+), &'a str), &'a str> {
                 let mut input = old_input;
                 let ($(mut $name),+);
                 $(
                    if let Ok((new_output, new_input)) = self.${index()}.parse(input) {
                        $name = new_output;
                        input = new_input;
                    } else {
                        return Err(old_input)
                    };
                 )+
                 Ok((($($name),+), input))
            }
        }
    }
}

macro_rules! tuple {
    ($parser_1: ident $output_1: ident $name_1: ident, $( $parser: ident $output: ident $name: ident),+) => {
        tuple_inner!($parser_1 $output_1 $name_1, $($parser $output $name),+);
        tuple!($($parser $output $name),+);
    };

    ($parser_1: ident $output_1: ident $name_1: ident) => {
        //tuple_inner!($parser_1 $output_1 $name_1);
    };
}

//supporting tuples up to len 20
tuple!(
    P1 O1 o1, P2 O2 o2, P3 O3 o3, P4 O4 o4, P5 O5 o5,
    P6 O6 o6, P7 O7 o7, P8 O8 o8, P9 O9 o9, P10 O10 o10,
    P11 O11 o11, P12 O12 o12, P13 O13 o13, P14 O14 o14, P15 O15 o15,
    P16 O16 o16, P17 O17 o17, P18 O18 o18, P19 O19 o19, P20 O20 o20
);

#[macro_export]
macro_rules! or {
    ($lhs: expr, $($rhs: expr),+) => {{
        $crate::combinator::or($lhs, $crate::or!($($rhs),+))
    }};
    
    ($sole: expr) => {{
        $sole
    }};
}

#[macro_export]
macro_rules! str {
    ($lhs: expr, $($rhs: expr),+) => {{
        $crate::combinator::string($lhs, $crate::str!($($rhs),+))
    }};
    
    ($sole: expr) => {{
        $sole
    }};
}