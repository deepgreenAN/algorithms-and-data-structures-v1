use std::mem;

struct UnionFind {
    par: Vec<Option<usize>>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: vec![None; n],
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        // 頂点xの根を求める
        match self.par[x] {
            None => x, // ベースケース
            Some(parent) => {
                let root_x = self.root(parent); // 再帰
                self.par[x] = Some(root_x); // 経路圧縮
                root_x
            }
        }
    }

    fn ref_root(&self, x: usize) -> usize {
        // 頂点の根を求める（変更しない）
        match self.par[x] {
            None => x, // ベースケース
            Some(parent) => {
                self.ref_root(parent) // 再帰
            }
        }
    }

    fn issame(&self, x: usize, y: usize) -> bool {
        // 頂点xと頂点yの根が同じであるかどうか
        self.ref_root(x) == self.ref_root(y)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        // 頂点xの含まれるグループと頂点yの含まれるグループを併合する
        let mut root_x = self.root(x);
        let mut root_y = self.root(y);

        // すでに同じグループの場合は何もしないでtrueを返す
        if root_x == root_y {
            return false;
        }

        // 以下union by size(小さいグループを大きなグループに併合する)
        // swapしてroot_yの方が小さくなるようにする
        if self.siz[root_x] < self.siz[root_y] {
            mem::swap(&mut root_x, &mut root_y);
        }

        // yをxの子供とする
        self.par[root_y] = Some(root_x); // rootのみ変更．yの根をxの根に
        self.siz[root_x] += self.siz[root_y]; // rootのみ変更．xの根のサイズにyの根のサイズを加算

        true
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    // 今までと異なるため注意
    u: usize,
    v: usize,
    w: usize,
}

fn kruskal(edges: &mut Vec<Edge>) -> Vec<&Edge> {
    // 辺の重みが小さい順にソート
    edges.sort_by_key(|e| e.w);

    let mut used_edges: Vec<&Edge> = Vec::new();

    let mut uf = UnionFind::new(edges.len()); // サイクルの判定に利用

    for edge in edges.iter() {
        let Edge { u, v, w: _ } = edge;

        // 辺(u,v)の追加によってサイクルが形成されるときは追加しない
        if uf.issame(*u, *v) {
            continue;
        }

        // 辺(u,v)を追加する
        used_edges.push(edge);
        uf.unite(*u, *v);
    }

    used_edges
}

fn main() {
    let mut edges = vec![
        Edge { u: 0, v: 3, w: 5 },
        Edge { u: 0, v: 5, w: 6 },
        Edge { u: 0, v: 7, w: 3 },
        Edge { u: 1, v: 3, w: 8 },
        Edge { u: 1, v: 4, w: 4 },
        Edge { u: 1, v: 6, w: 3 },
        Edge { u: 2, v: 4, w: 9 },
        Edge { u: 2, v: 7, w: 5 },
        Edge { u: 2, v: 5, w: 10 },
        Edge { u: 3, v: 7, w: 6 },
        Edge { u: 4, v: 6, w: 2 },
        Edge { u: 6, v: 7, w: 7 },
    ];

    let used_edges = kruskal(&mut edges);
    println!("used edges:{:?}", used_edges);
}
