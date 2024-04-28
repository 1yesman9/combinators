use crate::Parser;

pub fn map<'a, T, K>(parser: impl Parser<'a, T>, map_fn: impl Fn(T) -> K) -> impl Parser<'a, K> {
    move |input: &'a str| {
        parser.parse(input).map(|(result, remaining)| (map_fn(result), remaining))
    }
}

pub fn and_then<'a, T, K, E>(parser: impl Parser<'a, T>, map_fn: impl Fn(T) -> Result<K, E>) -> impl Parser<'a, K> {
    move |input: &'a str| {
        parser.parse(input)
            .and_then(|(result, remaining)|
                map_fn(result)
                    .map(|k| (k, remaining))
                    .map_err(|_| remaining)
            )
    }
}

//this is infallible
pub fn zero_or_more<'a, Output>(mut parser: impl Parser<'a, Output>) -> impl Parser<'a, Vec<Output>> {
    move |mut input: &'a str| {
        let mut results = Vec::new();
        while let Ok((result, remaining)) = parser.parse(input) {
            results.push(result);
            input = remaining;
        }
        Ok((results, input))
    }
}

pub fn one_or_more<'a, Output>(parser: impl Parser<'a, Output>) -> impl Parser<'a, Vec<Output>> {
    let mut parser = zero_or_more(parser);
    move |input: &'a str| {
        let (result, remaining) = parser.parse(input).unwrap(); //safety: zero_or_more is infallible
        if remaining != input { Ok((result, remaining)) } else { Err(input) }
    }
}

//this is infallible
pub fn opt<'a, Output>(parser: impl Parser<'a, Output>) -> impl Parser<'a, Option<Output>> {
    move |input: &'a str| {
        Ok(match parser.parse(input) {
            Ok((result, remaining)) => (Some(result), remaining),
            Err(remaining) => (None, remaining)
        })
    }
}

pub fn opt_str<'a>(parser: impl Parser<'a, &'a str>) -> impl Parser<'a, &'a str> {
    map(opt(parser), |a| a.unwrap_or(""))
}

pub fn or<'a, Output>(left: impl Parser<'a, Output>, right: impl Parser<'a, Output>) -> impl Parser<'a, Output> {
    move |input: &'a str| {
        left.parse(input).or_else(|_| right.parse(input))
    }
}

pub fn and<'a, Output>(left: impl Parser<'a, Output>, right: impl Parser<'a, Output>) -> impl Parser<'a, Output> {
    move |input: &'a str| {
        left.parse(input)
            .and_then(|(result, remaining)| right.parse(remaining))
            .map_err(|_| input)
    }
}

//and, but returns the left
pub fn trail<'a, Output>(left: impl Parser<'a, Output>, right: impl Parser<'a, Output>) -> impl Parser<'a, Output> {
    move |input: &'a str| {
        left.parse(input)
            .and_then(|(left_res, left_rem)|
                //if the right works, return the left rest, with the right_remaining
                //if it doesn't return the original input as remaining
                right.parse(left_rem).and_then(|(right_res, right_rem)| Ok((left_res, right_rem))).map_err(|_| input)
            )
    }
}

pub fn string<'a>(left: impl Parser<'a, &'a str>, right: impl Parser<'a, &'a str>) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        left.parse(input)
            .and_then(|(result, remaining)|
                right.parse(remaining).map(|(r, remaining)| ((result.len(), r.len()), remaining))
            )
            .map(|((lhs, rhs), remaining)| (&input[0..(lhs+rhs)], remaining))
            .map_err(|_| input)
    }
}
