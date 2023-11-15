pub fn a() {
    input! {
        a:usize,
        b:usize,
    }
    let result = a.pow(b as u32) + b.pow(a as u32);
    println!("{}", result);
}

// 連続する部分文字列の抜き取り方。
// 文字列の両端と文字の隙間に番号を割り当てて左右の番号の取り型の組み合わせ
// 割り当てる番号の数は文字数+1になる
// https://drken1215.hatenablog.com/entry/2023/11/02/095137
pub fn b() {
    input! {
        s: Chars
    }
    let mut max_palindrome = 0;
    for left in 0..s.len() {
        for right in left + 1..=s.len() {
            // println!("l:{}, r:{}", left, right);
            // println!("{:?}", &s[left..right]);
            if is_palindrome(&s[left..right]) {
                max_palindrome = std::cmp::max(s[left..right].len(), max_palindrome);
            }
        }
    }
    println!("{}", max_palindrome);
}

fn is_palindrome(a: &[char]) -> bool {
    for i in 0..a.len() {
        if a[i] != a[a.len() - i - 1] {
            return false;
        }
    }
    true
}
