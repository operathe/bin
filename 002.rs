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
        s: usize,
    }
    // s個の()のみからなる文字列である。()この場合は２個である。
    // この文字列を並べて、正しい文字列にになるようにする。
    // 正しい文字列とは()(),(()),((())),()()()などである。
    //文字列を辞書順に出力する
    let mut ans = vec![];
    for i in 0..1 << s {
        let mut left = 0;
        let mut right = 0;
        let mut ok = true;
        for j in 0..s {
            if i >> j & 1 == 0 {
                left += 1;
            } else {
                right += 1;
            }
            if left < right {
                ok = false;
                break;
            }
        }
        if left != right {
            ok = false;
        }
        if ok {
            let mut t = String::new();
            for j in 0..s {
                if i >> j & 1 == 0 {
                    t.push('(');
                } else {
                    t.push(')');
                }
            }
            ans.push(t);
        }
    }
    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}
