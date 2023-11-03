use crate::graph::Graph;
use rand::seq::SliceRandom;

/// Given a `graph`, get (at most) `n` indexes of the higher degree vertices in the subgraph induced by
/// `subset`. If `list` is provided, don't use the induced subgraph and, instead, from the vertices
/// in `subset`, get the ones with higher degree in `list`.
fn get_n_largest_degree(
    n: &usize,
    graph: &Graph,
    subset: &[usize],
    list: Option<&[usize]>,
) -> Vec<usize> {
    let list = if let Some(list) = list { list } else { subset };
    let vertex_set: Vec<usize> = (0..graph.num_vertices()).collect();
    let mut degrees: Vec<(usize, usize)> = vertex_set
        .iter()
        .map(|vertex| (*vertex, graph.get_degree_in_list(vertex, list)))
        .collect();

    degrees.retain(|x| subset.contains(&x.0));

    degrees.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));

    degrees.iter().take(*n).map(|(index, _)| *index).collect()
}

/// Count the number of edges in subgraph induced by `graph` and `list`.
fn count_remaining_edges(graph: &Graph, list: &[usize]) -> usize {
    let mut count = 0;
    let matrix = graph.adjacency_matrix();

    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if matrix[list[i]][list[j]] {
                count += 1;
            }
        }
    }
    count
}

pub fn grasp(
    graph: Graph,
    grasp_iterations: i32,
    color_iterations: i32,
    color_list_size: usize,
) -> (usize, Vec<Vec<usize>>) {
    let max_colors = graph.num_vertices();
    let mut num_colors = max_colors;
    let mut best_class_list: Vec<Vec<usize>> = Vec::new();

    for _ in 0..grasp_iterations {
        let mut num_color_classes = 0;
        let mut vertex_set: Vec<usize> = (0..max_colors).collect();
        let mut class_list: Vec<Vec<usize>> = Vec::new();

        class_list.resize(max_colors, Vec::new());

        while !vertex_set.is_empty() {
            let mut min_num_edges_remaining = usize::MAX;

            num_color_classes += 1;

            for _ in 0..color_iterations {
                assign_color(
                    &vertex_set,
                    color_list_size,
                    &graph,
                    &mut min_num_edges_remaining,
                    &mut class_list,
                    num_color_classes,
                );
            }

            vertex_set.retain(|vertex| !class_list[num_color_classes - 1].contains(vertex));
        }
        // TODO ImprovePhase(V,E,i,{V1,...,Vi})
        if num_color_classes < num_colors {
            best_class_list = class_list;
            num_colors = num_color_classes;
        }
    }
    (num_colors, best_class_list)
}

fn assign_color(
    vertex_set: &[usize],
    color_list_size: usize,
    graph: &Graph,
    min_num_edges_remaining: &mut usize,
    class_list: &mut [Vec<usize>],
    num_color_classes: usize,
) {
    let mut admissible_uncolored: Vec<usize> = vertex_set.to_vec();
    let mut inadmissible_uncolored: Vec<usize> = Vec::new();
    let mut current_color_class: Vec<usize> = Vec::new();

    while !admissible_uncolored.is_empty() {
        let candidate_list = if inadmissible_uncolored.is_empty() {
            get_n_largest_degree(&color_list_size, graph, &admissible_uncolored, None)
        } else {
            get_n_largest_degree(
                &color_list_size,
                graph,
                &admissible_uncolored,
                Some(&inadmissible_uncolored),
            )
        };
        let vertex = candidate_list.choose(&mut rand::thread_rng());

        if let Some(vertex) = vertex {
            current_color_class.push(*vertex);
            let neighbors = graph.get_neighbors(*vertex);
            admissible_uncolored.retain(|node| node != vertex && !neighbors.contains(node));
            inadmissible_uncolored = [inadmissible_uncolored, neighbors].concat();
        } else {
            panic!("CSize must be at least 1")
        }
    }
    let mut remaining_vertices = vertex_set.to_vec();
    remaining_vertices.retain(|vertex| !current_color_class.contains(vertex));
    let remaining_edges = count_remaining_edges(graph, &remaining_vertices);

    if remaining_edges < *min_num_edges_remaining {
        class_list[num_color_classes - 1] = current_color_class;
        *min_num_edges_remaining = remaining_edges;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_get_n_largest_degree() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            // Use a subset to filter,
            // i.e., use an induced subgraph
            let set_subset = vec![10, 3, 4, 5];
            let largest_degrees = get_n_largest_degree(&3, &graph, &set_subset, None);

            assert_eq!(largest_degrees, vec![3, 5, 4]);

            // "Don't" use the subset to filter
            // Since the parameter isn't optional, this effect is emulated by setting the subset to
            // all vertices
            let set_entire_graph: Vec<usize> = (0..graph.num_vertices()).collect();
            let largest_degrees = get_n_largest_degree(&5, &graph, &set_entire_graph, None);

            assert_eq!(largest_degrees, vec![10, 0, 1, 2, 3]);

            // We don't care if the number of elements we're actually taking is smaller than the
            // number we requested, due to a limitation in the subset length
            let n_larger_than_subset = set_subset.len() + 1;
            let largest_degrees =
                get_n_largest_degree(&n_larger_than_subset, &graph, &set_subset, None);

            assert_eq!(largest_degrees.len(), set_subset.len());

            // We also don't care if we request too many elements overall
            // i.e., more elements than the number of vertices in the graph
            let too_many_elements = set_entire_graph.len() + 1;
            let largest_degrees =
                get_n_largest_degree(&too_many_elements, &graph, &set_entire_graph, None);

            assert_eq!(largest_degrees.len(), set_entire_graph.len());

            // TODO test parameter `list`
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_count_remaining_edges() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let list = vec![0, 1, 2];
            let num_edges = count_remaining_edges(&graph, &list);
            assert_eq!(num_edges, 2);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_grasp() {
        // Asserts GRASP provides
        // 1. A solution and,
        // 2. Said solution is viable
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel4.col") {
            let adjacency_matrix = graph.adjacency_matrix();
            let num_vertices = graph.num_vertices();
            let (_num_colors, class_colors) = grasp(graph, 10, 5, 5);

            // TODO compare num_colors with an instance whose chromatic number is known

            let mut coloring: Vec<usize> = vec![0; num_vertices];

            for (i, class) in class_colors.iter().enumerate() {
                for vertex in class {
                    assert_eq!(coloring[*vertex], 0); // Each vertex must get only one color;
                    coloring[*vertex] = i + 1;
                }
            }

            for (i, vertex) in coloring.iter().enumerate() {
                assert_ne!(*vertex, 0); // Each vertex must get a color

                (0..num_vertices).for_each(|j| {
                    if adjacency_matrix[i][j] {
                        assert_ne!(coloring[j], *vertex); // Neighbors can't share colors
                    }
                });
            }
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}
