use std::env;

mod algorithms;
mod graph;
mod input;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // A ideia poderia ser: você especifica um arquivo ou tamanho para gerar aleatório e rodar
        eprintln!("Give a path to a file");
        return;
    }

    if let Ok(Some(graph)) = input::read_graph_from_file(args[1].as_str()) {
        // TODO
    } else {
        eprintln!("Input file doesn't follow the specification");
    }
}
