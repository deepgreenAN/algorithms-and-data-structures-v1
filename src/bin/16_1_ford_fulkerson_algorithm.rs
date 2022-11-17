// use once_cell::sync::OnceCell;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;
// use std::sync::RwLock;
use std::thread_local;

// 再帰関数で使うためRwRock
// static NGRAPH: OnceCell<RwLock<Vec<Vec<usize>>>> = OnceCell::new();

thread_local! {
    static NGRAPH: Rc<RefCell<Vec<Vec<usize>>>> = Rc::new(RefCell::new(Vec::new()));
}

#[derive(Debug)]
struct Edge {
    cap: u32, // 容量
}

impl Edge {
    fn new(cap: u32) -> Self {
        Edge { cap }
    }
}

// uに接続するv
struct Graph {
    map: HashMap<(usize, usize), Edge>,
}

impl Graph {
    fn new(n: usize) -> Self {
        // NGRAPH.get_or_init(|| RwLock::new(vec![Vec::new(); n]));
        let n_graph = NGRAPH.with(|n_graph| n_graph.clone());
        *(n_graph.borrow_mut()) = vec![Vec::new(); n];

        Self {
            map: HashMap::new(),
        }
    }

    fn size(&self) -> usize {
        // NGRAPH.get().unwrap().read().unwrap().len()
        NGRAPH.with(|n_graph| n_graph.clone()).borrow().len()
    }

    // 頂点uから頂点vへ容量capの辺を張る
    // このときvからuへも容量0の辺を張っておく(逆辺になる)
    fn addedge(&mut self, (from, to): (usize, usize), cap: u32) {
        // 最低限逆辺を確保
        if self.map.insert((from, to), Edge::new(cap)).is_none() {
            self.map.insert((to, from), Edge::new(0));
        }

        // let mut n_graph_vec = NGRAPH.get().unwrap().write().unwrap();
        let n_graph = NGRAPH.with(|n_graph| n_graph.clone());
        let mut n_graph_vec = n_graph.borrow_mut();

        n_graph_vec[from].push(to);
        n_graph_vec[to].push(from);
    }

    fn edge(&self, (from, to): (usize, usize)) -> &Edge {
        self.map.get(&(from, to)).unwrap()
    }

    fn edge_mut(&mut self, (from, to): (usize, usize)) -> &mut Edge {
        self.map.get_mut(&(from, to)).unwrap()
    }

    // 辺(u, v)に流量fのフローを流し残余グラフを計算する
    // (u, v)の容量がfだけ減少する
    // 逆辺(v, u)の容量がf増加する
    // (u, v)に流量を追加する
    fn run_flow(&mut self, (from, to): (usize, usize), f: u32) {
        self.edge_mut((from, to)).cap -= f;
        self.edge_mut((to, from)).cap += f;
    }
}

// 残余グラフ上でs-tパスを見つけ容量の最小値を返す
// f: sからvへ到達した過程の各辺の容量の最小値
fn fodfs(g: &mut Graph, seen: &mut Vec<bool>, v: usize, t: usize, f: u32) -> Option<u32> {
    // 終端tに到達したらリターン
    if v == t {
        return Some(f);
    }

    // 深さ優先探索
    seen[v] = true; // 訪問済みとする

    // let from_v_n_graph = &NGRAPH.get().unwrap().read().unwrap()[v];
    let n_graph = NGRAPH.with(|n_graph| n_graph.clone());
    let from_v_n_graph = &n_graph.borrow()[v];

    for to in from_v_n_graph.iter() {
        let cap = g.edge((v, *to)).cap;

        if seen[*to] {
            continue;
        } // 探索済みの場合は訪問しない

        // 容量0の辺は実際には存在しないため
        if cap == 0 {
            continue;
        }

        // s-tパスを探す
        // 見つかったらflowはパス上の最小容量
        // 見つからなかったらNoneが返り次辺へ
        match fodfs(g, seen, *to, t, cmp::min(f, cap)) {
            None => {
                continue;
            } // 見つからなかった場合
            Some(flow) => {
                g.run_flow((v, *to), flow); // 見つかったら辺eに最小容量のflowを流す
                return Some(flow); // 最小容量を伝播
            }
        }
    }

    // 見つからなかった場合
    None
}

const INF: u32 = 10_u32.pow(5);

// グラフgのs-t間の最大流量を求める
// gは変更され残余グラフになることに注意
fn fordfulkerson(g: &mut Graph, s: usize, t: usize) -> u32 {
    let mut max_flow = 0_u32;

    loop {
        // 残余グラフにs-tパスが無くなるまでループ
        let mut seen = vec![false; g.size()];
        match fodfs(g, &mut seen, s, t, INF) {
            None => {
                break;
            }
            Some(flow) => {
                max_flow += flow; // flowを加算
            }
        }
    }
    max_flow
}

fn main() {
    let mut g = Graph::new(6_usize);
    g.addedge((0, 1), 5);
    g.addedge((0, 3), 5);
    g.addedge((1, 2), 4);
    g.addedge((1, 3), 37);
    g.addedge((2, 4), 56);
    g.addedge((2, 5), 9);
    g.addedge((3, 2), 3);
    g.addedge((3, 4), 9);
    g.addedge((4, 5), 2);

    let max_flow = fordfulkerson(&mut g, 0, 5);
    if max_flow == 0 {
        println!("not exist s-t path");
    } else {
        println!("max flow:{:?}", max_flow);
    }
}
