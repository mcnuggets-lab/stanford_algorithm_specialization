use std::collections::HashMap;

fn knapsack(knapsack_size: usize, values: &Vec<usize>, weights: &Vec<usize>) -> usize {
    fn knapsack_recursive(cache: &mut HashMap<(usize, usize), usize>, num_items: usize, knapsack_size: usize, values: &Vec<usize>, weights: &Vec<usize>) -> usize {
        if num_items == 0 {
            return 0;
        }
        match cache.get(&(num_items, knapsack_size)) {
            Some(result) => *result,
            None => {
                let res = if weights[num_items - 1] > knapsack_size {
                    knapsack_recursive(cache, num_items - 1, knapsack_size, values, weights)
                }
                else {
                        std::cmp::max(
                        knapsack_recursive(cache, num_items - 1, knapsack_size, values, weights),
                        knapsack_recursive(
                            cache, num_items - 1, knapsack_size - weights[num_items - 1], values, weights
                        ) + values[num_items - 1],
                    )
                };
                cache.insert((num_items, knapsack_size), res);
                res
            }
        }
    }

    knapsack_recursive(&mut HashMap::new(), weights.len(), knapsack_size, values, weights)
}

fn main() {

    let file_path = "knapsack_big.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut lines_iter = contents.lines();
    let line: Vec<&str> = lines_iter.next().unwrap().split(" ").collect();
    let knapsack_size: usize = str::parse(line[0]).unwrap();
    let _num_items: usize = str::parse(line[1]).unwrap();

    let mut values: Vec<usize> = Vec::new();
    let mut weights: Vec<usize> = Vec::new();

    for content in lines_iter {
        let line: Vec<&str> = content.split(" ").collect();
        let value = str::parse(line[0]).unwrap();
        let weight = str::parse(line[1]).unwrap();
        values.push(value);
        weights.push(weight);
    }

    println!("{:?}", knapsack(knapsack_size, &values, &weights));  // 2493893 for knapsack1, 4243395 for knapsack_big

}
