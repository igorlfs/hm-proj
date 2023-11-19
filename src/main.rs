mod algorithms;
mod args;
mod graph;
mod input;

use algorithms::grasp::grasp_wrapper;
use algorithms::{genetic::genetic, grasp_pr::grasp_path_relinking};
use args::{Algorithm, Args};
use clap::Parser;
use std::process;
use std::time::{Duration, Instant};

fn main() {
    let Args { algorithm, path } = Args::parse();

    if let Ok(Some(graph)) = input::read_graph_from_file(path.as_str()) {
        let start: Instant = Instant::now();

        let (num_colors, coloring) = match algorithm {
            Algorithm::Genetic => genetic(&graph, 10000, 100, 2, 0.01, 0.2),
            Algorithm::Grasp => grasp_wrapper(&graph, 10, 5, 5),
            Algorithm::GraspPR => grasp_path_relinking(&graph, 5),
        };

        let duration: Duration = start.elapsed();

        println!("Number of colors used: {:?}", num_colors);
        println!("Color assignment: {:?}", coloring);
        println!("Duration: {:?}", duration);
    } else {
        eprintln!("Failed to open the specified instance: {path}");
        process::exit(1);
    }
}
