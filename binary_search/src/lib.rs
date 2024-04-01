#[allow(dead_code)]
fn binary_search(_target: usize, _slice: &[usize]) -> Option<usize> {
    None
}

#[test]
fn even_length_array() {
    let slice = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100] as [usize; 10];

    for (i, &target) in slice.iter().enumerate() {
        let result = binary_search(target, &slice);
        assert!(result.is_some(), "target {} was not found!", target);
        assert_eq!(result.unwrap(), i, "index is incorrect!");
    }

    let target = 0;
    let result = binary_search(target, &slice);
    assert!(result.is_none(), "target was found!");
}

#[test]
fn odd_length_array() {
    let slice = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110] as [usize; 11];

    for (i, &target) in slice.iter().enumerate() {
        let result = binary_search(target, &slice);
        assert!(result.is_some(), "target {} was not found!", target);
        assert_eq!(result.unwrap(), i, "index is incorrect!");
    }

    let target = 0;
    let result = binary_search(target, &slice);
    assert!(result.is_none(), "target was found!");
}

#[test]
fn one_item_array() {
    let slice = [10] as [usize; 1];
    let target = 10;
    let result = binary_search(target, &slice);

    assert!(result.is_some(), "target 0 was not found!");
    assert_eq!(result.unwrap(), target, "index is incorrect!");

    let target = 0;
    let result = binary_search(target, &slice);
    assert!(result.is_none(), "target was found!");
}

#[test]
fn empty_array() {
    let slice = [];
    let target = 0;
    let result = binary_search(target, &slice);
    assert!(result.is_none(), "target was found!");
}
