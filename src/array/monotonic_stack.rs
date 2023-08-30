// For each index `i`, returns the largest `j`
//   where vec[i..j].any(|e| e >= vec[i]).
//
// Postfix order is easier to understand and implement.
pub fn to_geq_post_vec(vec: &Vec<u32>) -> Vec<usize> {
    let n = vec.len();
    let mut ret = Vec::with_capacity(n);

    let mut stack = vec![(u32::MIN, n)]; // barrier at i=n
    for (i, &e) in vec.iter().enumerate().rev() {
        let ptr = stack.partition_point(|&(e0, _)| e0 < e); // ptr > 0 (why??)
        ret.push(stack[ptr - 1].1);

        stack.truncate(ptr);
        stack.push((e, i));
    }
    ret.reverse();

    ret
}
