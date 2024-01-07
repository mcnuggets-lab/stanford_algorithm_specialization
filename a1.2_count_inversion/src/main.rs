fn count_inversion(ar: &Vec<usize>) -> usize {
    sort_and_count(ar).1
}

fn sort_and_count(ar: &Vec<usize>) -> (Vec<usize>, usize) {
    if ar.len() == 1 {
        (ar.to_vec(), 0)
    }
    else {
        let (ar1, count1) = sort_and_count(&ar[..ar.len() / 2].to_vec());
        let (ar2, count2) = sort_and_count(&ar[ar.len() / 2..].to_vec());
        let (ar3, count3) = count_split_inv(&ar1, &ar2);
        (ar3.to_vec(), count1 + count2 + count3)
    }
}

fn count_split_inv(ar1: &Vec<usize>, ar2: &Vec<usize>) -> (Vec<usize>, usize) {
    let mut res: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut count: usize = 0;

    while i < ar1.len() && j < ar2.len() {
        if ar1[i] <= ar2[j] {
            res.push(ar1[i]);
            i += 1;
        }
        else {
            res.push(ar2[j]);
            j += 1;
            count += ar1.len() - i;
        }
    }

    if i == ar1.len() {
        ([&res[..], &ar2[j..]].concat(), count)
    }
    else {
        // case j == ar2.len()
        ([&res[..], &ar1[i..]].concat(), count)
    }
}

fn main() {
    let file_path = "IntegerArray.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();

    let mut ar: Vec<usize> = Vec::new();
    for content in contents.lines() {
        ar.push(str::parse::<usize>(content).unwrap());
    }

    println!("{:?}", count_inversion(&ar));
}
