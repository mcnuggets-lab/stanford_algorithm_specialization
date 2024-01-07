fn dp_bst(probs: Vec<f64>) -> f64 {
    let mut dp_array = vec![vec![0.0; probs.len()]; probs.len()];

    // initialize
    for i in 0..probs.len() {
        dp_array[i][i] = probs[i];
    }

    for s in 1..probs.len() {
        for i in 0..probs.len()-s {
            let mut min_value = f64::MAX;
            for r in i..=i+s {
                let size: usize = probs.len() - 1;
                let cur_value = match r {
                    0 => dp_array[1][i+s],
                    r if r == size => dp_array[i][probs.len() - 2],
                    r => dp_array[i][r-1] + dp_array[r+1][i+s],
                };
                if cur_value < min_value {
                    min_value = cur_value;
                }
            }
            dp_array[i][i+s] = (&probs[i..=i+s]).iter().sum::<f64>() + min_value;
        }
    }

    dp_array[0][probs.len() - 1]
}

fn main() {
    let probs: Vec<f64> = vec![0.05, 0.4, 0.08, 0.04, 0.1, 0.1, 0.23];
    let probs2: Vec<f64> = vec![0.2, 0.05, 0.17, 0.1, 0.2, 0.03, 0.25];

    println!("{:?}", dp_bst(probs));
    println!("{:?}", dp_bst(probs2));

}
