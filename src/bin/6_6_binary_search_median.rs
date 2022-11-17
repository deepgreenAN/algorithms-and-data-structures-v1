/// 擬似的な値であることに注意
fn main() {
    let a: Vec<u32> = vec![1, 3, 4, 7, 9, 10, 13, 15, 17];
    let n: usize = a.len();

    let max = a.iter().cloned().max().unwrap(); // Nで求められる

    let mut left = 0;
    let mut right = max;

    while (right - left) > 1 {
        let mid = (left + right) / 2; // 範囲の中心の高度

        let mut counter = 0_usize;

        for item in a.iter() {
            if *item < mid {
                counter += 1;
            }
        }

        if counter >= (n - 1) / 2 {
            right = mid; // 区間の中心を右端に
        } else {
            left = mid; // 区間の中心を左端に
        }
    }

    println!("median: {:?}", right);
}
