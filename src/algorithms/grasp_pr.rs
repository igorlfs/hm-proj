use super::{count_colors, grasp::grasp, is_coloring_valid, Solution};
use crate::graph::Graph;

pub fn grasp_path_relinking(graph: &Graph, num_solutions_grasp: usize) -> Solution {
    let mut solutions = grasp(graph, 10, 5, 5, num_solutions_grasp);
    let mut best_solution = solutions.pop().unwrap().clone();

    while let Some(solution) = solutions.pop() {
        let original_best_coloring = best_solution.1.clone();
        let mut difference = simmetric_difference(&best_solution.1, &solution.1);
        let mut new_coloring = solution.1.clone();

        while let Some(vertex) = difference.pop() {
            new_coloring[vertex] = original_best_coloring[vertex];

            if !is_coloring_valid(graph, &new_coloring) {
                continue;
            }

            let num_colors = count_colors(&new_coloring);
            if num_colors < best_solution.0 {
                best_solution.0 = num_colors;
                best_solution.1 = new_coloring.clone();
            }
        }

        // At the end we should have turned `new_coloring` into the `original_best_coloring`
        assert_eq!(new_coloring, original_best_coloring);
    }

    best_solution
}

/// Calculates the indexes where `lhs` and `rhs` differ, given that they have the same length.
/// Else, create a new vector.
fn simmetric_difference(lhs: &[usize], rhs: &[usize]) -> Vec<usize> {
    if lhs.len() != rhs.len() {
        Vec::new()
    } else {
        let mut difference = Vec::new();

        for i in 0..lhs.len() {
            if lhs[i] != rhs[i] {
                difference.push(i);
            }
        }

        difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algorithms::is_coloring_valid, input};

    #[test]
    fn test_simmetric_difference() {
        let lhs = vec![1, 2, 3, 4];
        let rhs = vec![1, 3, 2, 4];

        assert_eq!(simmetric_difference(&lhs, &rhs), vec![1, 2]);
    }

    #[test]
    fn test_grasp_path_relinking() {
        // Asserts GRASP + PR provides a solution
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel6.col") {
            let (_, coloring) = grasp_path_relinking(&graph, 5);

            assert!(is_coloring_valid(&graph, &coloring));
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}
