use crate::graph::Graph;

pub mod grasp;
pub mod grasp_pr;

fn get_coloring_from_class_list(num_vertices: usize, class_list: &[Vec<usize>]) -> Vec<usize> {
    let mut coloring: Vec<usize> = vec![0; num_vertices];
    for (i, class) in class_list.iter().enumerate() {
        for vertex in class {
            assert_eq!(coloring[*vertex], 0);
            coloring[*vertex] = i + 1;
        }
    }
    coloring
}

/// Counts the number of forbidden edges from `vertex` in `graph` according to `coloring`.
fn count_forbidden_per_vertex(graph: &Graph, coloring: &[usize], vertex: usize) -> usize {
    let num_vertices = graph.num_vertices();
    let adjacency_matrix = graph.adjacency_matrix();
    let mut count = 0;
    for i in 0..num_vertices {
        if adjacency_matrix[vertex][i] && coloring[i] == coloring[vertex] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
fn check_viability(graph: &Graph, coloring: &[usize]) {
    let num_vertices = graph.num_vertices();
    let adjacency_matrix = graph.adjacency_matrix();

    for (i, vertex) in coloring.iter().enumerate() {
        // Each vertex must get a color
        assert_ne!(*vertex, 0);

        (0..num_vertices).for_each(|j| {
            if adjacency_matrix[i][j] {
                // Neighbors can't share colors
                assert_ne!(coloring[j], *vertex);
            }
        });
    }
}
