use std::cmp::Ordering::{Equal, Greater, Less};

fn binary_search(key: i32, a: &Vec<i32>) -> Option<usize> {
    let mut left: i32 = 0;
    let mut right: i32 = a.len() as i32 - 1;

    while right >= left {
        //let mid: i32 = (left + (right - left) / 2).round();  // 区間の真ん中
        let mid: i32 = left + (right - left) / 2; // 区間の真ん中(切り捨てられる)
        let mid_index: usize = mid as usize;

        match a[mid_index].cmp(&key) {
            Equal => {
                return Some(mid_index);
            } // たまたま中心にあたったかright==leftになった場合
            Greater => {
                right = mid - 1;
            }
            Less => {
                left = mid + 1;
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
