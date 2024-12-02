#![feature(iter_map_windows)]

use itertools::Itertools;

const INPUT: &str = include_str!("../input");

pub mod bonus {
    use super::*;

    pub fn solution() -> u32 {
        INPUT
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse::<u32>().unwrap()))
            .filter(|l| {
                l.clone()
                    .combinations(l.clone().count() - 1)
                    .map(|c| c.into_iter().map_windows(|&[l, r]| r as i32 - l as i32))
                    .any(|c| {
                        c.clone().map(i32::signum).all_equal()
                            && c.clone().all(|d| matches!(d.abs(), 1..=3))
                    })
            })
            .count() as _
    }
}

pub fn solution() -> u32 {
    INPUT
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .map_windows(|&[l, r]| r as i32 - l as i32)
        })
        .filter(|l| {
            l.clone().map(i32::signum).all_equal() && l.clone().all(|d| matches!(d.abs(), 1..=3))
        })
        .count() as _
}
