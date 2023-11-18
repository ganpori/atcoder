use proconio::input;

// N=2*10^5で全列挙するとO(n^2)ではTLEしてしまう
// 部分列を左端と右端を決めて全列挙する方法は覚えといて問題ない
// 両端キューvecdequeなどで両端から調べていこうとしたがstackで片側から連続性だけみれば十分だった
// アルファベットの列挙は'a'..='z'でできる
// この手法はランレングス圧縮と呼ぶらしい
// stackで実装しなくても左端と右端を尺取り法のように操作していけば右端ー左端で連続している数判定できる
// 連続している間は右端を右に伸ばす。連続しなくなったら左端を右端の位置に持ってくる
// 左端が文字列の最右端まできたらループ終了
// hasmapの使い方に時間とられた。dpで部分文字列操作する方法かどうかに気を取られた。
// 値の更新はinsert、そのhashの有無にかかわらず。値取得はgetメソッドか[&char]
// ハッシュテーブルはハッシュ衝突が起きない限りO(1)の計算量で削除、検索、挿入が可能
// よくハッシュ衝突するとO(N)になるらしい。衝突要因はvalue?key?よくわからん。
pub fn c() {
    input! {
      n:usize,
      mut s:Chars,
    }

    let mut dict_max_num = HashMap::new();
    for alp in 'a'..='z' {
        dict_max_num.insert(alp, 0 as usize);
    }
    let mut old_value = s.pop().unwrap();
    let mut num_continue = 1;
    dict_max_num.insert(old_value, num_continue);

    while s.len() > 0 {
        // println!("{:?}", s);
        let value = s.pop().unwrap();
        if old_value == value {
            num_continue += 1;
            dict_max_num.insert(
                old_value,
                std::cmp::max(dict_max_num[&old_value], num_continue),
            );
        } else {
            num_continue = 1;
            dict_max_num.insert(
                old_value,
                std::cmp::max(dict_max_num[&old_value], num_continue),
            );
            old_value = value;
        }
    }
    let mut sum = 0;
    for alp in 'a'..='z' {
        sum += dict_max_num[&alp];
    }
    println!("{:?}", sum);
}

pub fn b() {
    input! {
      n:usize,
      a:[usize;n]
    }
    let max_value = a.iter().max().unwrap();
    let mut m = 0;
    for value in a.iter() {
        if *value != *max_value {
            m = std::cmp::max(*value, m);
        }
    }
    println!("{}", m);
}

pub fn a() {
    input! {
      s:Chars,
    }
    for i in 0..s.len() - 1 {
        print!("{} ", s[i]);
    }
    print!("{}", s[s.len() - 1]);
}
