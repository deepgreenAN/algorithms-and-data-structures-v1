fn choose_min<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    // std::mem::replaceでもよい
    if *a > b {
        *a = b;
    }
}

const INF: i32 = 10_i32.pow(5);

fn rec(h: &Vec<i32>, dp: &mut Vec<i32>, i: usize) -> i32 {
    // 値がすでに更新されている場合（メモ化されている場合) <- 関数の最後までの実行回数は最適化されても呼び出し回数は最適化されないことに注意
    if dp[i] < INF {
        return dp[i];
    }

    // ベースケース（最初の足場の値）
    if i == 0 {
        return 0;
    }

    // 答えを表す変数をINFで初期化する
    let mut res = INF;

    // 足場i - 1から来た場合
    choose_min(&mut res, rec(h, dp, i - 1) + (h[i] - h[i - 1]).abs());

    // 足場i - 2から来た場合
    if i > 1 {
        choose_min(&mut res, rec(h, dp, i - 2) + (h[i] - h[i - 2]).abs());
    }

    // 結果をメモ化して返す
    dp[i] = res;

    dp[i]
}

fn main() {
    let n: usize = 7;
    let h: Vec<i32> = vec![2, 9, 4, 5, 1, 6, 10];

    let mut dp: Vec<i32> = vec![INF; n];

    let solution: i32 = rec(&h, &mut dp, n - 1);

    println!("solution: {:?}", solution);
}
