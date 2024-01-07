use std::collections::BinaryHeap;
use std::cmp::{max, min, Reverse};

fn running_median(inputs: Vec<usize>) -> Vec<usize> {
    let mut heap_low: BinaryHeap<usize> = BinaryHeap::new();
    let mut heap_high: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    let mut res: Vec<usize> = Vec::new();

    // initial case
    let mut count: usize = 1;
    heap_low.push(inputs[0]);
    res.push(inputs[0]);

    // main loop
    for i in &inputs[1..] {
        count += 1;

        let temp = if count % 2 == 1 { heap_high.pop().unwrap().0 } else { heap_low.pop().unwrap() };
        heap_high.push(Reverse(max(*i, temp)));
        heap_low.push(min(*i, temp));

        res.push(*heap_low.peek().unwrap());
    }

    res
}

fn main() {
    let file_path = "Median.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut inputs: Vec<usize> = Vec::new();
    for content in contents.lines() {
        inputs.push(str::parse(content).unwrap());
    }

    let res = running_median(inputs);
    let res_sum: usize = res.iter().sum();
    println!("{:?}", res_sum % 10000);
    
}
