pub fn partition_point<P: FnOnce(usize) -> bool + Copy>(
    p0: usize,
    p1: usize,
    predicate: P,
) -> usize {
    if p0 >= p1 {
        return p0;
    }

    let p_decision = (p0 + p1) / 2;
    if predicate(p_decision) {
        partition_point(p_decision + 1, p1, predicate)
    } else {
        partition_point(p0, p_decision, predicate)
    }
}
