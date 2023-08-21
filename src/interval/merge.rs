pub fn merge_intervals(mut interval_vec: Vec<[usize; 2]>) -> Vec<[usize; 2]> {
    interval_vec.sort_unstable();
    interval_vec.dedup();

    let mut stack = vec![];

    for [t0, t1] in interval_vec {
        if stack.is_empty() {
            stack.push([t0, t1]);

            continue;
        }

        let [t0_prev, t1_prev] = stack.pop().unwrap();
        if t1_prev < t0 {
            stack.push([t0_prev, t1_prev]);
            stack.push([t0, t1]);

            continue;
        }

        stack.push([t0_prev, t1_prev.max(t1)]);
    }

    stack
}
