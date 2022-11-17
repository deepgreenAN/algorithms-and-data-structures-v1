fn main() {
    let n = 5;
    let w = 10;
    let a: Vec<i32> = [1,2,4,5,11].to_vec();

    let mut solution_bit: Option<u32> = None;  // 解

    for bit in 0..(1<<n) as u32 {
        let mut sum = 0;
        for (i, value) in a.iter().enumerate() {
            if 0 != (bit & (1 << i)) {  // i番目の要素a[i]が部分集合に含まれてるかどうか
                sum += *value;  // a[i]を足す
            }
        }

        if sum == w {  // sumがwに一致するかどうか
            solution_bit = Some(bit);
        }
    }

    match solution_bit {  // 解の表示
        None => { println!("solution is nothing.");},
        Some(solution_bit) => {
            let solution_bit_str = format!("{:0width$b}", solution_bit, width=n);  //　幅をn桁にしてビット表示
            println!("solution_bit:{}", solution_bit_str);

            let mut solution_bit_vec = solution_bit_str.chars().collect::<Vec<_>>();  // 解のビット表示をベクトル化
            solution_bit_vec.reverse();  // 桁の読み方が文字列ては逆なため
            let solution_a: Vec<i32> = a.into_iter().zip(solution_bit_vec).filter(
                |(_, bit_str)|{*bit_str=='1'}  // charとして比較
            ).map(|(x,_)|{x}).collect();  // solution_bit_vecが'1'であるaつまり解のリストを取得
            println!("solution_a:{:?}", solution_a);
        }
    }
    
}