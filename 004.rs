use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h]
    }

    let mut row_sums = vec![0; h];
    let mut col_sums = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            row_sums[i] += a[i][j];
            col_sums[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            if j != 0 {
                print!(" ");
            }
            print!("{}", row_sums[i] + col_sums[j] - a[i][j]);
        }
        println!();
    }
}

// #[fastout]
// fn main() {
//     input! {
//         h: usize, w: usize,
//         a: [[usize; w]; h]
//     }

//     let row_sums: Vec<usize> = a.iter().map(|row| row.iter().sum()).collect();
//     let col_sums: Vec<usize> = (0..w).map(|j| a.iter().map(|row| row[j]).sum()).collect();

//     for i in 0..h {
//         let row: Vec<String> = (0..w)
//             .map(|j| (row_sums[i] + col_sums[j] - a[i][j]).to_string())
//             .collect();
//         println!("{}", row.join(" "));
//     }
// }
