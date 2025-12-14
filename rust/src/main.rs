fn prob(v: &[i64]) -> i64 {
    let sum: i64 = v.iter().sum();
    sum / v.len() as i64
}

fn main() {
    let vec = vec![1, 2, 3, 5];

    println!("{}", prob(&vec));
}

