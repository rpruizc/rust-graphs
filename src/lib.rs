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

// Graph maintains the invariant that nodes can only be added, never removed.
// Idx will always be valid as long as it is used with the correct Graph
impl Graph {
    pub fn push(&mut self, children: Vec<Idx>) -> Idx {
        self.nodes.push(Node { children });
        Idx(self.nodes.len() - 1)
    }

    // Returns the number of paths between each leaf node and the final node.
    // The number of paths from leaves to each node is memoized
    //
    // A DAG does not have a true 'final node', but it is used here for simplicity
    pub fn count_paths(&self) -> usize {
        let mut path_counts = Vec::new();

        for node in &self.nodes {
            let paths_to_here = if node.children.is_empty() {
                1
            } else {
                node.children
                    .iter()
                    .map(|child_index| path_counts[child_index.0])
                    .sum()
            };
            path_counts.push(paths_to_here);
        }

        path_counts[path_counts.len() - 1]
    }
}