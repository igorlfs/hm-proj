mod algorithms;
mod args;
mod graph;
mod input;

use algorithms::grasp::grasp_wrapper;
use algorithms::{genetic::genetic, grasp_pr::grasp_path_relinking};
use args::{Algorithm, Args};
use clap::Parser;
use std::process;
use std::time::Instant;

fn main() {
    let Args {
        algorithm,
        path,
        pr_solutions,
        grasp_iterations,
        color_iterations,
        color_list_size,
        generations,
        population_size,
        offspring_size,
        mutation_probaility,
        population_ratio,
    } = Args::parse();

    if let Ok(Some(graph)) = input::read_graph_from_file(path.as_str()) {
        let start = Instant::now();

        let (num_colors, coloring) = match algorithm {
            Algorithm::Genetic => genetic(
                &graph,
                generations.unwrap_or(10000),
                population_size.unwrap_or(100),
                offspring_size.unwrap_or(2),
                mutation_probaility.unwrap_or(0.01),
                population_ratio.unwrap_or(0.2),
            ),
            Algorithm::Grasp => grasp_wrapper(
                &graph,
                grasp_iterations.unwrap_or(10),
                color_iterations.unwrap_or(5),
                color_list_size.unwrap_or(5),
            ),
            Algorithm::GraspPR => grasp_path_relinking(&graph, pr_solutions.unwrap_or(5)),
        };

        let duration = start.elapsed().as_millis();

        println!("Number of colors used: {:?}", num_colors);
        println!("Color assignment: {:?}", coloring);
        println!("Duration: {:?}", duration);
    } else {
        eprintln!("Failed to open the specified instance: {path}");
        process::exit(1);
    }
}
