// a[left..right]をin-placeでクイックソート
fn quick_sort<T: PartialOrd>(a: &mut [T], left: usize, right: usize) {
    if right - left <= 1 {
        // ペースケース
        return; // 要素が一つしかないとき
    }

    // ピボットの初期化
    let pivot_index = (left + right) / 2; // 最初は中心とする

    a.swap(pivot_index, right - 1); // ピポットと右端をスワップ

    let mut i = left; // pivot未満の最大インデックス
    for j in left..right - 1 {
        // 右端を除く
        if a[j] < a[right - 1] {
            // ピボット未満の場合左詰め
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, right - 1); // ピボットを右端から適切な位置へ

    // 再帰的に解く
    quick_sort(a, left, i); // 左半分
    quick_sort(a, i + 1, right); // 右半分
}

fn main() {
    let mut a = vec![10, 12, 15, 3, 8, 17, 4, 1];
    let n = a.len();

    println!("a:{:?}", a);
    quick_sort(&mut a, 0, n);
    println!("sorted a:{:?}", a);
}
