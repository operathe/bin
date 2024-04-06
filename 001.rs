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
        n: usize, l: usize,
        k: usize,
        a: [usize; n],
    }
    //lをaの箇所でk+1個に分割することを考える。最小の長さxが最大になるようにする。
    //xを二分探索する。
    let mut left = 0;
    let mut right = l + 1;
    while right - left > 1 {
        let mid = (left + right) / 2;
        let mut cnt = 0;
        let mut prev = 0;
        for &a_i in &a {
            if a_i - prev >= mid {
                cnt += 1;
                prev = a_i;
            }
        }
        if l - prev >= mid {
            cnt += 1;
        }
        if cnt >= k + 1 {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
