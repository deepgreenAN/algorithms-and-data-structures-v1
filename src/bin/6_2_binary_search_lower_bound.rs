// P(x) = trueとなる最小の配列aのインデックスを返す
fn lower_bound<F, T>(mut p: F, a: &[T]) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    let mut left: i32 = 0;
    let mut right: i32 = a.len() as i32 - 1;

    // 左端が条件を満たす場合
    if p(a.first()?) {
        return Some(left as usize);
    }

    // 右端が条件を満たさない場合
    if !p(a.last()?) {
        return None;
    }

    while (right - left) > 1 {
        let mid = left + (right - left) / 2; // 切り捨てられる

        if p(&a[mid as usize]) {
            // 区間の中心が条件を満たす場合
            right = mid; // 区間の中心を右端とする
        } else {
            left = mid; // 区間の中心を左端とする
        }
    }

    Some(right as usize) // lowwer boundを返す
}

fn main() {
    let a = vec![3, 5, 8, 10, 14, 17, 21, 39];

    println!(
        "lower bound index(bigger than 9): {:?}",
        lower_bound(|x| { *x >= 9 }, &a)
    );
    println!(
        "lower bound index(bigger than 10): {:?}",
        lower_bound(|x| { *x >= 10 }, &a)
    );
    println!(
        "lower bound index(bigger than 0): {:?}",
        lower_bound(|x| { *x >= 0 }, &a)
    );
    println!(
        "lower bound index(bigger than 50): {:?}",
        lower_bound(|x| { *x >= 50 }, &a)
    );
}
