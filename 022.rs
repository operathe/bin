use proconio::input;

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    }

    let gcd_ab = gcd(a, b);
    let gcd_abc = gcd(gcd_ab, c);

    let ans = (a / gcd_abc) + (b / gcd_abc) + (c / gcd_abc) - 3;
    println!("{}", ans);
}
