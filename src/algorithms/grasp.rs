use super::Solution;
use crate::graph::adj_list::AdjList;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::collections::{BinaryHeap, HashSet};

/// Given a `graph`, gets (at most) `n` indexes of the higher degree vertices in the subgraph induced by
/// `subset`. If `list` is provided, don't use the induced subgraph.
/// Instead, from the vertices in `subset` count the *overall* degrees only within the `list`.
fn get_n_largest_degree(
    n: usize,
    graph: &AdjList,
    subset: &[usize],
    list: Option<&[usize]>,
) -> Vec<usize> {
    let list = if let Some(list) = list { list } else { subset };
    let vertex_set: Vec<usize> = (0..graph.num_vertices()).collect();
    let mut degrees: Vec<(usize, usize)> = vertex_set
        .iter()
        .map(|vertex| (*vertex, graph.get_degree_in_list(*vertex, list)))
        .collect();

    degrees.retain(|x| subset.contains(&x.0));

    degrees.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));

    degrees.iter().take(n).map(|(index, _)| *index).collect()
}

/// Counts the number of edges in subgraph induced by `graph` and `list`.
fn count_remaining_edges(graph: &AdjList, list: &[usize]) -> usize {
    let mut count = 0;

    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if graph.adj_list()[i].contains(&j) {
                count += 1;
            }
        }
    }

    count
}

/// Runs a single GRASP execution with the given parameters.
pub fn grasp_wrapper(
    graph: &AdjList,
    grasp_iterations: i32,
    color_iterations: i32,
    color_list_size: usize,
) -> Solution {
    let mut solutions = grasp(
        graph,
        grasp_iterations,
        color_iterations,
        color_list_size,
        1,
    );
    let (num_colors, coloring) = solutions.pop().unwrap();
    (num_colors, coloring)
}

pub fn grasp(
    graph: &AdjList,
    grasp_iterations: i32,
    color_iterations: i32,
    color_list_size: usize,
    num_solutions: usize,
) -> BinaryHeap<Solution> {
    let max_colors = graph.num_vertices();
    let mut solutions = BinaryHeap::with_capacity(num_solutions);

    let all: Vec<Solution> = (0..grasp_iterations)
        .into_par_iter()
        .map(|_| {
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
                        graph,
                        &mut min_num_edges_remaining,
                        &mut class_list,
                        num_color_classes,
                    );
                }

                vertex_set.retain(|vertex| !class_list[num_color_classes - 1].contains(vertex));
            }

            improve_phase(graph, &mut num_color_classes, &mut class_list);

            let coloring = get_coloring_from_class_list(max_colors, &class_list);
            (num_color_classes, coloring)
        })
        .collect();

    for solution in all {
        if solutions.len() < num_solutions {
            solutions.push(solution);
        } else if solution.0 < solutions.peek().unwrap().0 {
            solutions.pop();
            solutions.push(solution);
        }
    }

    solutions
}

/// Tries to assign a color class `num_color_classes` to `class_list`
/// following the greedy heuristic.
///
/// The greedy heuristic chooses an available vertex: a vertex such that none of its neighbors have
/// been colored. It tries to cover the remaining graph entirely (or until no candidates remain).
///
/// Refer to the article for more information about the heuristic.
fn assign_color(
    vertex_set: &[usize],
    color_list_size: usize,
    graph: &AdjList,
    min_num_edges_remaining: &mut usize,
    class_list: &mut [Vec<usize>],
    num_color_classes: usize,
) {
    let mut admissible_uncolored: Vec<usize> = vertex_set.to_vec();
    let mut inadmissible_uncolored: Vec<usize> = Vec::new();
    let mut current_color_class: Vec<usize> = Vec::new();

    while !admissible_uncolored.is_empty() {
        let candidate_list = if inadmissible_uncolored.is_empty() {
            get_n_largest_degree(color_list_size, graph, &admissible_uncolored, None)
        } else {
            get_n_largest_degree(
                color_list_size,
                graph,
                &admissible_uncolored,
                Some(&inadmissible_uncolored),
            )
        };
        let vertex = candidate_list.choose(&mut rand::thread_rng());

        if let Some(vertex) = vertex {
            current_color_class.push(*vertex);
            let neighbors = graph.adj_list()[*vertex].clone();
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

/// Tries to improve the coloring from `class_list` by
///
/// 1. Merging the smallest class colors
/// 2. Applying a local search for the resulting class list
///
/// The process repeats until a forbidden coloring is found
fn improve_phase(graph: &AdjList, num_classes: &mut usize, class_list: &mut Vec<Vec<usize>>) {
    let mut num_forbidden = 0;

    while num_forbidden == 0 {
        let mut lenghts: Vec<(usize, usize)> = class_list
            .iter()
            .enumerate()
            .map(|(index, class)| (index, class.len()))
            .take(*num_classes)
            .collect();

        lenghts.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));

        let smallest_lengths: Vec<usize> = lenghts
            .iter()
            .rev()
            .take(2)
            .map(|(index, _)| *index)
            .collect();

        let mut combined_class: Vec<usize> = vec![];

        for index in smallest_lengths.iter() {
            combined_class.append(&mut class_list[*index].clone());
        }

        let mut new_classes: Vec<Vec<usize>> = vec![];

        new_classes.push(combined_class);

        for (index, class) in class_list.iter().enumerate() {
            if index == smallest_lengths[0] || index == smallest_lengths[1] || class.is_empty() {
                continue;
            }
            new_classes.push(class.clone());
        }

        num_forbidden = local_search(graph, &mut new_classes);

        if num_forbidden == 0 {
            *num_classes = new_classes.len();
            *class_list = new_classes;
        }
    }

    let num_vertices = graph.num_vertices();
    class_list.resize(num_vertices, Vec::new());
}

/// Counts the number of forbidden edges in `graph` according to `class_list`.
///
/// Saves the corresponding vertices in a set.
fn get_forbidden_vertices(graph: &AdjList, class_list: &[Vec<usize>]) -> (usize, HashSet<usize>) {
    let num_vertices = graph.num_vertices();
    let adj_list = graph.adj_list();
    let coloring = get_coloring_from_class_list(num_vertices, class_list);
    let mut count = 0;
    let mut forbidden = HashSet::new();
    for (i, v) in adj_list.iter().enumerate() {
        for j in v.iter() {
            if coloring[i] == coloring[*j] {
                count += 1;
                forbidden.insert(i);
                forbidden.insert(*j);
            }
        }
    }
    (count / 2, forbidden)
}

/// Applies a local search for `class_list` according to `graph`.
///
/// The local search works by selecting an illegal vertex and trying every possible color swap for
/// said vertex to reduce the number of forbidden vertices in the graph.
/// If we can improve, we update the `class_list`.
///
/// Repeats the process while they are forbidden vertices
/// or the number of iterations that haven't improved `class_list` reaches a threshold.
///
/// Returns the number of edges that are still forbidden.
fn local_search(graph: &AdjList, class_list: &mut Vec<Vec<usize>>) -> usize {
    let (mut forbidden_count, mut forbidden_set) = get_forbidden_vertices(graph, class_list);
    let no_improvement_ceil = 2 * forbidden_count;
    let mut forbidden_vertices: Vec<usize> = forbidden_set.into_iter().collect();
    // We use this variable to control how many iterations we can go by without improvement
    let mut no_improvement = 0;

    while forbidden_count > 0 && no_improvement < no_improvement_ceil {
        // Randomly choose an illegal vertex (i.e., one that is colored with the same color as an adjacent vertex).

        // Since forbidden_count > 0 we can unwrap
        let vertex = forbidden_vertices.choose(&mut rand::thread_rng()).unwrap();
        let mut coloring = get_coloring_from_class_list(graph.num_vertices(), class_list);
        let mut best_count = count_forbidden_per_vertex(graph, &coloring, *vertex);
        let original_count = best_count;
        let mut best_color = coloring[*vertex];
        let original_color = best_color;

        // Make all possible attempts to switch v to a different color to improve the current value of f(s).

        // Colors are 1-indexed
        for i in 1..class_list.len() + 1 {
            coloring[*vertex] = i;
            let new_count = count_forbidden_per_vertex(graph, &coloring, *vertex);
            if new_count < best_count {
                best_count = new_count;
                best_color = i;
            }
        }

        if best_count < original_count {
            no_improvement = 0;

            // Updating class_list
            let original_index_in_class_list = class_list[original_color - 1]
                .iter()
                .position(|x| *x == *vertex)
                .unwrap();
            class_list[original_color - 1].remove(original_index_in_class_list);
            class_list[best_color - 1].push(*vertex);

            (forbidden_count, forbidden_set) = get_forbidden_vertices(graph, class_list);
            forbidden_vertices = forbidden_set.into_iter().collect();
        } else {
            no_improvement += 1;
        }
    }

    forbidden_count
}

/// Turn a "Class List" into a traditional coloring. A class list assigns each index in a vector to
/// a vector of vertices, which represent a given color.
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
fn count_forbidden_per_vertex(graph: &AdjList, coloring: &[usize], vertex: usize) -> usize {
    graph.adj_list()[vertex]
        .iter()
        .filter(|x| coloring[**x] == coloring[vertex])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{algorithms::is_coloring_valid, input};

    #[test]
    fn test_get_n_largest_degree() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            // Use a subset to filter,
            // i.e., use an induced subgraph
            let set_subset = vec![10, 3, 4, 5];
            let largest_degrees = get_n_largest_degree(3, &graph, &set_subset, None);

            assert_eq!(largest_degrees, vec![3, 5, 4]);

            // "Don't" use the subset to filter
            // Since the parameter isn't optional, this effect is emulated by setting the subset to
            // all vertices
            let set_entire_graph: Vec<usize> = (0..graph.num_vertices()).collect();
            let largest_degrees = get_n_largest_degree(5, &graph, &set_entire_graph, None);

            assert_eq!(largest_degrees, vec![10, 0, 1, 2, 3]);

            // We don't care if the number of elements we're actually taking is smaller than the
            // number we requested, due to a limitation in the subset length
            let n_larger_than_subset = set_subset.len() + 1;
            let largest_degrees =
                get_n_largest_degree(n_larger_than_subset, &graph, &set_subset, None);

            assert_eq!(largest_degrees.len(), set_subset.len());

            // We also don't care if we request too many elements overall
            // i.e., more elements than the number of vertices in the graph
            let too_many_elements = set_entire_graph.len() + 1;
            let largest_degrees =
                get_n_largest_degree(too_many_elements, &graph, &set_entire_graph, None);

            assert_eq!(largest_degrees.len(), set_entire_graph.len());
        } else {
            panic!("The file containing the test graph is missing")
        }

        let mut graph = AdjList::complete(4);

        // Given the subgraph induce by &[0,1,3] (K3) and the list &[1]
        // The vertices with largest_degree ought to be [0,3] since they share an edge with [1]
        let largest_degrees = get_n_largest_degree(2, &graph, &[0, 1, 3], Some(&[1]));

        assert_eq!(largest_degrees, vec![0, 3]);

        // There are no edges between the vertex 2 in the subgraph induced by [0,1,3]
        // But we don't count the edges within the induced subgraph: instead, we use the subset
        // parameter as a `filter`
        //
        // As in, to only consider these vertices, but the overall edges.
        //
        // Hence, when we remove an edge outside the induced subgraph,
        // the return value should be updated accordingly
        graph.sub_edge(0, 2);
        let largest_degrees = get_n_largest_degree(2, &graph, &[0, 1, 3], Some(&[2]));

        assert_eq!(largest_degrees, vec![1, 3]);
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
    fn test_grasp_wrapper() {
        // Asserts GRASP provides a solution
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel5.col") {
            let (_, coloring) = grasp_wrapper(&graph, 10, 5, 5);

            assert!(is_coloring_valid(&graph, &coloring));
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_improve_phase() {
        let mut graph = AdjList::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);

        let mut num_classes = 4;
        let mut class_list = vec![vec![1], vec![2], vec![4, 5], vec![0, 3]];

        improve_phase(&graph, &mut num_classes, &mut class_list);

        assert!(num_classes <= 4);

        // Since the algorithm is randomized, we can't compare to an expected result
        // But it should still be valid nonetheless
        let coloring = get_coloring_from_class_list(6, &class_list);

        assert!(is_coloring_valid(&graph, &coloring));
    }

    #[test]
    fn test_get_forbidden_vertices() {
        // The complete graph
        let graph = AdjList::complete(5);
        let color_classes = vec![vec![0], vec![1], vec![2, 3, 4]];

        let (count, forbidden) = get_forbidden_vertices(&graph, &color_classes);

        assert_eq!(forbidden, HashSet::from([2, 3, 4]));
        assert_eq!(count, 3)
    }

    #[test]
    fn test_local_search() {
        // Basically a linked list colored as 1---2---2---3
        let mut graph = AdjList::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        let mut color_classes = vec![vec![0], vec![1, 2], vec![3]];

        let num_forbidden = local_search(&graph, &mut color_classes);

        assert_eq!(num_forbidden, 0);
    }

    #[test]
    fn test_get_coloring_from_class_list() {
        let class_list = vec![vec![0], vec![1, 2], vec![3]];
        let coloring = get_coloring_from_class_list(4, &class_list);

        assert_eq!(coloring, [1, 2, 2, 3])
    }

    #[test]
    fn test_count_forbidden_per_vertex() {
        let graph = AdjList::complete(5);
        let coloring = [1, 1, 1, 1, 1];

        let num_forbidden = count_forbidden_per_vertex(&graph, &coloring, 1);

        assert_eq!(num_forbidden, 4);
    }
}
