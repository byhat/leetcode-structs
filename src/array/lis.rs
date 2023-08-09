pub fn longest_increasing_subsequence(vec: Vec<i32>) -> i32 {
    let mut stack = vec![]; // monotonic stack
    for e in vec {
        let e_max = if let Some(inner) = stack.last() {
            *inner
        } else {
            stack.push(e);
            continue;
        };

        if e > e_max {
            stack.push(e);
            continue;
        }

        let ptr = stack.partition_point(|&_e| _e < e);
        stack[ptr] = e;
    }

    stack.len() as i32
}
