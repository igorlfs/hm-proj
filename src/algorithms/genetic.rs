use crate::graph::Graph;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

/// Counts the number of colors used in a GCP solution.
fn count_colors(solution: &[usize]) -> usize {
    let colors: HashSet<&usize> = solution.iter().collect();

    colors.len()
}

/// Checks if the current color assignment of a node and his neighborhood is valid.
fn valid_color_assignment(graph: &Graph, solution: &[usize], node: usize) -> bool {
    !graph
        .get_neighbors(node)
        .iter()
        .any(|x| solution[node] == solution[*x])
}

// A coloring upper bound based on the largest neighborhood
// Tighter upper bounds help during the randomized color
// assignment (individual generation and mutation)
/// Determine an upper bound for the number of colors in a graph.
///
/// This upper bound is calculated based on the `Brook's theorem` (i.e., the chromatic number is less or equal than the maximum vertex degree of the graph plus one).
fn coloring_upper_bound(graph: &Graph) -> usize {
    let mut colors = 0;

    for i in 0..graph.num_vertices() {
        let neighborhood_size = graph.get_neighbors(i).len();

        if neighborhood_size + 1 > colors {
            colors = neighborhood_size + 1;
        }
    }

    colors
}

/// Generates a randomized solution for the GCP problem.
///
/// A solution consists of a vector of size `n`, with `n` being the number of vertices of the graph,
/// where the `ith position` receives a number between `1` and the `upper bound` for coloring.
/// This number represents the `color of the ith vertex` in the current solution.
fn generate_individual(graph: &Graph, upper_bound: usize) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut individual = vec![1; n];

    for i in 0..n {
        individual[i] = rand::thread_rng().gen_range(1..=upper_bound);

        while !valid_color_assignment(graph, &individual, i) {
            individual[i] = rand::thread_rng().gen_range(1..=upper_bound);
        }
    }

    individual
}

/// Traverses the solution vector, changing the color of each vertex to a random color
/// with a probability given by the `mutation_probability` parameter.
fn mutate(graph: &Graph, individual: &mut [usize], upper_bound: usize, mutation_probability: f64) {
    let n = graph.num_vertices();
    let mut rng = rand::thread_rng();

    for i in 0..n {
        let rand = rng.gen_range(0.0..1.0);

        if rand <= mutation_probability {
            individual[i] = rng.gen_range(1..=upper_bound);

            while !valid_color_assignment(graph, individual, i) {
                individual[i] = rng.gen_range(1..=upper_bound);
            }
        }
    }
}

/// Selects a fraction (`selected_population_ratio` parameter) of the fittest individuals
/// in the population and, from them, randomly selects two individuals who will be the parents
/// of an offspring.
fn select(
    population: &Vec<(usize, Vec<usize>)>,
    population_size: usize,
    selected_population_ratio: f64,
) -> (Vec<usize>, Vec<usize>) {
    let limit = (population_size as f64 * selected_population_ratio).floor() as usize;

    let (_, mut colors): (Vec<usize>, Vec<Vec<usize>>) =
        population.to_owned().clone().into_iter().unzip();

    colors.truncate(limit + 1);

    let p: Vec<Vec<usize>> = colors
        .choose_multiple(&mut rand::thread_rng(), 2)
        .cloned()
        .collect();

    (p[0].clone(), p[1].clone())
}

/// Given two parents `p1` and `p2`, returns an offspring generated from the recombination
/// of `p1` and `p2`.
///
/// The crossover strategy implemented is the one-point crossover (i.e., chooses
/// a random position of the vector and makes the offspring equal to the first parent up to
/// that position and equal to the second parent from that position onwards.)
fn crossover(graph: &Graph, p1: Vec<usize>, p2: Vec<usize>) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut rng = rand::thread_rng();
    let mut offspring = vec![1; n];
    let pos = rng.gen_range(0..n);

    offspring[..(pos + 1)].copy_from_slice(&p1[..(pos + 1)]);

    offspring[(pos + 1)..].copy_from_slice(&p2[(pos + 1)..]);

    for i in 0..n {
        let mut start_color = 1;

        while !valid_color_assignment(graph, &offspring, i) {
            offspring[i] = start_color;
            start_color += 1;
        }
    }

    offspring
}

/// Truncates the population to keep the original size after the crossover operator.
///
/// This function is called after a `sort`, so the remaining elements after the `truncate` are the
/// fittest individuals in a population with the original size.
fn replace(population: &mut Vec<(usize, Vec<usize>)>, population_size: usize) {
    population.truncate(population_size);
}

pub fn genetic(
    graph: &Graph,
    generations: usize,
    population_size: usize,
    offsprings_per_generation: usize,
    mutation_probability: f64,
    selected_population_ratio: f64,
) -> (usize, Vec<usize>) {
    let mut best = graph.num_vertices();
    let mut colors = (1..=best).collect();
    let mut population = Vec::<(usize, Vec<usize>)>::new();
    let upper_bound = coloring_upper_bound(graph);

    for _ in 0..population_size {
        let individual = generate_individual(graph, upper_bound);
        population.push((count_colors(&individual), individual));
    }

    population.sort();

    for _ in 0..generations {
        for _ in 0..offsprings_per_generation {
            let (p1, p2) = select(&population, population_size, selected_population_ratio);

            let mut offspring = crossover(graph, p1, p2);

            mutate(graph, &mut offspring, upper_bound, mutation_probability);

            population.push((count_colors(&offspring), offspring));
        }

        population.sort();

        replace(&mut population, population_size);

        let current_best = population[0].clone();

        if current_best.0 < best {
            best = current_best.0;
            colors = current_best.1.clone()
        }
    }

    (best, colors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input;

    #[test]
    fn test_count_colors() {
        assert_eq!(count_colors(&vec![1]), 1);
        assert_eq!(count_colors(&vec![3, 1, 6, 6, 1, 5]), 4);
        assert_eq!(count_colors(&vec![2, 1, 3, 1, 1, 4, 5, 10, 4, 3, 3]), 6);
        assert_eq!(count_colors(&vec![]), 0);
    }

    #[test]
    fn test_valid_color_assignment() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert!(valid_color_assignment(&graph, &vec![1, 2, 3, 1], 2));
        assert!(!valid_color_assignment(&graph, &vec![1, 2, 2, 1], 2));
    }

    #[test]
    fn test_coloring_upper_bound() {
        let mut g1 = Graph::new(4);
        g1.add_edge(0, 1);
        g1.add_edge(0, 2);
        g1.add_edge(1, 2);
        g1.add_edge(2, 3);

        assert_eq!(coloring_upper_bound(&g1), 4);

        if let Ok(Some(g2)) = input::read_graph_from_file("data/myc/myciel3.col") {
            assert_eq!(coloring_upper_bound(&g2), 6);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_generate_individual() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let upper_bound = coloring_upper_bound(&graph);

            assert_eq!(upper_bound, 6);

            let individual = generate_individual(&graph, upper_bound);

            for i in 0..graph.num_vertices() {
                assert!(valid_color_assignment(&graph, &individual, i))
            }
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_mutate() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let upper_bound = coloring_upper_bound(&graph);

            assert_eq!(upper_bound, 6);

            let mut individual = generate_individual(&graph, upper_bound);

            for i in 0..graph.num_vertices() {
                assert!(valid_color_assignment(&graph, &individual, i))
            }

            // A little higher mutation probability just to ensure that some vertices actually change
            mutate(&graph, &mut individual, upper_bound, 0.2);

            for i in 0..graph.num_vertices() {
                assert!(valid_color_assignment(&graph, &individual, i))
            }
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_select() {
        let mut population = vec![
            (3, vec![1, 2, 1, 3, 1]),
            (2, vec![2, 1, 2, 2, 1]),
            (4, vec![1, 2, 3, 4]),
            (3, vec![3, 2, 2, 1, 3, 1]),
            (3, vec![1, 2, 3, 1, 2, 3, 1, 2, 3]),
            (5, vec![1, 2, 3, 4, 5]),
        ];

        population.sort();

        let (p1, p2) = select(&population, population.len(), 0.2);

        assert_ne!(p1, p2);

        for i in 0..population.len() {
            if population[i].1 == p1 || population[i].1 == p2 {
                assert!(i <= (population.len() as f64 * 0.2).floor() as usize);
            }
        }
    }

    #[test]
    fn test_crossover() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let mut population = Vec::new();
            let upper_bound = coloring_upper_bound(&graph);

            for _ in 0..6 {
                let individual = generate_individual(&graph, upper_bound);
                population.push((count_colors(&individual), individual));
            }

            population.sort();

            let (p1, p2) = select(&population, population.len(), 0.2);

            let offspring = crossover(&graph, p1, p2);

            for i in 0..graph.num_vertices() {
                assert!(valid_color_assignment(&graph, &offspring, i));
            }
        } else {
            panic!("The file containing the test graph is missing")
        }
    }

    #[test]
    fn test_replace() {
        let population = vec![
            (3, vec![1, 2, 1, 3, 1]),
            (2, vec![2, 1, 2, 2, 1]),
            (4, vec![1, 2, 3, 4]),
            (3, vec![3, 2, 2, 1, 3, 1]),
            (3, vec![1, 2, 3, 1, 2, 3, 1, 2, 3]),
            (5, vec![1, 2, 3, 4, 5]),
        ];

        let mut pop1 = population.clone();
        let mut pop2 = population.clone();

        replace(&mut pop1, 3);

        assert_eq!(
            pop1,
            vec![
                (3, vec![1, 2, 1, 3, 1]),
                (2, vec![2, 1, 2, 2, 1]),
                (4, vec![1, 2, 3, 4])
            ]
        );

        replace(&mut pop2, 1);

        assert_eq!(pop2, vec![(3, vec![1, 2, 1, 3, 1]),]);
    }

    #[test]
    fn test_genetic() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/myc/myciel3.col") {
            let (best, colors) = genetic(&graph, 10000, 100, 2, 0.01, 0.2);

            assert!(best <= coloring_upper_bound(&graph));

            for i in 0..graph.num_vertices() {
                assert!(valid_color_assignment(&graph, &colors, i))
            }
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}
