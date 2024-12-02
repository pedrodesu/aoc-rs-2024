use itertools::Itertools;

const INPUT: &str = include_str!("../input");

pub mod bonus {
    use super::*;

    pub fn solution() -> u32 {
        let (a, b) = INPUT
            .lines()
            .map(|l| {
                let mut it = l.split_whitespace();
                let (l, r) = it
                    .by_ref()
                    .map(|n| n.parse::<u32>().unwrap())
                    .next_tuple()
                    .unwrap();

                assert_eq!(it.count(), 0);

                (l, r)
            })
            .unzip::<_, _, Vec<u32>, Vec<u32>>();

        a.into_iter()
            .map(|n| n * b.iter().copied().filter(|&n2| n == n2).count() as u32)
            .sum()
    }
}

pub fn solution() -> u32 {
    let (mut a, mut b) = INPUT
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let (l, r) = it
                .by_ref()
                .map(|n| n.parse::<u32>().unwrap())
                .next_tuple()
                .unwrap();

            assert_eq!(it.count(), 0);

            (l, r)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    a.sort_unstable();
    b.sort_unstable();

    a.into_iter().zip(b).map(|(l, r)| l.abs_diff(r)).sum()
}