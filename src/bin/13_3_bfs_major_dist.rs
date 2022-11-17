use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

fn bfs(g: &Graph, s: usize) -> Vec<Option<u32>> {
    let n = g.len();

    let mut dist: Vec<Option<u32>> = vec![None; n]; // s空の距離
    let mut que = VecDeque::<usize>::new(); // これから探索すべき頂点につながる訪問済みの頂点

    // 初期条件
    dist[s] = Some(0); // 始点は距離0
    que.push_back(s);

    // bfs開始
    while let Some(v) = que.pop_front() {
        // vからたどれる頂点をすべて探索
        for x in g[v].iter() {
            // すでに発見済みの頂点は探索しない
            if dist[*x].is_some() {
                continue;
            }

            // 新たな頂点xについて距離情報を更新してキューに挿入
            dist[*x] = Some(dist[v].unwrap() + 1);
            que.push_back(*x);
        }
    }

    dist
}

fn main() {
    let g: Graph = vec![
        vec![1, 2, 4],
        vec![0, 3, 4, 8],
        vec![0, 5],
        vec![1, 7, 8],
        vec![0, 1, 8],
        vec![2, 6, 8],
        vec![5, 7],
        vec![3, 6],
        vec![1, 3, 4],
    ];

    let dist = bfs(&g, 0_usize);
    println!("distance from 0:{:?}", dist);
}
