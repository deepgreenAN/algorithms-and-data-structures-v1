struct Heap<T: PartialOrd> {
    heap_vec: Vec<T>, // ヒープを実装する配列
}

impl<T: PartialOrd> Heap<T> {
    fn push(&mut self, x: T) {
        self.heap_vec.push(x); // 最後尾に要素を追加
        let mut i: usize = self.heap_vec.len() - 1; // 探索するインデックス

        while i > 0 {
            // 上に探索するため
            let p: usize = (i - 1) / 2; // 親のインデックス
            if self.heap_vec[p] >= self.heap_vec[i] {
                // 親とxに逆転が無ければ終了
                break;
            }

            // 親とxが逆転している場合
            self.heap_vec.swap(i, p); // 逆転があるため入れ替える
            i = p; // 自分は上にいく
        }
    }

    fn new(heap_vec: Vec<T>) -> Self {
        // コンストラクタ
        let mut instance = Self {
            heap_vec: Vec::new(),
        };
        for i in heap_vec.into_iter() {
            instance.push(i);
        }
        instance
    }

    fn top(&self) -> Option<&T> {
        // 最大値を取得
        self.heap_vec.first()
    }

    fn pop(&mut self) -> Option<T> {
        // 最大値（根）を削除

        let poped_value = self.heap_vec.swap_remove(0); // ポップで返される値．同時に最後の値を最初に持ってくる(swap_remove)

        let mut i: usize = 0; // 根から降ろしてくる
        while i * 2 + 1 < self.heap_vec.len() {
            // 子がいるかぎり続く
            let mut child1: usize = i * 2 + 1;
            let child2: usize = i * 2 + 2;
            if child2 < self.heap_vec.len() && self.heap_vec[child2] > self.heap_vec[child1] {
                // 子頂点同士を比較して大きい方をchild1とする
                child1 = child2;
            }

            // 枝の調整
            if self.heap_vec[child1] <= self.heap_vec[i] {
                // i番目の要素とその子に逆転が無ければ終了
                break;
            }

            // xと子が逆転している場合
            self.heap_vec.swap(i, child1); // 逆転があるため入れ替える
            i = child1; // 自分は下に行く
        }

        Some(poped_value)
    }
}

fn main() {
    let mut heap = Heap::new(vec![1, 2, 3, 3, 5, 6, 7, 7, 10, 12, 15, 19]);
    println!("heap: {:?}", heap.heap_vec);

    println!("top of heap: {:?}", heap.top());

    heap.push(17);
    println!("added 17");
    println!("added heap: {:?}", heap.heap_vec);

    println!("poped value: {:?}", heap.pop());
    println!("remove root heap: {:?}", heap.heap_vec);
}
