// モジュールの概念の説明
// https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
// https://zenn.dev/newgyu/articles/3b4677b4086768

use proconio::input;

pub fn a11() {
    input! {
        n:usize,
        x:usize,
        a:[usize;n]
    }
    let mut index_max = a.len() - 1;
    let mut index_min = 0usize;
    let mut half_index;
    loop {
        half_index = (index_min + index_max) / 2;
        if a[half_index] < x {
            index_min = half_index + 1;
        } else if a[half_index] > x {
            index_max = half_index - 1;
        } else {
            println!("{}", half_index + 1);
            break;
        }
    }
}
