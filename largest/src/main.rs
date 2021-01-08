fn main() {
    let numbers = vec![2, 1, 3, 7];
    println!("{}", largest(&numbers));

    let papiesz = vec!['p', 'e', 'd', 'a', 'u'];
    println!("{}", largest(&papiesz));

    let kurwa = vec!['k', 'u', 'w', 'w', 'a'];
    println!("{}", largest(&kurwa));
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest
}
