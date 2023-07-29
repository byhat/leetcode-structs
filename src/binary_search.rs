// Given a partitioned vector of [True, True, ..., False, False],
//   find the index to the partition point.
//
// This function is identical to Vec::partition_point(),
//   but it generates elements in the vector on-demand from `predicate`,
//   making it O(\log n) space-wise.
//
// This is especially useful when searching across a large solution space.
pub fn partition_point<P: FnOnce(usize) -> bool + Copy>(
    p0: usize,
    p1: usize,
    predicate: P,
) -> usize {
    if p0 >= p1 {
        return p0;
    }

    let p_decision = (p0 + p1) / 2;
    let (p0_next, p1_next) = if predicate(p_decision) {
        (p_decision + 1, p1)
    } else {
        (p0, p_decision)
    };

    partition_point(p0_next, p1_next, predicate) // tail recursion
}
