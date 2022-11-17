fn choose_max<T: std::cmp::PartialOrd>(a: &mut T, b: T) {
    if *a < b {
        *a = b;
    }
}

fn main() {
    let n: usize = 6;
    let n_w: usize = 15;

    let weight: Vec<i32> = vec![2, 1, 3, 2, 1, 5];
    let value = vec![3, 2, 6, 1, 3, 85];

    let mut dp: Vec<Vec<i32>> = vec![vec![0; n_w + 1]; n + 1];

    for i in 0..n {
        for w in 0..n_w + 1 {
            // i番目の品物を選ぶ場合
            let weight_point = w as i32 - weight[i];
            if weight_point >= 0 {
                // 重さの条件を満たす場合
                let hold_value = dp[i][weight_point as usize] + value[i];
                choose_max(&mut dp[i + 1][w], hold_value);
            }

            // i番目の品物を選ばない場合
            let no_hold_value = dp[i][w];
            choose_max(&mut dp[i + 1][w], no_hold_value);
        }
    }

    // 最適の出力
    println!("solution: {:?}", dp[n][n_w]);
    println!("dp: {:?}", dp);
}
