fn prob(v: &Vec<i64>) -> i64 {
    return v.iter().sum::<i64>() / i64::try_from(v.len()).unwrap();
}

fn main() {
    let vec = vec![1, 2, 3, 5];

    println!("{}", prob(&vec));
}

