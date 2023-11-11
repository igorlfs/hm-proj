mod algorithms;
mod args;
mod graph;
mod input;

use std::process;

use args::{Algorithm, Args};
use clap::Parser;

use std::time::{Duration, Instant};

fn main() {
    // O uso aqui passa a ser ```cargo run -- -a grasp -p "instance-path"```
    // ```cargo run -- -h``` fala mais informações
    let Args { algorithm, path } = Args::parse();

    if let Ok(Some(graph)) = input::read_graph_from_file(path.as_str()) {
        let start: Instant = Instant::now();

        // A gente pode padronizar o retorno de todos os algoritmos e retornar, por exemplo, o número de cores usada e a cor de cada vértice e retornar pra esse match
        match algorithm {
            Algorithm::Genetic => panic!("Genetic not implemented yet"),
            Algorithm::Grasp => panic!("Grasp not implemented yet"),
            Algorithm::GraspPR => panic!("Grasp with PR not implemented yet"),
        };

        let duration: Duration = start.elapsed();

        // println!("Number of colors used: {:?}", colors_used);
        // println!("Color assignment: {:?}", color_assignment);
        // println!("Duration: {:?}", duration);
    } else {
        eprintln!("Failed to open the specified instance");
        process::exit(1);
    }
}
