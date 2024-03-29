use super::{count_colors, grasp::grasp, is_coloring_valid, Solution};
use crate::graph::adj_list::AdjList;

pub fn grasp_path_relinking(graph: &AdjList, num_solutions_grasp: usize) -> Solution {
    let mut solutions = grasp(graph, 25, 25, 3, num_solutions_grasp).into_sorted_vec();
    solutions.reverse();
    let mut best_solution = solutions.pop().unwrap().clone();

    while let Some(solution) = solutions.pop() {
        // Always follow the current best coloring, instead of using the starting one
        let original_best_coloring = best_solution.1.clone();
        let mut difference = simmetric_difference(&original_best_coloring, &solution.1);
        let mut new_coloring = solution.1.clone();

        while let Some(vertex) = difference.pop() {
            new_coloring[vertex] = original_best_coloring[vertex];

            let num_colors = count_colors(&new_coloring);

            // Avoid having to check if the coloring is valid (since it's more expensive)
            // if the number of colors hasn't improved
            if num_colors < best_solution.0 {
                continue;
            }

            if !is_coloring_valid(graph, &new_coloring) {
                continue;
            }

            best_solution.0 = num_colors;
            best_solution.1 = new_coloring.clone();
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
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel5.col") {
            let (_, coloring) = grasp_path_relinking(&graph, 5);

            assert!(is_coloring_valid(&graph, &coloring));
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}
