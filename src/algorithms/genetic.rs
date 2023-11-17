use rand::Rng;

use crate::graph::Graph;

/// Counts the number of colors used in a GCP solution.
fn count_colors(solution: &Vec<usize>) -> usize {
    let mut solution = solution.clone();

    solution.sort();
    solution.dedup();

    solution.len()
}

/// Checks if the current color assignment of a node and his neighborhood is valid.
fn valid_color_assignment(graph: &Graph, solution: &Vec<usize>, node: usize) -> bool {
    let mut is_valid = true;
    let neighbors = graph.get_neighbors(node);

    for i in neighbors {
        if solution[node] == solution[i] {
            is_valid = false;
        }
    }

    is_valid
}

// A coloring upper bound based on the largest neighborhood
// Tighter upper bounds helps a lot in the randomized color
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
/// where the `ith position` receives a number between `1` and the `upper bound for coloring`.
/// This number represents the `color of the ith vertex` in the current solution.
fn generate_individual(graph: &Graph, upper_bound: usize) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut individual = vec![0; n];
    let mut is_invalid = true;

    for i in 0..n {
        while is_invalid {
            individual[i] = rand::thread_rng().gen_range(1..=upper_bound);
            is_invalid = !valid_color_assignment(&graph, &individual, i);
        }

        is_invalid = true;
    }

    individual
}

/// Traverses the solution vector, changing the color of each vertex to a random color
/// with a probability given by the `mutation_probability` parameter.
fn mutate(
    graph: &Graph,
    individual: &mut Vec<usize>,
    upper_bound: usize,
    mutation_probability: &f64,
) {
    let n = graph.num_vertices();
    let mut rng = rand::thread_rng();
    let mut is_invalid = true;

    for i in 1..n {
        let rand = rng.gen_range(0.0..1.0);

        if rand <= *mutation_probability {
            while is_invalid {
                individual[i] = rng.gen_range(1..=upper_bound);
                is_invalid = !valid_color_assignment(&graph, &individual, i);
            }

            is_invalid = true;
        }
    }
}

/// Selects 20% of the fittest individuals in the population and, from them, randomly
/// selects two individuals who will be the parents of an offspring.
fn select(
    population: &Vec<(usize, Vec<usize>)>,
    population_size: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut rng = rand::thread_rng();
    let limit = (population_size as f64 * 0.2).floor() as usize;
    let mut p1 = rng.gen_range(0..limit);
    let mut p2 = rng.gen_range(0..limit);

    while p1 == p2 {
        p1 = rng.gen_range(0..limit);
        p2 = rng.gen_range(0..limit);
    }

    (population[p1].1.clone(), population[p2].1.clone())
}

/// Given two parents `p1` and `p2`, returns an offspring generated from the recombination
/// of `p1` and `p2`.
///
/// The crossover strategy implemented was the one-point crossover (i.e., chooses
/// a random position of the vector and makes the offspring equal to the first parent up to
/// that position and equal to the second parent from that position onwards.)
fn crossover(graph: &Graph, p1: Vec<usize>, p2: Vec<usize>) -> Vec<usize> {
    let n = graph.num_vertices();
    let mut rng = rand::thread_rng();
    let mut offspring = vec![0; n];
    let pos = rng.gen_range(0..n);

    for i in 0..=pos {
        offspring[i] = p1[i];
    }

    for i in (pos + 1)..n {
        offspring[i] = p2[i];
    }

    for i in 0..n {
        let mut start_color = 1;

        while !valid_color_assignment(&graph, &offspring, i) {
            offspring[i] = start_color;
            start_color += 1;
        }
    }

    offspring
}

/// Truncates the population to keep the original size after the crossover operator.
///
/// This function is called after a sort, so the elements after the truncate
fn replace(population: &mut Vec<(usize, Vec<usize>)>, population_size: usize) {
    population.truncate(population_size);
}

pub fn genetic(
    graph: Graph,
    generations: usize,
    population_size: usize,
    offsprings_per_generation: usize,
    mutation_probability: f64,
) -> (usize, Vec<usize>) {
    let mut best = graph.num_vertices();
    let mut colors = (1..=graph.num_vertices()).collect();
    let mut population = Vec::<(usize, Vec<usize>)>::new();
    let upper_bound = coloring_upper_bound(&graph);

    for _ in 0..population_size {
        let individual = generate_individual(&graph, upper_bound);
        population.push((count_colors(&individual), individual));
    }

    population.sort();

    for _ in 0..generations {
        for _ in 0..offsprings_per_generation {
            let (p1, p2) = select(&population, population_size);

            let mut offspring = crossover(&graph, p1, p2);

            mutate(&graph, &mut offspring, upper_bound, &mutation_probability);

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
    fn test_genetic() {
        if let Ok(Some(graph)) = input::read_graph_from_file("data/sgb/anna.col") {
            // grafo, número de gerações, tamanho da população, fração da população que se reproduz, fração da população
            let (best, colors) = genetic(graph, 10000, 100, 2, 0.01);

            println!("{} - {:?}", best, colors);
        } else {
            panic!("The file containing the test graph is missing")
        }
    }
}
