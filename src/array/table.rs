pub fn to_map<T: std::cmp::Ord>(mut vec: Vec<T>) -> Vec<(usize, T)> {
    vec.sort_unstable();
    vec.reverse();

    let mut ret = vec![];

    while let Some(e) = vec.pop() {
        let ptr = vec.partition_point(|e0| e0 > &e);
        let count = vec.len() + 1 - ptr;

        vec.truncate(ptr);

        ret.push((count, e));
    }

    ret
}
