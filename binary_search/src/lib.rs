/// Finds the index of a target in an array via binary search
#[allow(dead_code)]
fn binary_search(target: i32, slice: &[i32]) -> Option<usize> {
    let mut bottom: i8 = 0;
    let mut top: i8 = slice.len() as i8 - 1;

    while bottom <= top {
        let mid = ((top - bottom) / 2) + bottom;
        let current = slice[mid as usize];

        if current == target {
            return Some(mid as usize);
        }

        if current > target {
            top = mid - 1;
            continue;
        }

        if current < target {
            bottom = mid + 1;
            continue;
        }
    }

    None
}

/// Finds the index of a target in an array via linear search
#[allow(dead_code)]
fn linear_search(target: i32, slice: &[i32]) -> Option<usize> {
    for (index, &item) in slice.iter().enumerate() {
        if target == item {
            return Some(index);
        }
    }
    None
}

#[test]
fn even_length_array() {
    let slice = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    for target in slice {
        let lin = linear_search(target, &slice);
        let bin = binary_search(target, &slice);
        assert_eq!(bin, lin);
    }

    let target = 0;
    let lin = linear_search(target, &slice);
    let bin = binary_search(target, &slice);
    assert_eq!(bin, lin);
}

#[test]
fn odd_length_array() {
    let slice = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110];

    for target in slice {
        let lin = linear_search(target, &slice);
        let bin = binary_search(target, &slice);
        assert_eq!(bin, lin);
    }

    let target = 0;
    let lin = linear_search(target, &slice);
    let bin = binary_search(target, &slice);
    assert_eq!(bin, lin);
}

#[test]
fn one_item_array() {
    let slice = [10];

    let target = 10;
    let lin = linear_search(target, &slice);
    let bin = binary_search(target, &slice);
    assert_eq!(bin, lin);

    let target = 0;
    let lin = linear_search(target, &slice);
    let bin = binary_search(target, &slice);
    assert_eq!(bin, lin);
}

#[test]
fn empty_array() {
    let slice = [];
    let target = 0;

    let lin = linear_search(target, &slice);
    let bin = binary_search(target, &slice);
    assert_eq!(bin, lin);
}
