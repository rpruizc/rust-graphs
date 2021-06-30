// The algorithm traverses the graph and memoizes intermediate values
// Memoization is preferred as paths grow exponentially as more nodes
// are added, recursive implementations can be slow.

#[derive(Copy, Clone)]
pub struct Idx(usize);

pub struct Node {
    children: Vec<Idx>,
}

#[derive(Default)]
pub struct Graph {
    nodes: Vec<Node>,
}