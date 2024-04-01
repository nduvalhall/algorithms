#[allow(dead_code)]
fn binary_search(target: i32, array: &[i32]) -> Option<i32> {
    println!("Binary search: searching for {}", target);

    let index = 0;
    // do search

    println!("Found {} at index {}", target, index);

    Some(index)
}

#[test]
fn it_works() {
    let target = 12;
    let array = [1, 3, 6, 12, 24, 35];

    let index = binary_search(target, &array);

    assert!(index.is_some())
}
