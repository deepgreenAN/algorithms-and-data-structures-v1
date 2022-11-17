fn func(i: usize, w: i32, a: &Vec<i32>, solution: &mut Vec<i32>) -> bool {
    // ベースケース
    if i == 0 {
        return w == 0;
    }

    // a[i-1]を選ばない場合
    if func(i - 1, w, a, solution) {
        return true;
    }

    // a[i-1]を選ぶ場合
    if func(i - 1, w - a[i - 1], a, solution) {
        solution.push(a[i - 1]);
        return true;
    }

    // どちらもfalseの場合はfalse
    false
}

fn main() {
    let n = 5_usize;
    let w = 10;
    let a: Vec<i32> = [1, 2, 4, 5, 11].to_vec();

    let mut solution: Vec<i32> = [].to_vec();

    if func(n, w, &a, &mut solution) {
        println!("solution_a:{:?}", &solution)
    } else {
        println!("solution is nothing.")
    }
}
