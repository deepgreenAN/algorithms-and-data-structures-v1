type Graph = Vec<Vec<usize>>;

fn dfs_recur(g: &Graph, v: usize, seen: &mut [bool], path: &mut Vec<usize>) {
    seen[v] = true; // vを訪問済にする
    path.push(v); // パスにvを追加

    // v からいける各頂点next_vについて
    for next_v in g[v].iter() {
        if seen[*next_v] {
            continue;
        } // next_vが訪問済みならば探索しない
        dfs_recur(g, *next_v, seen, path);
    }
}

fn dfs(g: &Graph, initial_v: usize, seen: &mut [bool]) -> Vec<usize> {
    let mut path: Vec<usize> = Vec::new();

    dfs_recur(g, initial_v, seen, &mut path);
    path
}

fn main() {
    let g: Graph = vec![
        vec![5],
        vec![3, 6],
        vec![5, 7],
        vec![0, 7],
        vec![1, 2, 6],
        vec![],
        vec![7],
        vec![0],
    ];

    let mut seen = vec![false; g.len()];
    let mut all_path: Vec<Vec<usize>> = Vec::new();

    for i in 0..g.len() {
        if seen[i] {
            continue;
        } // すでに訪問済みならば探索しない
        let sub_path = dfs(&g, i, &mut seen);
        all_path.push(sub_path);
    }

    println!("all_path: {:?}", all_path);
}
