use itertools::Itertools;
use std::fmt;
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

    fn size(&self, x: usize) -> usize {
        // xの含まれるグループのサイズを返す
        let root_x = self.ref_root(x);
        self.siz[root_x]
    }
}

// 表示について
impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut group_sorted_vertex: Vec<usize> = (0..self.par.len()).collect();
        group_sorted_vertex.sort_by_key(|x| self.ref_root(*x)); // 根毎にソート

        let groups = group_sorted_vertex
            .into_iter()
            .group_by(|x| self.ref_root(*x))
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect::<Vec<Vec<usize>>>();

        write!(f, "UnionFind {:?}", groups)
    }
}

fn main() {
    let mut uf = UnionFind::new(7);

    println!("{:}", uf);

    uf.unite(1, 2);
    println!("unite 1 and 2");
    uf.unite(2, 3);
    println!("unite 2 and 3");
    uf.unite(5, 6);
    println!("unite 5 and 6");

    println!("{:}", uf);

    println!("1 and 3 is same group: {:?}", uf.issame(1, 3));
    println!("2 and 5 is same group: {:?}", uf.issame(2, 5));

    uf.unite(2, 5);
    println!("unite 2 and 5");

    println!("{:}", uf);

    println!("2 and 5 is same group: {:?}", uf.issame(2, 5));
    println!("size of 2: {:?}", uf.size(2));
}
