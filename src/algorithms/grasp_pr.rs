use super::{count_forbidden_per_vertex, get_coloring_from_class_list, grasp::grasp};
use crate::graph::Graph;
use std::collections::HashSet;

type Solution = (usize, Vec<Vec<usize>>);

pub fn grasp_path_relinking(num_iterarions_grasp: usize, graph: &Graph) -> Solution {
    let num_vertices = graph.num_vertices();
    let mut solutions_grasp: Vec<Solution> = Vec::new();
    let mut best_solution: Solution = (usize::MAX, Vec::new());
    let mut best_coloring = Vec::new();

    for _ in 0..num_iterarions_grasp {
        let new_solution = grasp(graph, 10, 5, 5);

        if new_solution.0 < best_solution.0 {
            best_solution = new_solution.clone();
            best_coloring = get_coloring_from_class_list(num_vertices, &new_solution.1);
        }

        solutions_grasp.push(new_solution);
    }

    while let Some(mut solution) = solutions_grasp.pop() {
        let coloring = get_coloring_from_class_list(num_vertices, &solution.1);
        let mut difference = simmetric_difference(&best_coloring, &coloring);

        while let Some(vertex) = difference.pop() {
            let original_color = coloring[vertex];
            let mut new_coloring = coloring.clone();

            new_coloring[vertex] = best_coloring[vertex];

            if count_forbidden_per_vertex(graph, &new_coloring, vertex) > 0 {
                continue;
            }

            let colors: HashSet<usize> = new_coloring.into_iter().collect();

            if colors.len() < best_solution.0 {
                let original_index_in_class_list = solution.1[original_color - 1]
                    .iter()
                    .position(|x| *x == vertex)
                    .unwrap();
                solution.1[original_color - 1].remove(original_index_in_class_list);
                solution.1[best_coloring[vertex] - 1].push(vertex);

                solution.0 = colors.len();

                best_solution = solution.clone();
            }
        }
    }

    best_solution
}

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
    use crate::{algorithms::check_viability, input};

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
            let num_vertices = graph.num_vertices();
            let (_, class_colors) = grasp_path_relinking(5, &graph);

            let coloring = get_coloring_from_class_list(num_vertices, &class_colors);

            check_viability(&graph, &coloring);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}
