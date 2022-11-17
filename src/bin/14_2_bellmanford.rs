const INFINITY: i32 = 10_i32.pow(5);

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

fn bellman_ford(g: &Graph, s: usize) -> Result<Vec<i32>, String> {
    let mut dist: Vec<i32> = vec![INFINITY; g.len()]; // None(INF)とする．
    dist[s] = 0; // 開始点

    for iter in 0..g.len() {
        // たかだか|V|回の反復
        let mut update = false; // 更新が発生したかどうか
        for v in 0..g.len() {
            // vにつながる辺について緩和
            if dist[v] >= INFINITY {
                continue;
            } // dist[v]がinf(頂点から到達できない)のときは緩和を行わない

            for e in g[v].iter() {
                // 緩和処理を行う。更新されたらupdateをtrueにする
                let provisual_dist = dist[v] + e.w; // 暫定距離
                if choice_min(&mut dist[e.to], provisual_dist) {
                    update = true;
                }
            }
        }

        // 更新が行われなければ、すでに最短路が求められている
        if !update {
            break;
        }

        // N回目の反復で更新が行われたならば、負回路をもつ。
        if iter == g.len() - 1 && update {
            return Err("This graph has negative circle.".to_string());
        }
    }

    Ok(dist)
}

fn main() {
    let g: Graph = vec![
        vec![Edge { to: 1, w: 3 }, Edge { to: 3, w: 100 }],
        vec![
            Edge { to: 2, w: 50 },
            Edge { to: 3, w: 57 },
            Edge { to: 4, w: -4 },
        ],
        vec![
            Edge { to: 3, w: -10 },
            Edge { to: 4, w: -5 },
            Edge { to: 5, w: 100 },
        ],
        vec![Edge { to: 1, w: -5 }],
        vec![
            Edge { to: 2, w: 57 },
            Edge { to: 3, w: 25 },
            Edge { to: 5, w: 8 },
        ],
        vec![],
    ];

    let dist = bellman_ford(&g, 0_usize);
    if let Ok(dist) = dist {
        print!("dist:{:?}", dist);
    }
}
