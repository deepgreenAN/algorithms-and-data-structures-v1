//const INF: i32 = 10_i32.pow(3);

fn main() {
    // 値はhttps://atcoder.jp/contests/abc023/tasks/abc023_dを参考にした
    let n: usize = 4;
    let h: Vec<i32> = vec![5, 12, 14, 21];
    let s: Vec<i32> = vec![6, 4, 7, 2];

    let mut left = 0; // 高度の左端
                      //let mut right = INF;  // 高度の右端
    let mut right = h
        .iter()
        .zip(s.iter())
        .map(|(h_i, s_i)| h_i + (n as i32) * s_i)
        .max()
        .unwrap(); // N秒後の最大の高度を計算

    // 二分探索
    while (right - left) > 1 {
        let mid = (left + right) / 2; // 区間の中心の高度

        // 判定
        let mut can_shoot = true;
        let mut t = vec![0; n];

        for i in 0..n {
            // そもそもmidが初期高度より低かったらfalse <- 初期高度より低いペナルティは存在しないため
            if mid < h[i] {
                can_shoot = false;
            } else {
                t[i] = (mid - h[i]) / s[i]; // ペナルティをmidとしたときのiの飛ぶ時間(残りの時間)
            }
        }

        t.sort(); // 時間が差し迫っている順にソート
        for (i, time) in t.iter().enumerate() {
            // 順に風船を撃っていく
            if *time < i as i32 {
                // 残りの時間が経過時間を下回ってしまった場合
                can_shoot = false;
            }
        }

        if can_shoot {
            right = mid; // 区間の中心を右端に(最小値を求めるため)
        } else {
            left = mid; // 区間の中心を左端に
        }
    }

    println!("min cost:{:?}", right);
}
