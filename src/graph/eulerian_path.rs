pub fn eulerian_path(mut graph: Vec<Vec<usize>>, src: usize) -> Vec<usize> {
    fn _impl(graph: &mut Vec<Vec<usize>>, src: usize, ret: &mut Vec<usize>) {
        while let Some(v_next) = graph[src].pop() {
            _impl(graph, v_next, ret);
        }
        ret.push(src);
    }

    let mut ret = vec![];
    _impl(&mut graph, src, &mut ret);
    ret.reverse();

    ret
}
