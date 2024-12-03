use std::str::Chars;

use itertools::Itertools;

const INPUT: &str = include_str!("../input");

pub mod bonus {
    use super::*;

    pub fn solution() -> u32 {
        let mut counts = true;

        let mut collection = INPUT
            .match_indices("do")
            .chain(INPUT.match_indices("don't"))
            .chain(INPUT.match_indices("mul"))
            .collect::<Vec<_>>();

        collection.sort_by_key(|&(i, _)| i);

        collection
            .into_iter()
            .filter_map(|(i, v)| {
                let mut it = INPUT[i + v.len()..].chars();

                if it.next()? != '(' {
                    return None;
                }

                match v {
                    "mul" => {
                        let take_n = |it: &mut Chars| {
                            it.take_while_ref(|c| c.is_ascii_digit())
                                .collect::<String>()
                                .parse::<u32>()
                                .ok()
                        };

                        let lhs = take_n(&mut it)?;

                        if it.next()? != ',' {
                            return None;
                        }

                        let rhs = take_n(&mut it)?;

                        if it.next()? != ')' {
                            return None;
                        }

                        if counts { Some((lhs, rhs)) } else { None }
                    }
                    _ => {
                        if it.next()? != ')' {
                            return None;
                        }

                        counts = v == "do";

                        None
                    }
                }
            })
            .map(|(l, r)| l * r)
            .sum()
    }
}

pub fn solution() -> u32 {
    INPUT
        .match_indices("mul")
        .filter_map(|(i, _)| {
            let mut it = INPUT[i + 3..].chars();

            if it.next()? != '(' {
                return None;
            }

            let take_n = |it: &mut Chars| {
                it.take_while_ref(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .ok()
            };

            let lhs = take_n(&mut it)?;

            if it.next()? != ',' {
                return None;
            }

            let rhs = take_n(&mut it)?;

            if it.next()? != ')' {
                return None;
            }

            Some((lhs, rhs))
        })
        .map(|(l, r)| l * r)
        .sum()
}
