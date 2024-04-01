#[allow(dead_code)]
fn binary_search(target: usize, slice: &[usize]) -> Option<usize> {
    if slice.len() == 0 {
        return None;
    }

    let current_slice = slice;
    let mut index = (current_slice.len() - 1) / 2;

    loop {
        if current_slice.len() == 1 {
            if current_slice[0] == target {
                return Some(0);
            } else {
                return None;
            }
        }

        if current_slice[index] == target {
            return Some(index);
        }

        if current_slice[index] > target {
            let current_slice = &current_slice[0..index];
            index = (current_slice.len() - 1) / 2;
        }

        if current_slice[index] < target {
            let current_slice = &current_slice[index..current_slice.len()];
            index = (current_slice.len() - 1) / 2;
        }
    }
}

#[allow(dead_code)]
fn linear_search(target: usize, slice: &[usize]) -> Option<usize> {
    for (index, &item) in slice.iter().enumerate() {
        if target == item {
            return Some(index);
        }
    }
    None
}

#[test]
fn even_length_array() {
    let slice = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100] as [usize; 10];

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
    let slice = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110] as [usize; 11];

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
    let slice = [10] as [usize; 1];

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
