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
        n: usize, k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    // aの要素を+1, -1したものをbの要素として表現できるかどうかを判定する
    // つまり、a[i] - b[i]がkの倍数であるかどうかを判定する
    let mut diff = 0;
    for i in 0..n {
        diff += (a[i] as isize - b[i] as isize).abs();
    }
    let ans = if diff <= k as isize && (k as isize - diff) % 2 == 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
