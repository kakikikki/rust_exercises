use std::collections::HashMap;

fn main() {

    let vector = vec![2, 1, 3, 7, 4, 2, 0, 6, 9, 1, 9, 3, 9, 1, 9, 4, 5];

    println!("Average = {}", avg(&vector));
    println!("Median = {}", median(&vector));
    println!("Mode = {}", mode(&vector));
}

fn avg(arr: &[i32]) -> f64 {

    let mut total = 0;
    let length = arr.len() as f64;

    for i in arr {
        total += i;
    }

    total as f64 / length
}

fn median(vec: &[i32]) -> i32 {

    let mut arr = vec.to_vec();

    arr.sort();

    let length = arr.len();

    if length % 2 == 0 {
        let a = length / 2;
        let b = &a - 1;

        return (&arr[a] + &arr[b]) / 2;
    }
    else {
        let a = length / 2;

        return arr[a];
    }
}

fn mode(arr: &[i32]) -> i32 {

    let mut map = HashMap::new();

    for number in arr {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut key_biggest = 0;
    let mut val_biggest = 0;
    for (key, val) in map {
        if val > val_biggest {
            key_biggest = *key;
            val_biggest = val;
        }
    }

    return key_biggest;
}
