type Graph = Vec<Vec<usize>>;

fn recur(g: &Graph, v: usize, seen: &mut [bool], order: &mut Vec<usize>) {
    seen[v] = true; // vを訪問済にする

    // v からいける各頂点next_vについて
    for next_v in g[v].iter() {
        if seen[*next_v] {
            continue;
        } // next_vが訪問済みならば探索しない
        recur(g, *next_v, seen, order);
    }

    // v-outを記録する
    order.push(v);
}

fn topological_sort(g: &Graph) -> Vec<usize> {
    let mut seen = vec![false; g.len()];
    let mut order: Vec<usize> = Vec::new();

    for x in 0..g.len() {
        if seen[x] {
            continue;
        } // すでに訪問済みならば探索しない
        recur(g, x, &mut seen, &mut order);
    }

    order.reverse();
    order
}

fn main() {
    let g: Graph = vec![
        vec![5],
        vec![3, 6],
        vec![5, 7],
        vec![0, 7],
        vec![1, 2, 6],
        vec![],
        vec![2, 7],
        vec![0],
    ];

    let order = topological_sort(&g);

    println!("order:{:?}", order);
}
