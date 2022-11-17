const INF: i32 = 10_i32.pow(5);

struct Edge {
    to: usize,
    w: i32,
}

type Graph = Vec<Vec<Edge>>;

fn choice_min<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        return true;
    }
    false
}

fn dijkstra(g: &Graph, s: usize) -> Vec<i32> {
    let mut used: Vec<bool> = vec![false; g.len()];
    let mut dist: Vec<i32> = vec![INF; g.len()];
    dist[s] = 0; // 開始点

    for _ in 0..g.len() {
        // 辺を緩和し終わっていない頂点のうち、distが最小の頂点を探す
        let min_v = (0..g.len()).filter(|v| !used[*v]).min_by_key(|v| dist[*v]);

        // もしそのような頂点がみつからなければ、計算を終了
        // min_vを始点とした各辺を緩和する
        if let Some(min_v_usize) = min_v {
            for e in g[min_v_usize].iter() {
                let exact_dist = dist[min_v_usize] + e.w;
                choice_min(&mut dist[e.to], exact_dist);
            }
            used[min_v_usize] = true; // min_vを使用済みとする
        } else {
            break;
        }
    }
    dist
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

    let dist = dijkstra(&g, 0_usize);
    println!("distance from 0:{:?}", dist);
}
