use std::mem;

// 外部配列を用いるためtakeで所有権を奪うのでDefalutトレイトが必要
// a[left..right]をin-placeでマージソート
fn merge_sort<T: PartialOrd + Default>(a: &mut [T], left: usize, right: usize) {
    if right - left == 1 {
        // ベースケース
        return; // 要素が一つの時は何もしない
    }

    let mid = left + (right - left) / 2;

    // 左半分　[left, mid) をソート
    merge_sort(a, left, mid);

    // 右半分　[mid, right) をソート
    merge_sort(a, mid, right);

    let mut buf: Vec<T> = Vec::with_capacity(right - left + 1); // 要素を入れるバッファ
    for a_value in a.iter_mut().take(mid).skip(left) {
        buf.push(mem::take(a_value));
    } // 左半分
    for a_value in a.iter_mut().take(right).skip(mid).rev() {
        buf.push(mem::take(a_value));
    } //  右半分は反転させる

    // 初期インデックスをソートする
    let mut index_left: usize = 0; // バッファの左端から
    let mut index_right: usize = buf.len() - 1; // バッファの右端から

    for a_value in a.iter_mut().take(right).skip(left) {
        // バッファ分のインデックスをソートしてaに代入

        if buf[index_left] <= buf[index_right] {
            // 左側採用
            *a_value = mem::take(&mut buf[index_left]);
            index_left += 1;
        } else {
            // 右側採用
            *a_value = mem::take(&mut buf[index_right]);
            index_right -= 1;
        }
    }
}

fn main() {
    let mut a = vec![3, 9, 12, 15, 1, 6, 8, 7, 3, 16, 22, 32];
    let n = a.len();

    println!("a:{:?}", a);
    merge_sort(&mut a, 0, n);
    println!("sorted a:{:?}", a);
}
