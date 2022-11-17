fn choose_min<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    if *a > b {
        *a = b;
    }
}

const INF: u32 = 10_u32.pow(5);

fn main() {
    let s: &str = "logistic";
    let t: &str = "algorithm";

    let mut dp: Vec<Vec<u32>> = vec![vec![INF; t.len() + 1]; s.len() + 1];

    // dp初期条件
    dp[0][0] = 0;

    for i in 0..s.len() + 1 {
        for j in 0..t.len() + 1 {
            // 変更操作をした場合のdpの遷移
            if i > 0 && j > 0 {
                // 最初の空白文字以外
                if s.chars().nth(i) == t.chars().nth(j) {
                    // sのi文字目とtのj文字目が一致
                    let new_value = dp[i - 1][j - 1];
                    choose_min(&mut dp[i][j], new_value);
                } else {
                    let new_value = dp[i - 1][j - 1] + 1;
                    choose_min(&mut dp[i][j], new_value);
                }
            }

            // 削除操作をした場合のdpの遷移
            if i > 0 {
                let new_value = dp[i - 1][j] + 1;
                choose_min(&mut dp[i][j], new_value);
            }

            // 挿入操作(tの削除)をした場合のdpの遷移
            if j > 0 {
                let new_value = dp[i][j - 1] + 1;
                choose_min(&mut dp[i][j], new_value);
            }
        }
    }

    // 答えの出力
    println!("levenshtein_distance:{:?}", dp[s.len()][t.len()]);
}
