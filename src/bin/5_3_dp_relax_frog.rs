fn choose_min<T: std::cmp::PartialOrd>(a:&mut T, b:T){
    if *a > b{
        *a = b;
    }
}


fn main() {
    let n:usize = 7;
    let h:Vec<i32> = vec![2,9,4,5,1,6,10];

    let mut dp:Vec<i32> = vec![10_i32.pow(5);n];
    
    // 初期条件
    dp[0] = 0;

    for i in 1..n {
        if i == 1 {  // 1の時は二つ前が存在しないため
            let one_step_cost = dp[i - 1] + (h[i] - h[i - 1]).abs();
            choose_min(&mut dp[i], one_step_cost);
        } else {
            let one_step_cost = dp[i - 1] + (h[i] - h[i - 1]).abs();
            let two_step_cost = dp[i - 2] + (h[i] - h[i - 2]).abs();

            choose_min(&mut dp[i], one_step_cost);
            choose_min(&mut dp[i], two_step_cost);
        }
    }

    println!("solution: {:?}", dp[n-1]);

}