fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let length = left.len() + right.len();
    let mut merged_vec = Vec::with_capacity(length);

    let mut left_idx: usize = 0;
    let mut right_idx: usize = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            merged_vec.push(left[left_idx]);
            left_idx += 1;
        } else {
            merged_vec.push(right[right_idx]);
            right_idx += 1;
        }
    }
    merged_vec.extend_from_slice(&left[left_idx..]);
    merged_vec.extend_from_slice(&right[right_idx..]);

    merged_vec
}

pub fn merge_sort(list: &[i32]) -> Vec<i32> {
    if list.len() <= 1 {
        return list.to_vec();
    }
    let mid = list.len() / 2;
    let (left, right) = list.split_at(mid);

    merge(&merge_sort(left), &merge_sort(right))
}
