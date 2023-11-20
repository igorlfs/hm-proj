#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Algorithm {
    Genetic,
    Grasp,
    GraspPR,
}

#[derive(Debug, clap::Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to a Graph Coloring instance
    #[arg(short, long)]
    pub path: String,

    /// Heuristic approach used to solve the instance
    #[arg(short, long)]
    pub algorithm: Algorithm,

    /// Number of GRASP solutions to use in PR for GRASP+PR.
    /// Does NOT affect the actual GRASP parameters.
    /// Defaults to 5 if not provided.
    #[arg(long)]
    pub pr_solutions: Option<usize>,

    #[arg(long)]
    /// Total GRASP iterations.
    /// Defaults to 10 if not provided.
    pub grasp_iterations: Option<i32>,

    #[arg(long)]
    /// Iterations per color for GRASP.
    /// Defaults to 5 if not provided.
    pub color_iterations: Option<i32>,

    #[arg(long)]
    /// Number of vertices taken into account for color assignment in GRASP.
    /// Defaults to 5 if not provided.
    pub color_list_size: Option<usize>,

    #[arg(long)]
    /// Number of generations for the Genetic Algorithm.
    /// Defaults to 10000 if not provided.
    pub generations: Option<usize>,

    #[arg(long)]
    /// Population size for the Genetic Algorithm.
    /// Defaults to 100 if not provided.
    pub population_size: Option<usize>,

    #[arg(long)]
    /// Offspring per generation for the Genetic Algorithm.
    /// Defaults to 2 if not provided.
    pub offspring_size: Option<usize>,

    #[arg(long)]
    /// Mutation probability for the Genetic Algorithm
    /// Defaults to 0.01 if not provided.
    pub mutation_probaility: Option<f64>,

    #[arg(long)]
    /// Population selection ratio for the Genetic Algorithm
    /// Defaults to 0.2 if not provided.
    pub population_ratio: Option<f64>,
}
