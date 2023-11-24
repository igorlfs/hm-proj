use crate::graph::adj_list::AdjList;
use std::collections::HashSet;

pub mod genetic;
pub mod grasp;
pub mod grasp_pr;

type Solution = (usize, Vec<usize>);

/// Checks if the current color assignment of a node and his neighborhood is valid.
fn is_valid_color_assignment(graph: &AdjList, solution: &[usize], node: usize) -> bool {
    !graph.adj_list()[node]
        .iter()
        .any(|x| solution[node] == solution[*x])
}

/// Counts the number of colors used in a GCP solution.
fn count_colors(solution: &[usize]) -> usize {
    let colors: HashSet<&usize> = solution.iter().collect();

    colors.len()
}

/// Checks if `coloring` is valid for `graph`.
fn is_coloring_valid(graph: &AdjList, coloring: &[usize]) -> bool {
    (0..graph.num_vertices()).all(|x| is_valid_color_assignment(graph, coloring, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_colors() {
        assert_eq!(count_colors(&[1]), 1);
        assert_eq!(count_colors(&[3, 1, 6, 6, 1, 5]), 4);
        assert_eq!(count_colors(&[2, 1, 3, 1, 1, 4, 5, 10, 4, 3, 3]), 6);
        assert_eq!(count_colors(&[]), 0);
    }

    #[test]
    fn test_valid_color_assignment() {
        let mut graph = AdjList::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert!(is_valid_color_assignment(&graph, &[1, 2, 3, 1], 2));
        assert!(!is_valid_color_assignment(&graph, &[1, 2, 2, 1], 2));
    }

    #[test]
    fn test_is_coloring_valid() {
        let mut graph = AdjList::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert!(is_coloring_valid(&graph, &[1, 2, 3, 1]));
        assert!(!is_coloring_valid(&graph, &[1, 2, 2, 1]));
    }
}
