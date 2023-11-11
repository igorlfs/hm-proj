use ::clap::Parser;

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum Algorithm {
    Genetic,
    Grasp,
    GraspPR,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Heuristic approach used to solve the instance
    #[arg(short, long)]
    pub algorithm: Algorithm,

    /// Path to a Graph Coloring instance
    #[arg(short, long)]
    pub path: String,
}
