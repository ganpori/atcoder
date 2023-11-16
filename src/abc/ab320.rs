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

// 順序を与えて条件を単純化する。
// どちらも推せる可能性があっても順序を与えて場合分けするとシンプル
// 解くというよりはシミュレーションをする。その仕方を工夫する。
pub fn c() {
    input! {
        m:usize,
        s: [Chars;3],
    }

    let mut vec_target_value = Vec::<usize>::new();
    for i in 0..=9 as usize {
        let i_char = std::char::from_digit(i as u32, 10).unwrap();
        if s[0].contains(&i_char) && s[1].contains(&i_char) && s[2].contains(&i_char) {
            vec_target_value.push(i);
        }
    }
    if vec_target_value.len() == 0 {
        println!("-1");
        return;
    }
    let mut ans = 3 * m + 1;
    for target_value in &vec_target_value {
        let target_char = std::char::from_digit(*target_value as u32, 10).unwrap();
        // println!("target_char: {}", target_char);
        for perm in (0..3 as usize).permutations(3) {
            let result = solve(&s, target_char, perm);
            // println!("result:{}", result);
            ans = std::cmp::min(result, ans);
            // println!("ans:{}", ans);
        }
    }
    println!("{}", ans);
}

fn solve(s: &Vec<Vec<char>>, target_char: char, perm: Vec<usize>) -> usize {
    // println!("{:?}", perm);
    let m = s[0].len();
    for i in 0..m {
        if s[perm[0]][i] == target_char {
            for j in i + 1..i + 1 + m {
                if s[perm[1]][j % m] == target_char {
                    for k in j + 1..j + 1 + m {
                        if s[perm[2]][k % m] == target_char {
                            return k;
                        }
                    }
                }
            }
        }
    }
    3 * m + 1
}
