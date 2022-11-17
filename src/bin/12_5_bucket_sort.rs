const MAX: usize = 10_usize.pow(5);

fn bucket_sort(a: &[u32]) -> Vec<u32> {
    let n = a.len();

    // 各要素の個数をカウントする配列
    let mut num = vec![0_u32; MAX];
    for i in 0..n {
        num[a[i] as usize] += 1; // a[i]をカウント
    }

    // numの累積和をとる（a[i]が全体の中で何番目かを計算）
    let mut sum = vec![0_u32; MAX];
    for i in 1..MAX {
        sum[i] = sum[i - 1] + num[i]
    }

    // 累積和をもとにソートした新しい配列を作成
    let mut a2 = vec![0_u32; n];
    for i in 0..n {
        a2[(sum[a[i] as usize] - 1) as usize] = a[i];
    }

    a2
}

fn main() {
    let a = vec![10, 12, 15, 3, 8, 17, 4, 1];

    println!("a:{:?}", a);
    let a2 = bucket_sort(&a);
    println!("sorted a:{:?}", a2);
}
