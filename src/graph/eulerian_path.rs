fn eulerian_path_inner(graph: &mut Vec<Vec<usize>>, src: usize, ret: &mut Vec<usize>) {
    while let Some(v_next) = graph[src].pop() {
        eulerian_path_inner(graph, v_next, ret);
    }
    ret.push(src);
}

pub fn eulerian_path(mut graph: Vec<Vec<usize>>, src: usize) -> Vec<usize> {
    let mut ret = vec![];
    eulerian_path_inner(&mut graph, src, &mut ret);
    ret.reverse();

    ret
}
