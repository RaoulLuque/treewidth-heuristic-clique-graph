use std::{collections::HashSet, hash::BuildHasher};

use petgraph::graph::NodeIndex;

pub fn neutral_heuristic(_: &HashSet<NodeIndex>, _: &HashSet<NodeIndex>) -> i32 {
    0
}

// Classic
pub fn negative_intersection_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    -(first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32)
}

pub fn positive_intersection_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

pub fn disjoint_union_heuristic<S: BuildHasher>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    (first_vertex.len() + second_vertex.len()) as i32
}

pub fn union_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .union(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

// Classic alt?
pub fn least_difference_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .symmetric_difference(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}
