///  i番目の頂点を根とする部分木(大きい方の枝のみ)について，ヒープ条件を満たすようにする
///  aのうち0番目からN-1番目までの部分a[0:N]についてのみ考える．
fn heapify<T: PartialOrd>(a: &mut [T], i: usize, n: usize) {
    let mut child1 = i * 2 + 1; // 左の子供
    if child1 >= n {
        return;
    } // ベースケース．子供が以内ときは終了

    // 子供同士を比較して大きい方をchild1とする
    if child1 + 1 < n && a[child1 + 1] > a[child1] {
        child1 += 1;
    }

    // 子供と逆転が無かったら終了
    if a[child1] <= a[i] {
        return;
    }

    // 大きい方の子供とi番目を入れ替える
    a.swap(i, child1);

    // 再帰
    heapify(a, child1, n);
}

// in-placeなヒープソート
fn heap_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();

    // a全体をヒープにする
    for i in (0..(n / 2 - 1)).rev() {
        // 最後-1の深さまで下から
        heapify(a, i, n);
    }

    // ヒープから一つ一つ最大値をpopする
    for i in (0..n).rev() {
        // 後ろから
        a.swap(0, i); // ヒープの最大値を右詰め
        heapify(a, 0, i); // 0からiまでヒープ化
    }
}

fn main() {
    let mut a = vec![10, 12, 15, 3, 8, 17, 4, 1];

    println!("a:{:?}", a);
    heap_sort(&mut a);
    println!("sorted a:{:?}", a);
}
