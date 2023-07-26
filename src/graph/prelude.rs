// Given a vector of edges `[v_src, v_dest, weight]`,
//   returns an array of next vertices and weights for each vertex.
pub fn edges_to_graph(edges: Vec<Vec<i32>>, n: usize) -> [Vec<Vec<(usize, u32)>>; 2] {
    let mut ret_0 = vec![vec![]; n];
    let mut ret_1 = vec![vec![]; n];
    for v in edges {
        assert!(v.len() >= 3, "malformed input");
        ret_0[v[0] as usize].push((v[1] as usize, v[2] as u32));
        ret_1[v[1] as usize].push((v[0] as usize, v[2] as u32));
    }
    [ret_0, ret_1]
}
