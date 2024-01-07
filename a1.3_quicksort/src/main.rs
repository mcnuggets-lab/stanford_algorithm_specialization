fn swap(ar: &mut [usize], i: usize, j: usize) {
    (ar[i], ar[j]) = (ar[j], ar[i]);
}

fn pivot_strategy_first(_ar: &[usize]) -> usize {
    0
}

fn pivot_strategy_last(ar: &[usize]) -> usize {
    ar.len() - 1
}

fn pivot_strategy_median_of_three(ar: &[usize]) -> usize {
    let median = (ar.len() - 1) / 2;
    let first = 0;
    let last = ar.len() - 1;

    if ar[first] < ar[median] {
        if ar[median] < ar[last] {
            median
        }
        else if ar[first] > ar[last] {
            first
        }
        else {
            last
        }
    }
    else if ar[median] > ar[last] {
        median
    }
    else if ar[first] > ar[last] {
        last
    }
    else {
        first
    }
}

/// partition around the pivot, and return the index position of the pivot after the partition
fn partition_around_pivot(ar: &mut [usize], pivot_index: usize) -> usize {
    swap(ar, pivot_index, 0);
    let mut i = 1;
    for j in 1..ar.len() {
        if ar[j] <= ar[0] {
            swap(ar, i, j);
            i += 1;
        }
    }
    swap(ar, 0, i - 1);

    i - 1
}

fn qsort_count (ar: &mut [usize], pivot_strategy: fn(&[usize]) -> usize) -> usize {
    if ar.len() <= 1 {
        0
    }
    else {
        let pivot_index = pivot_strategy(ar);
        let new_pivot_index = partition_around_pivot(ar, pivot_index);
        let q1 = qsort_count(&mut ar[..new_pivot_index], pivot_strategy);
        let q2 = qsort_count(&mut ar[new_pivot_index + 1 ..], pivot_strategy);

        q1 + q2 + ar.len() - 1
    }
}

fn main() {
    let file_path = "QuickSort.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut ar: Vec<usize> = Vec::new();
    for content in contents.lines() {
        ar.push(str::parse::<usize>(content).unwrap());
    }

    let mut ar1 = ar.clone();
    println!("{:?}", qsort_count(&mut ar1[..], pivot_strategy_first));

    let mut ar2 = ar.clone();
    println!("{:?}", qsort_count(&mut ar2[..], pivot_strategy_last));

    let mut ar3 = ar.clone();
    println!("{:?}", qsort_count(&mut ar3[..], pivot_strategy_median_of_three));
    
}
