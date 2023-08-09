// Returns the length of LIS for vector `vec`.
// Implementation notice:
//   all elements in `vec` must be strictly larger than i32::MIN.
pub fn longest_inc_subseq(vec: Vec<i32>) -> usize {
    let mut stack = vec![]; // monotonic stack
    for e in vec {
        let e_max = stack.last().cloned().unwrap_or(i32::MIN);
        if e > e_max {
            stack.push(e);
            continue;
        }

        let ptr = stack.partition_point(|&_e| _e < e);
        stack[ptr] = e;
    }

    stack.len()
}

// Returns the length of LNdS for vector `vec`.
// Implementation notice:
//   all elements in `vec` must be strictly larger than i32::MIN.
pub fn longest_nondec_subseq(vec: Vec<i32>) -> usize {
    let mut stack = vec![]; // monotonic stack
    for e in vec {
        let e_max = stack.last().cloned().unwrap_or(i32::MIN);
        if e >= e_max {
            stack.push(e);
            continue;
        }

        let ptr = stack.partition_point(|&_e| _e <= e);
        stack[ptr] = e;
    }

    stack.len()
}
