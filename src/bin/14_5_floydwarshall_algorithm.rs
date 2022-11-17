use std::cmp::min;

const INF: i32 = 10_i32.pow(5);

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    w: i32,
}

type Graph = Vec<Vec<Edge>>;

fn floydwarshall(g: &Graph) -> Vec<Vec<i32>> {
    let mut dp: Vec<Vec<i32>> = vec![vec![INF; g.len()]; g.len()];

    // 初期条件
    for (i, e_vec) in g.iter().enumerate() {
        for e in e_vec.iter() {
            dp[i][e.to] = e.w;
        }
    }

    for (i, dp_row) in dp.iter_mut().enumerate() {
        dp_row[i] = 0; // dp[i][i] = 0
    }

    // dp遷移(フロイド・ワーシャル法)
    for k in 0..g.len() {
        for i in 0..g.len() {
            for j in 0..g.len() {
                let min_dist = min(dp[i][j], dp[i][k] + dp[k][j]);
                dp[i][j] = min_dist;
            }
        }
    }

    dp
}

fn main() {
    let g: Graph = vec![
        vec![Edge { to: 1, w: 3 }, Edge { to: 2, w: 5 }],
        vec![Edge { to: 2, w: 4 }, Edge { to: 3, w: 12 }],
        vec![Edge { to: 3, w: 9 }, Edge { to: 4, w: 4 }],
        vec![Edge { to: 5, w: 2 }],
        vec![Edge { to: 3, w: 7 }, Edge { to: 5, w: 8 }],
        vec![],
    ];

    let dist = floydwarshall(&g);
    println!("distance:{:?}", dist);
}
