use std::cmp::Ordering::{Equal, Greater, Less};

fn binary_search(key: i32, a: &Vec<i32>) -> Option<usize> {
    let mut left = 0_usize;
    let mut right = a.len();

    while right > left {
        let mid = left + (right - left) / 2; // 区間の真ん中(切り捨てられる)

        match a.get(mid)?.cmp(&key) {
            Equal => {
                return Some(mid);
            }
            Greater => {
                right = mid;
            }
            Less => {
                left = mid + 1; // どちらもmidだと無限ループになる
            }
        }
    }
    None
}

fn main() {
    let a = vec![3, 5, 8, 10, 14, 17, 21, 39];

    println!("key=10:index={:?}", binary_search(10, &a));
    println!("key=3:index={:?}", binary_search(3, &a));
    println!("key=39:index={:?}", binary_search(39, &a));

    println!("key=-100:index={:?}", binary_search(-100, &a));
    println!("key=9:index={:?}", binary_search(9, &a));
    println!("key=100:index={:?}", binary_search(100, &a));
}
