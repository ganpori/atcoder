// vecの要素がvecの時、中身のvecの値でsort:ab.sort_by(|a, b| a[0].cmp(&b[0]));
//　境界のややこしい話をできる限りデータ構造に押し込める。さもないと謎の境界エラーに苦しめられる。
fn c() {
    input! {
      n:usize,
      k:usize,
    mut ab:[[usize;2];n]
    }

    ab.sort_by(|a, b| a[0].cmp(&b[0]));
    // 日付に対して飲む数が単調減少なので二分探索が可能。
    // 累積和のほうが良い？
    // そもそも探索せずとも関数作れる？
    // 変化点だけ検知すればよい
    let mut num_medicine = HashMap::new();
    let mut sum = 0;
    for i in 0..n {
        *num_medicine.entry(ab[i][0]).or_insert(0) += ab[i][1];
        sum += ab[i][1];
    }
    if sum <= k {
        println!("1");
        return;
    }

    for day in num_medicine.keys().sorted() {
        // day+1日からnum_medicine[key]個飲む薬が減る
        sum -= num_medicine[day];
        if sum <= k {
            println!("{}", day + 1);
            return;
        }
    }
}
