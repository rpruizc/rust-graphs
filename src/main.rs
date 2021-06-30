use graphs::*;

fn main() {
    let mut g = Graph::default();
    let a = g.push(vec![]);
    let b = g.push(vec![a]);
    let c = g.push(vec![a, b]);
    let d = g.push(vec![a, b, c]);

    assert_eq!(4, g.count_paths());
}