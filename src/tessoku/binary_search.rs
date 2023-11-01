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

    println!("{:?}", a);
}
