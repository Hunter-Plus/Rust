fn main() {
    let v1 = vec![1, 2, 3, 5];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));

    for val in v1_iter {
        println!("{val}");
    }

    // Consuming adapters : sum
    let v2 = vec![1, 2, 3, 5];
    let v2_iter = v2.iter();

    let total: i32 = v2_iter.sum();

    println!("{total}");

    // cannot call the iterator because it had been moved to sum
    // for val in v2_iter{
    //     println!("{val}");
    // }

    // Iterator adapters : map
    let v3: Vec<i32> = vec![1, 2, 3];

    v3.iter().map(|x| x + 1); // doing nothing
                              // must call .collect()
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();

    assert_eq!(v4, vec![2, 3, 4]);
}
