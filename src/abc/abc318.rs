fn a() {
    input! {
        n:usize,
        m:usize,
        p:usize
    }
    if n >= m {
        let num_moon = (n - m) / p + 1;
        println!("{}", num_moon);
    } else {
        println!("0");
    }
}

fn b() {
    input! {
        n:usize,
        vec_rectangle:[[usize;4];n]
    }

    let mut count_cordinate: [[usize; 100]; 100] = [[0; 100]; 100];
    for rectangle in vec_rectangle {
        for x in rectangle[0]..rectangle[1] {
            for y in rectangle[2]..rectangle[3] {
                count_cordinate[x][y] += 1;
            }
        }
    }
    // println!("{:?}", count_cordinate);
    let mut count = 0;
    for x in 0..100 {
        for y in 0..100 {
            if count_cordinate[x][y] >= 1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

//下記のようなc++と同じ実装にするとわかりやすそう。
//std::lower_bound() は，ソートされた配列に対して二分探索を行い，指定した key 以上の要素の内，一番左側の要素の位置（最小のインデックス）をイテレータで返します．
// その値以上の最小のindex取得を身にしみこませる。=の時はrightをmiddleにする。<のときはleftをmiddle+1に寄せていく。
//　二分探索で溶ける問題は累積和でもとけがちだからそっちも検討するようにする
fn c() {
    input! {
        n:usize,
        d:usize,
        p:usize,
    mut f:[usize;n]
    }
    f.sort();

    let mut fd = vec![0 as usize; n]; // 割合計算すると型がややこしいから整数倍して計算する
    let mut sum_f = 0;
    for i in 0..n {
        fd[i] = d * f[i];
        sum_f += f[i];
    }
    if fd[n - 1] <= p {
        println!("{}", sum_f); // 全部pより効率いいならもとの料金で旅する
        return;
    } else if fd[0] >= p {
        //全部pより効率悪いならpだけで旅する。ただ境界の扱いには注意
        let num_set = n / d;
        let set_price = p * num_set;

        let num_amari_day = n % d;
        let mut amari_price = 0;
        for i in 0..num_amari_day {
            amari_price += f[i];
        }
        let min_value = std::cmp::min(set_price + amari_price, set_price + p);
        println!("{}", min_value);
        return;
    }

    let mut left_index: usize = 0;
    let mut right_index: usize = n - 1;
    let mut border_index = (left_index + right_index) / 2;
    while left_index < right_index {
        if fd[border_index] > p {
            right_index = border_index; // -1すると指定した値より小さいindexも対象になってしまう。
        } else if fd[border_index] < p {
            left_index = border_index + 1; // この+1で指定した値より以上の値に向かうようになる
        } else if fd[border_index] == p {
            right_index = border_index; // 値が一緒の時はrightを寄せてくることでその値の中でも小さいindexを参照するようになる
        }
        border_index = (left_index + right_index) / 2;
    }
    // let num_over_day = n - border_index;
    // println!("f_sorted:{:?}", f);
    // println!("index:{}", border_index);
    // println!("f:{}", f[border_index]);
    // println!("fd:{}", fd[border_index]);

    let num_set = (n - border_index) / d;
    let set_price = p * num_set;

    let amari_day = n - num_set * d;
    let mut amari_price = 0;
    for i in 0..amari_day {
        amari_price += f[i];
    }

    let mut new_amari_day: usize = 0;
    if amari_day >= d {
        new_amari_day = amari_day - d;
    }
    let mut new_amari_price = 0;
    for i in 0..new_amari_day {
        new_amari_price += f[i]
    }
    let min_value = std::cmp::min(set_price + amari_price, set_price + p + new_amari_price);
    println!("{}", min_value);
}
