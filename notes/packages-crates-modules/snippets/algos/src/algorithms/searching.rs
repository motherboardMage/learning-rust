use std::cmp::Ordering;

pub fn linear_search(list: &[i32], target: i32) -> Option<usize> {
    for i in 0..list.len() {
        if list[i] == target {
            return Some(i);
        }
    }
    None
}

pub fn binary_search(list: &[i32], target: i32) -> Option<usize> {
    let mut start: usize = 0;
    let mut end: usize = list.len() - 1;

    while start < end {
        let mid: usize = start + (end - start) / 2;

        match list[mid].cmp(&target) {
            Ordering::Less => start = mid + 1,
            Ordering::Greater => end = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }
    None
}
