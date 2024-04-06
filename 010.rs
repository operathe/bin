#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[allow(non_snake_case)]
#[fastout]

fn main() {
    input! {
        n: usize,
        class_point: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let (mut class1, mut class2) = (vec![0; n + 1], vec![0; n + 1]);

    for (i, &(c, p)) in class_point.iter().enumerate() {
        class1[i + 1] = class1[i];
        class2[i + 1] = class2[i];
        match c {
            1 => class1[i + 1] += p,
            _ => class2[i + 1] += p,
        }
    }

    for &(l, r) in &lr {
        println!(
            "{} {}",
            class1[r] - class1[l - 1],
            class2[r] - class2[l - 1]
        );
    }
}
