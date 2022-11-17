// in-placeな挿入ソート
fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 1..n {
        // i番目の値を挿入する適切な場所までswap(パフォーマンスは多少落ちるがrust的に安全)
        for j in (0..i).rev() {
            if a[j + 1] < a[j] {
                //  逆転したものはswap(vもふくむ)
                a.swap(j + 1, j);
            } else {
                break; // 逆転がなくなったら止める
            }
        }
    }
}

fn main() {
    let mut a = vec![4, 1, 3, 5, 2];
    println!("a:{:?}", a);
    insertion_sort(&mut a);
    println!("sorted a:{:?}", a);
}
