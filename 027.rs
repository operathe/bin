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
        mut s: [String; n],
    }
    //s[i]が初めて出現するインデックスを出力する
    let mut set = HashSet::new();
    for index in 0..n {
        if set.contains(&s[index]) {
            continue;
        }
        set.insert(&s[index]);
        println!("{}", index + 1);
    }
}
