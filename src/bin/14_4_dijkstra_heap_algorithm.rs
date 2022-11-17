use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INF: i32 = 10_i32.pow(5);

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    w: i32,
}

// ヒープで利用するvとdist[v]のベア
#[derive(Eq, PartialEq)]
struct State {
    pos: usize, // v
    dist: i32,  // ヒープに加えるdist[v]
}

// ヒープで利用するStateのふるまい(binary_heapで最小値を取得するために逆順にしてる)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| other.pos.cmp(&self.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Graph = Vec<Vec<Edge>>;

fn choice_min<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        return true;
    }
    false
}

fn dijkstra_heap(g: &Graph, s: usize) -> Vec<i32> {
    let mut dist = vec![INF; g.len()];
    dist[s] = 0;

    // (v, dist[v])のベアのヒープを作る
    let mut prior_que = BinaryHeap::new();
    prior_que.push(State {
        pos: s,
        dist: dist[s],
    });

    while let Some(State { pos: v, dist: d }) = prior_que.pop() {
        // 最小値を取得
        if d > dist[v] {
            continue;
        } // ヒープに置ける重複値（ゴミ）だった場合

        // 頂点vを始点とした各辺を緩和
        for e in g[v].iter() {
            if choice_min(&mut dist[e.to], d + e.w) {
                // 更新があるならヒープに新たに挿入
                prior_que.push(State {
                    pos: e.to,
                    dist: dist[e.to],
                });
            }
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

    let dist = dijkstra_heap(&g, 0_usize);
    println!("distance from 0:{:?}", dist);
}
